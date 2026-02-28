use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;

const FIXTURE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../fixtures/bedrock");

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BedrockConverseFixture {
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BedrockStreamFixture {
    pub chunks: Vec<BedrockStreamChunk>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BedrockStreamChunk {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub value: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
}

pub fn fixture_root() -> &'static Path {
    Path::new(FIXTURE_ROOT)
}

fn fixture_path(name: &str) -> PathBuf {
    fixture_root().join(name)
}

fn load_json_fixture<T: DeserializeOwned>(name: &str) -> T {
    let path = fixture_path(name);
    let file = std::fs::File::open(&path)
        .unwrap_or_else(|err| panic!("failed to open fixture {}: {err}", path.display()));
    serde_json::from_reader(file)
        .unwrap_or_else(|err| panic!("failed to parse fixture {}: {err}", path.display()))
}

pub fn load_converse_fixture(name: &str) -> BedrockConverseFixture {
    load_json_fixture(name)
}

pub fn load_converse_payload(name: &str) -> Value {
    load_converse_fixture(name).payload
}

pub fn load_stream_fixture(name: &str) -> BedrockStreamFixture {
    load_json_fixture(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn loads_converse_payload_fixture() {
        let payload = load_converse_payload("converse_basic.json");
        assert_eq!(
            payload.pointer("/output/message/content/0/text"),
            Some(&Value::String("Hello from Bedrock!".to_string()))
        );
    }

    #[test]
    fn loads_stream_fixture_chunk_sequence() {
        let fixture = load_stream_fixture("stream_basic.json");
        let kinds: Vec<&str> = fixture
            .chunks
            .iter()
            .map(|chunk| chunk.kind.as_str())
            .collect();
        assert_eq!(
            kinds,
            vec!["message_delta", "message_delta", "usage", "done"]
        );
    }
}
