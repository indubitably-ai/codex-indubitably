# Bedrock Test Fixtures

Fixtures in this directory model payloads exchanged with AWS Bedrock's Converse and ConverseStream APIs. They will be fleshed out alongside the mock runtime so tests can hydrate scripted responses without embedding large JSON blobs inline.

Available files:

- `converse_basic.json` – non-streaming text completion
- `converse_tool_call.json` – assistant tool invocation
- `converse_tool_result.json` – tool result payload
- `stream_basic.json` – streaming deltas for a simple response
- `stream_multi_tool.json` – streaming flow covering multiple tool_use/tool_result pairs
- `stream_throttled.json` – streaming sequence yielding a throttling error after initial delta
- `stream_drop_and_resume.json` – initial response fragment followed by a transport drop
- `stream_resume_text.json` – resumed text response that completes with usage metrics
- `stream_resume_tool.json` – resumed stream containing a tool call, tool result, and completion
- `error_throttled.json` – throttling error body for non-streaming flows
