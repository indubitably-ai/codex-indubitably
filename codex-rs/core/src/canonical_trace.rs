use std::collections::HashMap;
use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use codex_protocol::ThreadId;
use codex_protocol::items::AgentMessageContent;
use codex_protocol::items::TurnItem;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

use crate::event_mapping::parse_turn_item;
use crate::protocol::EventMsg;
use crate::protocol::FileChange;
use crate::protocol::RolloutItem;
use crate::protocol::RolloutLine;
use crate::protocol::SessionSource;
use crate::protocol::SkillInvocationEvent;
use crate::protocol::TokenUsage;
use crate::protocol::TurnAbortReason;
use crate::protocol::TurnContextItem;
use crate::protocol::TurnSkillContextEvent;
use crate::protocol::TurnSkillDescriptor;

const CANONICAL_TRACE_PRODUCER: &str = "codex_app_server";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalEventCounts {
    pub total: usize,
    pub prompts: usize,
    pub responses: usize,
    pub thinking: usize,
    pub tool_calls: usize,
    pub skills: usize,
    pub file_changes: usize,
    pub plans: usize,
    pub system: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalTraceMetadata {
    pub session_id: Option<ThreadId>,
    pub title: Option<String>,
    pub agent_name: Option<String>,
    pub model_alias: Option<String>,
    pub model_provider: Option<String>,
    pub last_message_at_ms: Option<i64>,
    pub selected_skills: Option<Vec<TurnSkillDescriptor>>,
    pub effective_skills: Option<Vec<TurnSkillDescriptor>>,
    pub event_counts: CanonicalEventCounts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalTraceEvent {
    pub event_id: String,
    pub ts_ms: i64,
    pub kind: String,
    pub payload: Value,
    pub provenance: Option<Value>,
    pub run_id: Option<String>,
    pub job_id: Option<String>,
    pub parent_job_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CanonicalTrace {
    pub metadata: CanonicalTraceMetadata,
    pub events: Vec<CanonicalTraceEvent>,
}

#[derive(Debug, Default)]
struct TraceIndex {
    session_id: Option<ThreadId>,
    origin: Option<String>,
    agent_name: Option<String>,
    latest_model_alias: Option<String>,
    model_provider: Option<String>,
    latest_skill_context: Option<TurnSkillContextEvent>,
    skill_context_by_turn: HashMap<String, TurnSkillContextEvent>,
    model_alias_by_turn: HashMap<String, String>,
    trace_id_by_turn: HashMap<String, String>,
    usage_by_turn: HashMap<String, TokenUsage>,
}

pub fn build_canonical_trace(lines: &[RolloutLine]) -> CanonicalTrace {
    let mut index = TraceIndex::default();
    let mut current_turn_id: Option<String> = None;

    for line in lines {
        match &line.item {
            RolloutItem::SessionMeta(meta_line) => {
                if index.session_id.is_none() {
                    index.session_id = Some(meta_line.meta.id);
                }
                if index.origin.is_none() {
                    index.origin = Some(origin_from_session_source(&meta_line.meta.source));
                }
                if index.agent_name.is_none() {
                    index.agent_name = meta_line
                        .meta
                        .agent_nickname
                        .clone()
                        .or_else(|| meta_line.meta.source.get_nickname());
                }
                if index.model_provider.is_none() {
                    index.model_provider = meta_line.meta.model_provider.clone();
                }
            }
            RolloutItem::TurnContext(turn_context) => {
                current_turn_id = turn_context.turn_id.clone();
                record_turn_context(&mut index, turn_context);
            }
            RolloutItem::EventMsg(EventMsg::SessionConfigured(event)) => {
                if index.session_id.is_none() {
                    index.session_id = Some(event.session_id);
                }
                index.latest_model_alias = Some(event.model.clone());
                index.model_provider = Some(event.model_provider_id.clone());
            }
            RolloutItem::EventMsg(EventMsg::TurnStarted(event)) => {
                current_turn_id = Some(event.turn_id.clone());
            }
            RolloutItem::EventMsg(EventMsg::TurnSkillContext(event)) => {
                index.latest_skill_context = Some(event.clone());
                index
                    .skill_context_by_turn
                    .insert(event.turn_id.clone(), event.clone());
            }
            RolloutItem::EventMsg(EventMsg::TokenCount(event)) => {
                if let Some(turn_id) = current_turn_id.as_ref()
                    && let Some(info) = event.info.as_ref()
                {
                    index
                        .usage_by_turn
                        .insert(turn_id.clone(), info.last_token_usage.clone());
                }
            }
            RolloutItem::ResponseItem(_) | RolloutItem::Compacted(_) | RolloutItem::EventMsg(_) => {
            }
        }
    }

    let mut trace = CanonicalTrace {
        metadata: CanonicalTraceMetadata {
            session_id: index.session_id,
            title: None,
            agent_name: Some(
                index
                    .agent_name
                    .clone()
                    .unwrap_or_else(|| "Codex".to_string()),
            ),
            model_alias: index.latest_model_alias.clone(),
            model_provider: index.model_provider.clone(),
            last_message_at_ms: None,
            selected_skills: index
                .latest_skill_context
                .as_ref()
                .map(|context| context.selected_skills.clone()),
            effective_skills: index
                .latest_skill_context
                .as_ref()
                .map(|context| context.effective_skills.clone()),
            event_counts: CanonicalEventCounts::default(),
        },
        events: Vec::new(),
    };

    let mut current_turn_id: Option<String> = None;
    for line in lines {
        let ts_ms = timestamp_to_ms(&line.timestamp);
        match &line.item {
            RolloutItem::TurnContext(turn_context) => {
                current_turn_id = turn_context.turn_id.clone();
            }
            RolloutItem::EventMsg(EventMsg::TurnStarted(event)) => {
                current_turn_id = Some(event.turn_id.clone());
            }
            RolloutItem::ResponseItem(item) => {
                if let Some(turn_item) = parse_turn_item(item) {
                    match turn_item {
                        TurnItem::UserMessage(user_message) => {
                            let turn_id = current_turn_id.clone();
                            let text = user_message.message();
                            let content_blocks = user_message.content;
                            let payload = json!({
                                "text": text,
                                "content_blocks": content_blocks,
                            });
                            trace.events.push(CanonicalTraceEvent {
                                event_id: format!("canonical-{}", trace.events.len() + 1),
                                ts_ms,
                                kind: "user_message".to_string(),
                                payload,
                                provenance: Some(build_provenance(
                                    &index,
                                    turn_id.as_deref(),
                                    None,
                                    None,
                                    None,
                                    None,
                                )),
                                run_id: None,
                                job_id: None,
                                parent_job_id: None,
                            });
                            increment_event_counts(
                                &mut trace.metadata.event_counts,
                                "user_message",
                            );
                            trace.metadata.last_message_at_ms = Some(ts_ms);
                            if trace.metadata.title.is_none() {
                                let trimmed = text.trim();
                                if !trimmed.is_empty() {
                                    trace.metadata.title = Some(trimmed.to_string());
                                }
                            }
                        }
                        TurnItem::AgentMessage(agent_message) => {
                            let turn_id = current_turn_id.clone();
                            let phase = agent_message.phase.as_ref().and_then(message_phase_string);
                            let text = join_agent_message_text(&agent_message.content);
                            let content_blocks = agent_message.content;
                            let payload = json!({
                                "text": text,
                                "phase": phase.clone(),
                                "content_blocks": content_blocks,
                            });
                            let item_id = match item {
                                codex_protocol::models::ResponseItem::Message { id, .. } => {
                                    id.clone()
                                }
                                _ => None,
                            };
                            trace.events.push(CanonicalTraceEvent {
                                event_id: format!("canonical-{}", trace.events.len() + 1),
                                ts_ms,
                                kind: "agent_message".to_string(),
                                payload,
                                provenance: Some(build_provenance(
                                    &index,
                                    turn_id.as_deref(),
                                    item_id.as_deref(),
                                    phase,
                                    None,
                                    None,
                                )),
                                run_id: None,
                                job_id: None,
                                parent_job_id: None,
                            });
                            increment_event_counts(
                                &mut trace.metadata.event_counts,
                                "agent_message",
                            );
                            trace.metadata.last_message_at_ms = Some(ts_ms);
                        }
                        TurnItem::Reasoning(reasoning) => {
                            let turn_id = current_turn_id.clone();
                            let reasoning_id = reasoning.id;
                            let payload = json!({
                                "summary_text": reasoning.summary_text,
                                "raw_content": reasoning.raw_content.into_iter().map(|text| json!({
                                    "type": "text",
                                    "text": text,
                                })).collect::<Vec<Value>>(),
                            });
                            trace.events.push(CanonicalTraceEvent {
                                event_id: format!("canonical-{}", trace.events.len() + 1),
                                ts_ms,
                                kind: "reasoning".to_string(),
                                payload,
                                provenance: Some(build_provenance(
                                    &index,
                                    turn_id.as_deref(),
                                    Some(reasoning_id.as_str()),
                                    None,
                                    None,
                                    None,
                                )),
                                run_id: None,
                                job_id: None,
                                parent_job_id: None,
                            });
                            increment_event_counts(&mut trace.metadata.event_counts, "reasoning");
                        }
                        TurnItem::Plan(_)
                        | TurnItem::HookPrompt(_)
                        | TurnItem::WebSearch(_)
                        | TurnItem::ImageGeneration(_)
                        | TurnItem::ContextCompaction(_) => {}
                    }
                }
            }
            RolloutItem::EventMsg(EventMsg::ExecCommandEnd(event)) => {
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "command_execution".to_string(),
                    payload: json!({
                        "command": event.command.join(" "),
                        "cwd": event.cwd.display().to_string(),
                        "status": serialized_enum_string(&event.status),
                        "aggregated_output": event.aggregated_output.clone(),
                        "exit_code": event.exit_code,
                        "duration_ms": duration_to_ms(event.duration),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        Some(event.turn_id.as_str()),
                        Some(event.call_id.as_str()),
                        None,
                        None,
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "command_execution");
            }
            RolloutItem::EventMsg(EventMsg::McpToolCallEnd(event)) => {
                let turn_id = current_turn_id.clone();
                let (status, result, error) = match &event.result {
                    Ok(result) if !result.is_error.unwrap_or(false) => (
                        "completed".to_string(),
                        serde_json::to_value(result).unwrap_or(Value::Null),
                        Value::Null,
                    ),
                    Ok(result) => (
                        "failed".to_string(),
                        serde_json::to_value(result).unwrap_or(Value::Null),
                        Value::Null,
                    ),
                    Err(error) => (
                        "failed".to_string(),
                        Value::Null,
                        json!({ "message": error }),
                    ),
                };
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "mcp_tool_call".to_string(),
                    payload: json!({
                        "server": event.invocation.server.clone(),
                        "tool": event.invocation.tool.clone(),
                        "arguments": event.invocation.arguments.clone(),
                        "status": status,
                        "result": result,
                        "error": error,
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        turn_id.as_deref(),
                        Some(event.call_id.as_str()),
                        None,
                        None,
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "mcp_tool_call");
            }
            RolloutItem::EventMsg(EventMsg::WebSearchEnd(event)) => {
                let turn_id = current_turn_id.clone();
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "web_search".to_string(),
                    payload: json!({
                        "query": event.query.clone(),
                        "action": web_search_action_string(&event.action),
                        "status": "completed",
                        "result_summary": Value::Null,
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        turn_id.as_deref(),
                        Some(event.call_id.as_str()),
                        None,
                        None,
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "web_search");
            }
            RolloutItem::EventMsg(EventMsg::SkillInvocation(event)) => {
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "skill_invocation".to_string(),
                    payload: json!({
                        "skill": event.skill.name.clone(),
                        "display_name": event.skill.display_name.clone(),
                        "trigger_mode": serialized_enum_string(&event.trigger_mode),
                        "status": serialized_enum_string(&event.status),
                        "scope": event.skill.scope.as_ref().and_then(serialized_enum_string).unwrap_or_else(|| "unknown".to_string()),
                        "source_path": event.skill.source_path.as_ref().map(|path| path.display().to_string()),
                        "input_summary": event.input_summary.clone(),
                        "output_summary": event.output_summary.clone(),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        Some(event.turn_id.as_str()),
                        None,
                        None,
                        Some(skill_context_for_invocation(event)),
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "skill_invocation");
            }
            RolloutItem::EventMsg(EventMsg::PatchApplyEnd(event)) => {
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "file_change".to_string(),
                    payload: json!({
                        "status": serialized_enum_string(&event.status),
                        "changes": normalized_file_changes(&event.changes),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        (!event.turn_id.is_empty()).then_some(event.turn_id.as_str()),
                        Some(event.call_id.as_str()),
                        None,
                        None,
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "file_change");
            }
            RolloutItem::EventMsg(EventMsg::PlanUpdate(event)) => {
                let turn_id = current_turn_id.clone();
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "plan_update".to_string(),
                    payload: json!({
                        "explanation": event.explanation.clone(),
                        "steps": event.plan.clone(),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        turn_id.as_deref(),
                        None,
                        None,
                        None,
                        None,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "plan_update");
            }
            RolloutItem::EventMsg(EventMsg::TurnComplete(event)) => {
                let usage = index.usage_by_turn.get(&event.turn_id).map(usage_json);
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "turn_completed".to_string(),
                    payload: json!({
                        "turn_id": event.turn_id.clone(),
                        "status": "completed",
                        "usage": usage.clone(),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        Some(event.turn_id.as_str()),
                        None,
                        None,
                        None,
                        usage,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "turn_completed");
            }
            RolloutItem::EventMsg(EventMsg::TurnAborted(event)) => {
                let turn_id = event.turn_id.clone().or_else(|| current_turn_id.clone());
                let usage = turn_id
                    .as_ref()
                    .and_then(|turn_id| index.usage_by_turn.get(turn_id))
                    .map(usage_json);
                trace.events.push(CanonicalTraceEvent {
                    event_id: format!("canonical-{}", trace.events.len() + 1),
                    ts_ms,
                    kind: "turn_completed".to_string(),
                    payload: json!({
                        "turn_id": turn_id.clone(),
                        "status": turn_status_from_abort_reason(&event.reason),
                        "usage": usage.clone(),
                    }),
                    provenance: Some(build_provenance(
                        &index,
                        turn_id.as_deref(),
                        None,
                        None,
                        None,
                        usage,
                    )),
                    run_id: None,
                    job_id: None,
                    parent_job_id: None,
                });
                increment_event_counts(&mut trace.metadata.event_counts, "turn_completed");
            }
            RolloutItem::SessionMeta(_) | RolloutItem::Compacted(_) | RolloutItem::EventMsg(_) => {}
        }
    }

    trace
}

fn record_turn_context(index: &mut TraceIndex, turn_context: &TurnContextItem) {
    if !turn_context.model.is_empty() {
        index.latest_model_alias = Some(turn_context.model.clone());
        if let Some(turn_id) = turn_context.turn_id.as_ref() {
            index
                .model_alias_by_turn
                .insert(turn_id.clone(), turn_context.model.clone());
        }
    }
    if let (Some(turn_id), Some(trace_id)) = (
        turn_context.turn_id.as_ref(),
        turn_context.trace_id.as_ref(),
    ) {
        index
            .trace_id_by_turn
            .insert(turn_id.clone(), trace_id.clone());
    }
}

fn build_provenance(
    index: &TraceIndex,
    turn_id: Option<&str>,
    item_id: Option<&str>,
    phase: Option<String>,
    skill_context: Option<Value>,
    usage: Option<Value>,
) -> Value {
    let skill_context_for_turn =
        turn_id.and_then(|turn_id| index.skill_context_by_turn.get(turn_id));
    let selected_skill_names = skill_context_for_turn.map(|context| {
        context
            .selected_skills
            .iter()
            .map(|skill| skill.name.clone())
            .collect::<Vec<String>>()
    });
    let effective_skill_names = skill_context_for_turn.map(|context| {
        context
            .effective_skills
            .iter()
            .map(|skill| skill.name.clone())
            .collect::<Vec<String>>()
    });

    json!({
        "origin": index.origin.clone().unwrap_or_else(|| "cli".to_string()),
        "producer": CANONICAL_TRACE_PRODUCER,
        "turn_id": turn_id,
        "item_id": item_id,
        "agent_name": index.agent_name.clone().unwrap_or_else(|| "Codex".to_string()),
        "model_alias": turn_id
            .and_then(|turn_id| index.model_alias_by_turn.get(turn_id))
            .cloned()
            .or_else(|| index.latest_model_alias.clone()),
        "model_provider": index.model_provider.clone(),
        "phase": phase,
        "selected_skill_names": selected_skill_names,
        "effective_skill_names": effective_skill_names,
        "skill_context": skill_context,
        "session_id": index.session_id.map(|session_id| session_id.to_string()),
        "trace_id": turn_id.and_then(|turn_id| index.trace_id_by_turn.get(turn_id)).cloned(),
        "conversation_id": Value::Null,
        "user_id": Value::Null,
        "email": Value::Null,
        "usage": usage,
    })
}

fn usage_json(usage: &TokenUsage) -> Value {
    json!({
        "input_tokens": usage.input_tokens,
        "output_tokens": usage.output_tokens,
        "total_tokens": usage.total_tokens,
    })
}

fn normalized_file_changes(changes: &HashMap<std::path::PathBuf, FileChange>) -> Vec<Value> {
    let mut normalized = changes
        .iter()
        .map(|(path, change)| {
            let (kind, diff) = match change {
                FileChange::Add { content } => ("add", Some(content.clone())),
                FileChange::Delete { content } => ("delete", Some(content.clone())),
                FileChange::Update { unified_diff, .. } => ("update", Some(unified_diff.clone())),
            };
            json!({
                "path": path.display().to_string(),
                "kind": kind,
                "diff": diff,
            })
        })
        .collect::<Vec<Value>>();
    normalized.sort_by(|left, right| {
        let left_path = left.get("path").and_then(Value::as_str).unwrap_or_default();
        let right_path = right
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        left_path.cmp(right_path)
    });
    normalized
}

fn skill_context_for_invocation(event: &SkillInvocationEvent) -> Value {
    json!([
        {
            "name": event.skill.name.clone(),
            "scope": event.skill.scope.as_ref().and_then(serialized_enum_string).unwrap_or_else(|| "unknown".to_string()),
            "trigger_mode": serialized_enum_string(&event.trigger_mode),
            "source_path": event.skill.source_path.as_ref().map(|path| path.display().to_string()),
        }
    ])
}

fn join_agent_message_text(content: &[AgentMessageContent]) -> String {
    content
        .iter()
        .map(|item| match item {
            AgentMessageContent::Text { text } => text.as_str(),
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn increment_event_counts(counts: &mut CanonicalEventCounts, kind: &str) {
    counts.total += 1;
    match kind {
        "user_message" => counts.prompts += 1,
        "agent_message" | "assistant_message" => counts.responses += 1,
        "reasoning" => counts.thinking += 1,
        "command_execution" | "mcp_tool_call" | "web_search" => counts.tool_calls += 1,
        "skill_invocation" => counts.skills += 1,
        "file_change" => counts.file_changes += 1,
        "plan_update" => counts.plans += 1,
        "turn_started" | "turn_completed" | "context_compaction" | "error" => {
            counts.system += 1;
        }
        _ => {}
    }
}

fn turn_status_from_abort_reason(reason: &TurnAbortReason) -> &'static str {
    match reason {
        TurnAbortReason::Interrupted => "interrupted",
        TurnAbortReason::Replaced => "replaced",
        TurnAbortReason::ReviewEnded => "review_ended",
    }
}

fn origin_from_session_source(source: &SessionSource) -> String {
    match source {
        SessionSource::Custom(origin) => origin.clone(),
        SessionSource::Cli
        | SessionSource::VSCode
        | SessionSource::Exec
        | SessionSource::Mcp
        | SessionSource::SubAgent(_)
        | SessionSource::Unknown => "cli".to_string(),
    }
}

fn serialized_enum_string<T: Serialize>(value: &T) -> Option<String> {
    match serde_json::to_value(value).ok()? {
        Value::String(value) => Some(value),
        _ => None,
    }
}

fn message_phase_string(phase: &codex_protocol::models::MessagePhase) -> Option<String> {
    match phase {
        codex_protocol::models::MessagePhase::Commentary => Some("commentary".to_string()),
        codex_protocol::models::MessagePhase::FinalAnswer => Some("final".to_string()),
    }
}

fn web_search_action_string(action: &codex_protocol::models::WebSearchAction) -> &'static str {
    match action {
        codex_protocol::models::WebSearchAction::Search { .. } => "search",
        codex_protocol::models::WebSearchAction::OpenPage { .. } => "open_page",
        codex_protocol::models::WebSearchAction::FindInPage { .. } => "find_in_page",
        codex_protocol::models::WebSearchAction::Other => "other",
    }
}

fn duration_to_ms(duration: Duration) -> i64 {
    duration.as_millis().min(i64::MAX as u128) as i64
}

fn timestamp_to_ms(timestamp: &str) -> i64 {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|datetime| datetime.with_timezone(&Utc).timestamp_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::time::Duration;

    use codex_protocol::ThreadId;
    use codex_protocol::config_types::ReasoningSummary;
    use codex_protocol::mcp::CallToolResult;
    use codex_protocol::models::ContentItem;
    use codex_protocol::models::MessagePhase;
    use codex_protocol::models::ResponseItem;
    use codex_protocol::models::WebSearchAction;
    use codex_protocol::plan_tool::PlanItemArg;
    use codex_protocol::plan_tool::StepStatus;
    use codex_protocol::plan_tool::UpdatePlanArgs;
    use codex_protocol::protocol::AskForApproval;
    use codex_protocol::protocol::EventMsg;
    use codex_protocol::protocol::ExecCommandEndEvent;
    use codex_protocol::protocol::ExecCommandSource;
    use codex_protocol::protocol::ExecCommandStatus;
    use codex_protocol::protocol::FileChange;
    use codex_protocol::protocol::McpInvocation;
    use codex_protocol::protocol::McpToolCallEndEvent;
    use codex_protocol::protocol::PatchApplyEndEvent;
    use codex_protocol::protocol::PatchApplyStatus;
    use codex_protocol::protocol::RolloutItem;
    use codex_protocol::protocol::RolloutLine;
    use codex_protocol::protocol::SandboxPolicy;
    use codex_protocol::protocol::SessionMeta;
    use codex_protocol::protocol::SessionMetaLine;
    use codex_protocol::protocol::SessionSource;
    use codex_protocol::protocol::SkillInvocationEvent;
    use codex_protocol::protocol::SkillInvocationStatus;
    use codex_protocol::protocol::SkillInvocationTriggerMode;
    use codex_protocol::protocol::SkillScope;
    use codex_protocol::protocol::TokenCountEvent;
    use codex_protocol::protocol::TokenUsage;
    use codex_protocol::protocol::TokenUsageInfo;
    use codex_protocol::protocol::TurnAbortReason;
    use codex_protocol::protocol::TurnAbortedEvent;
    use codex_protocol::protocol::TurnCompleteEvent;
    use codex_protocol::protocol::TurnContextItem;
    use codex_protocol::protocol::TurnSkillContextEvent;
    use codex_protocol::protocol::TurnSkillDescriptor;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use serde_json::json;

    use super::build_canonical_trace;

    #[test]
    fn normalizes_mixed_turn_trace_deterministically() {
        let thread_id =
            ThreadId::from_string("11111111-1111-4111-8111-111111111111").expect("thread id");
        let turn_id = "turn-1".to_string();
        let trace_id = "trace-1".to_string();
        let selected_skill = TurnSkillDescriptor {
            name: "playwright-interactive".to_string(),
            display_name: Some("Playwright Interactive".to_string()),
            source_path: Some(PathBuf::from(
                "/Users/gp/.codex/skills/playwright-interactive/SKILL.md",
            )),
            scope: Some(SkillScope::Repo),
        };

        let lines = vec![
            line(
                "2026-03-13T08:00:00Z",
                RolloutItem::SessionMeta(SessionMetaLine {
                    meta: SessionMeta {
                        id: thread_id,
                        timestamp: "2026-03-13T08:00:00Z".to_string(),
                        cwd: PathBuf::from("/repo"),
                        originator: "codex".to_string(),
                        cli_version: "1.0.0".to_string(),
                        source: SessionSource::Cli,
                        agent_nickname: None,
                        agent_role: None,
                        model_provider: Some("openai".to_string()),
                        forked_from_id: None,
                        base_instructions: None,
                        dynamic_tools: None,
                        memory_mode: None,
                    },
                    git: None,
                }),
            ),
            line(
                "2026-03-13T08:00:01Z",
                RolloutItem::TurnContext(TurnContextItem {
                    turn_id: Some(turn_id.clone()),
                    trace_id: Some(trace_id),
                    cwd: PathBuf::from("/repo"),
                    current_date: None,
                    timezone: None,
                    approval_policy: AskForApproval::Never,
                    sandbox_policy: SandboxPolicy::new_read_only_policy(),
                    network: None,
                    model: "gpt-5".to_string(),
                    personality: None,
                    collaboration_mode: None,
                    realtime_active: None,
                    effort: None,
                    summary: ReasoningSummary::default(),
                    user_instructions: None,
                    developer_instructions: None,
                    final_output_json_schema: None,
                    truncation_policy: None,
                }),
            ),
            line(
                "2026-03-13T08:00:02Z",
                RolloutItem::EventMsg(EventMsg::TurnSkillContext(TurnSkillContextEvent {
                    turn_id: turn_id.clone(),
                    selected_skills: vec![selected_skill.clone()],
                    effective_skills: vec![selected_skill.clone()],
                })),
            ),
            line(
                "2026-03-13T08:00:03Z",
                RolloutItem::ResponseItem(ResponseItem::Message {
                    id: None,
                    role: "user".to_string(),
                    content: vec![ContentItem::InputText {
                        text: "Investigate the flaky modal.".to_string(),
                    }],
                    end_turn: None,
                    phase: None,
                }),
            ),
            line(
                "2026-03-13T08:00:04Z",
                RolloutItem::ResponseItem(ResponseItem::Reasoning {
                    id: "rsn-1".to_string(),
                    summary: vec![
                        codex_protocol::models::ReasoningItemReasoningSummary::SummaryText {
                            text: "Need to gather browser state.".to_string(),
                        },
                    ],
                    content: Some(vec![
                        codex_protocol::models::ReasoningItemContent::ReasoningText {
                            text: "The modal likely races against hydration.".to_string(),
                        },
                    ]),
                    encrypted_content: None,
                }),
            ),
            line(
                "2026-03-13T08:00:05Z",
                RolloutItem::EventMsg(EventMsg::ExecCommandEnd(ExecCommandEndEvent {
                    call_id: "cmd-1".to_string(),
                    process_id: None,
                    turn_id: turn_id.clone(),
                    command: vec!["cargo".to_string(), "test".to_string()],
                    cwd: PathBuf::from("/repo"),
                    parsed_cmd: vec![],
                    source: ExecCommandSource::Agent,
                    interaction_input: None,
                    stdout: String::new(),
                    stderr: String::new(),
                    aggregated_output: "test output".to_string(),
                    exit_code: 0,
                    duration: Duration::from_millis(250),
                    formatted_output: String::new(),
                    status: ExecCommandStatus::Completed,
                })),
            ),
            line(
                "2026-03-13T08:00:06Z",
                RolloutItem::EventMsg(EventMsg::McpToolCallEnd(McpToolCallEndEvent {
                    call_id: "mcp-1".to_string(),
                    invocation: McpInvocation {
                        server: "chrome-devtools".to_string(),
                        tool: "take_snapshot".to_string(),
                        arguments: Some(json!({"full": true})),
                    },
                    duration: Duration::from_millis(100),
                    result: Ok(CallToolResult {
                        content: vec![json!({"ok": true})],
                        structured_content: Some(json!({"snapshot": "captured"})),
                        is_error: Some(false),
                        meta: None,
                    }),
                })),
            ),
            line(
                "2026-03-13T08:00:07Z",
                RolloutItem::EventMsg(EventMsg::WebSearchEnd(
                    codex_protocol::protocol::WebSearchEndEvent {
                        call_id: "web-1".to_string(),
                        query: "playwright modal hydration race".to_string(),
                        action: WebSearchAction::Search {
                            query: None,
                            queries: None,
                        },
                    },
                )),
            ),
            line(
                "2026-03-13T08:00:08Z",
                RolloutItem::EventMsg(EventMsg::SkillInvocation(SkillInvocationEvent {
                    turn_id: turn_id.clone(),
                    skill: selected_skill,
                    trigger_mode: SkillInvocationTriggerMode::Implicit,
                    status: SkillInvocationStatus::Completed,
                    input_summary: Some("Opened the browser workflow.".to_string()),
                    output_summary: Some("Captured the DOM after hydration.".to_string()),
                })),
            ),
            line(
                "2026-03-13T08:00:09Z",
                RolloutItem::EventMsg(EventMsg::PatchApplyEnd(PatchApplyEndEvent {
                    call_id: "patch-1".to_string(),
                    turn_id: turn_id.clone(),
                    stdout: String::new(),
                    stderr: String::new(),
                    success: true,
                    changes: HashMap::from([(
                        PathBuf::from("src/modal.tsx"),
                        FileChange::Update {
                            unified_diff: "@@ -1 +1 @@".to_string(),
                            move_path: None,
                        },
                    )]),
                    status: PatchApplyStatus::Completed,
                })),
            ),
            line(
                "2026-03-13T08:00:10Z",
                RolloutItem::EventMsg(EventMsg::PlanUpdate(UpdatePlanArgs {
                    explanation: Some("Keep the fix minimal.".to_string()),
                    plan: vec![PlanItemArg {
                        step: "Add a readiness guard".to_string(),
                        status: StepStatus::InProgress,
                    }],
                })),
            ),
            line(
                "2026-03-13T08:00:11Z",
                RolloutItem::ResponseItem(ResponseItem::Message {
                    id: Some("msg-1".to_string()),
                    role: "assistant".to_string(),
                    content: vec![ContentItem::OutputText {
                        text: "I added the readiness guard and verified the modal path."
                            .to_string(),
                    }],
                    end_turn: None,
                    phase: Some(MessagePhase::FinalAnswer),
                }),
            ),
            line(
                "2026-03-13T08:00:12Z",
                RolloutItem::EventMsg(EventMsg::TokenCount(TokenCountEvent {
                    info: Some(TokenUsageInfo {
                        total_token_usage: TokenUsage::default(),
                        last_token_usage: TokenUsage {
                            input_tokens: 10,
                            cached_input_tokens: 0,
                            output_tokens: 5,
                            reasoning_output_tokens: 0,
                            total_tokens: 15,
                        },
                        model_context_window: None,
                    }),
                    rate_limits: None,
                })),
            ),
            line(
                "2026-03-13T08:00:13Z",
                RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
                    turn_id,
                    last_agent_message: Some(
                        "I added the readiness guard and verified the modal path.".to_string(),
                    ),
                })),
            ),
        ];

        let trace = build_canonical_trace(&lines);

        assert_eq!(
            trace.metadata.title.as_deref(),
            Some("Investigate the flaky modal.")
        );
        assert_eq!(trace.metadata.model_alias.as_deref(), Some("gpt-5"));
        assert_eq!(trace.metadata.model_provider.as_deref(), Some("openai"));
        assert_eq!(
            trace.metadata.last_message_at_ms,
            Some(super::timestamp_to_ms("2026-03-13T08:00:11Z"))
        );
        assert_eq!(trace.metadata.event_counts.total, 10);
        assert_eq!(trace.metadata.event_counts.prompts, 1);
        assert_eq!(trace.metadata.event_counts.responses, 1);
        assert_eq!(trace.metadata.event_counts.thinking, 1);
        assert_eq!(trace.metadata.event_counts.tool_calls, 3);
        assert_eq!(trace.metadata.event_counts.skills, 1);
        assert_eq!(trace.metadata.event_counts.file_changes, 1);
        assert_eq!(trace.metadata.event_counts.plans, 1);
        assert_eq!(trace.metadata.event_counts.system, 1);
        assert_eq!(
            trace
                .events
                .iter()
                .map(|event| event.kind.as_str())
                .collect::<Vec<&str>>(),
            vec![
                "user_message",
                "reasoning",
                "command_execution",
                "mcp_tool_call",
                "web_search",
                "skill_invocation",
                "file_change",
                "plan_update",
                "agent_message",
                "turn_completed",
            ]
        );
        assert_eq!(trace.events[0].event_id, "canonical-1");
        assert_eq!(
            trace.events[5].payload,
            json!({
                "skill": "playwright-interactive",
                "display_name": "Playwright Interactive",
                "trigger_mode": "implicit",
                "status": "completed",
                "scope": "repo",
                "source_path": "/Users/gp/.codex/skills/playwright-interactive/SKILL.md",
                "input_summary": "Opened the browser workflow.",
                "output_summary": "Captured the DOM after hydration.",
            })
        );
        assert_eq!(
            trace.events[9].payload,
            json!({
                "turn_id": "turn-1",
                "status": "completed",
                "usage": {
                    "input_tokens": 10,
                    "output_tokens": 5,
                    "total_tokens": 15,
                }
            })
        );
        assert_eq!(
            trace.events[5]
                .provenance
                .as_ref()
                .and_then(|value| value.get("selected_skill_names"))
                .cloned(),
            Some(json!(["playwright-interactive"]))
        );
        assert_eq!(
            trace.events[0]
                .provenance
                .as_ref()
                .and_then(|value| value.get("trace_id"))
                .cloned(),
            Some(json!("trace-1"))
        );
    }

    #[test]
    fn preserves_selected_and_effective_skill_context_without_fake_invocations() {
        let thread_id =
            ThreadId::from_string("22222222-2222-4222-8222-222222222222").expect("thread id");
        let turn_id = "turn-2".to_string();
        let selected = vec![
            TurnSkillDescriptor {
                name: "codex-upstream-parity-sync".to_string(),
                display_name: Some("Upstream Parity Sync".to_string()),
                source_path: Some(PathBuf::from(
                    "/Users/gp/.codex/skills/codex-upstream-parity-sync/SKILL.md",
                )),
                scope: Some(SkillScope::User),
            },
            TurnSkillDescriptor {
                name: "playwright-interactive".to_string(),
                display_name: Some("Playwright Interactive".to_string()),
                source_path: Some(PathBuf::from(
                    "/Users/gp/.codex/skills/playwright-interactive/SKILL.md",
                )),
                scope: Some(SkillScope::Repo),
            },
        ];
        let effective = vec![selected[1].clone()];
        let lines = vec![
            line(
                "2026-03-13T09:00:00Z",
                RolloutItem::SessionMeta(SessionMetaLine {
                    meta: SessionMeta {
                        id: thread_id,
                        timestamp: "2026-03-13T09:00:00Z".to_string(),
                        cwd: PathBuf::from("/repo"),
                        originator: "codex".to_string(),
                        cli_version: "1.0.0".to_string(),
                        source: SessionSource::Cli,
                        agent_nickname: None,
                        agent_role: None,
                        model_provider: Some("openai".to_string()),
                        forked_from_id: None,
                        base_instructions: None,
                        dynamic_tools: None,
                        memory_mode: None,
                    },
                    git: None,
                }),
            ),
            line(
                "2026-03-13T09:00:01Z",
                RolloutItem::TurnContext(TurnContextItem {
                    turn_id: Some(turn_id.clone()),
                    trace_id: None,
                    cwd: PathBuf::from("/repo"),
                    current_date: None,
                    timezone: None,
                    approval_policy: AskForApproval::Never,
                    sandbox_policy: SandboxPolicy::new_read_only_policy(),
                    network: None,
                    model: "gpt-5".to_string(),
                    personality: None,
                    collaboration_mode: None,
                    realtime_active: None,
                    effort: None,
                    summary: ReasoningSummary::default(),
                    user_instructions: None,
                    developer_instructions: None,
                    final_output_json_schema: None,
                    truncation_policy: None,
                }),
            ),
            line(
                "2026-03-13T09:00:02Z",
                RolloutItem::EventMsg(EventMsg::TurnSkillContext(TurnSkillContextEvent {
                    turn_id: turn_id.clone(),
                    selected_skills: selected.clone(),
                    effective_skills: effective.clone(),
                })),
            ),
            line(
                "2026-03-13T09:00:03Z",
                RolloutItem::EventMsg(EventMsg::SkillInvocation(SkillInvocationEvent {
                    turn_id,
                    skill: effective[0].clone(),
                    trigger_mode: SkillInvocationTriggerMode::Implicit,
                    status: SkillInvocationStatus::Completed,
                    input_summary: Some("Opened browser automation.".to_string()),
                    output_summary: Some("Captured live DOM.".to_string()),
                })),
            ),
        ];

        let trace = build_canonical_trace(&lines);

        assert_eq!(trace.metadata.selected_skills, Some(selected));
        assert_eq!(trace.metadata.effective_skills, Some(effective));
        assert_eq!(trace.metadata.event_counts.skills, 1);
        assert_eq!(trace.events.len(), 1);
        assert_eq!(trace.events[0].kind, "skill_invocation");
        assert_eq!(
            trace.events[0]
                .provenance
                .as_ref()
                .and_then(|value| value.get("selected_skill_names"))
                .cloned(),
            Some(json!([
                "codex-upstream-parity-sync",
                "playwright-interactive"
            ]))
        );
        assert_eq!(
            trace.events[0]
                .provenance
                .as_ref()
                .and_then(|value| value.get("effective_skill_names"))
                .cloned(),
            Some(json!(["playwright-interactive"]))
        );
    }

    #[test]
    fn maps_interrupted_turns_to_interrupted_completion_status() {
        let thread_id =
            ThreadId::from_string("33333333-3333-4333-8333-333333333333").expect("thread id");
        let lines = vec![
            line(
                "2026-03-13T10:00:00Z",
                RolloutItem::SessionMeta(SessionMetaLine {
                    meta: SessionMeta {
                        id: thread_id,
                        timestamp: "2026-03-13T10:00:00Z".to_string(),
                        cwd: PathBuf::from("/repo"),
                        originator: "codex".to_string(),
                        cli_version: "1.0.0".to_string(),
                        source: SessionSource::Cli,
                        agent_nickname: None,
                        agent_role: None,
                        model_provider: Some("openai".to_string()),
                        forked_from_id: None,
                        base_instructions: None,
                        dynamic_tools: None,
                        memory_mode: None,
                    },
                    git: None,
                }),
            ),
            line(
                "2026-03-13T10:00:01Z",
                RolloutItem::TurnContext(TurnContextItem {
                    turn_id: Some("turn-3".to_string()),
                    trace_id: None,
                    cwd: PathBuf::from("/repo"),
                    current_date: None,
                    timezone: None,
                    approval_policy: AskForApproval::Never,
                    sandbox_policy: SandboxPolicy::new_read_only_policy(),
                    network: None,
                    model: "gpt-5".to_string(),
                    personality: None,
                    collaboration_mode: None,
                    realtime_active: None,
                    effort: None,
                    summary: ReasoningSummary::default(),
                    user_instructions: None,
                    developer_instructions: None,
                    final_output_json_schema: None,
                    truncation_policy: None,
                }),
            ),
            line(
                "2026-03-13T10:00:02Z",
                RolloutItem::EventMsg(EventMsg::TurnAborted(TurnAbortedEvent {
                    turn_id: Some("turn-3".to_string()),
                    reason: TurnAbortReason::Interrupted,
                })),
            ),
        ];

        let trace = build_canonical_trace(&lines);

        assert_eq!(trace.events.len(), 1);
        assert_eq!(trace.events[0].kind, "turn_completed");
        assert_eq!(
            trace.events[0].payload,
            json!({
                "turn_id": "turn-3",
                "status": "interrupted",
                "usage": Value::Null,
            })
        );
    }

    fn line(timestamp: &str, item: RolloutItem) -> RolloutLine {
        RolloutLine {
            timestamp: timestamp.to_string(),
            item,
        }
    }
}
