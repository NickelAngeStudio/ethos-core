#[repr(u16)]
pub enum EthosCoreNetError {
    /// Happens when server or client received an invalid message.
    ECNetErrorInvalidNetMessage = 0,

    /// Happens when server or client received an invalid message type.
    ECNetErrorInvalidNetMessageType,
}