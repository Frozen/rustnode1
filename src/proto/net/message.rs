use crate::proto::net::peers::PeersMessage;

pub enum Message {
    GetPeersMessage,
    PeersMessage(PeersMessage),
}
