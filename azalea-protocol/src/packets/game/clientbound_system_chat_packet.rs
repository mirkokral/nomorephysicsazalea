use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSystemChatPacket {
    pub content: Component,
    pub overlay: bool,
}
