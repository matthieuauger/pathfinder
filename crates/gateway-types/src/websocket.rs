// Types used for web socket subscription events
use crate::reply::Block;
use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize)]
pub struct WebsocketEventSync {
    pub block: Box<Block>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebsocketEventNewHeadEvent {
    pub block: Box<Block>,
}

#[derive(Debug, Clone)]
pub struct WebsocketSenders {
    pub sync: broadcast::Sender<WebsocketEventSync>,
    pub new_head: broadcast::Sender<WebsocketEventNewHeadEvent>,
}

impl WebsocketSenders {
    pub fn new() -> WebsocketSenders {
        return WebsocketSenders {
            sync: broadcast::channel(100).0,
            new_head: broadcast::channel(100).0,
        };
    }
}
