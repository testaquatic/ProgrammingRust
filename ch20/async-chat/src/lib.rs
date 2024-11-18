use std::sync::Arc;

pub mod args;
pub mod utils;

/// 클라이언트가 서버에 보내는 패킷
#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

/// 서버가 클라이언트에 보내는 패킷
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_fromclient_json() {
    let from_client = FromClient::Post {
        group_name: Arc::new("Dogs".to_string()),
        message: Arc::new("Samoyeds rock!".to_string()),
    };

    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"Post": {"group_name": "Dogs", "message": "Samoyeds rock!"}})
            .to_string()
    );
}
