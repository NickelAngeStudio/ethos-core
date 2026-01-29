//! Objects related to communication between client and server.

#[doc(hidden)]
mod macros;

#[doc(hidden)]
pub mod payload;

#[doc(hidden)]
pub mod msg;

// Re-export
pub use msg::Message as Message;
pub use payload::Payload as Payload;

/// Recommended buffer size (1mb) to read datas from net stream
pub const READ_BUFFER_SIZE : usize = 1024*1024;

/// Recommended buffer size for [Message::pack_bytes]
pub const PACK_BUFFER_SIZE : usize = size_of::<Message>();

/// Ethos TCP port 3847
/// 
/// Note
/// ('eths' on a phone keyboard)
pub const TCP_PORT : u16 = 3847;

/// Ethos UDP port 38467
/// 
/// Note
/// ('ethos' on a phone keyboard)
pub const UDP_PORT : u16 = 38467;