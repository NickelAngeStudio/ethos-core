use crate::{error::EthosError, payload::{EthosMessagePayload, PayloadDiscriminantType}};

/// Minimum message size possible used to prevent buffer overflow
const MINIMUM_MESSAGE_SIZE : usize = size_of::<u16>() + size_of::<u64>() + size_of::<PayloadDiscriminantType>();

/// Message sent between client and server
pub struct NetMessage {

    /// Size of the message including payload
    size : u16,

    /// Timestamp of the message (usually in sync with server)
    timestamp : u64,

    /// Payload of the message
    payload : EthosMessagePayload,

}

impl NetMessage {

    /// Create a new [NetMessage] from timestamp and payload
    pub fn new(timestamp : u64, payload : EthosMessagePayload) -> NetMessage {
        NetMessage { size: size_of::<u16>() as u16 + size_of::<u64>() as u16 + payload.size_of_bytes() as u16, timestamp, payload }
    }

    /// Extract a message from an array of bytes 
    pub fn from_bytes(bytes : &[u8]) -> Result<NetMessage, EthosError> {

        if bytes.len() >= MINIMUM_MESSAGE_SIZE {
            // Read the size
            let size = crate::read_buffer!(u16, bytes, 0);

            // Read timestamp
            let timestamp : u64 = crate::read_buffer!(u64, bytes, size_of::<u16>());

            // Extract payload
            match EthosMessagePayload::from_bytes(&bytes[size_of::<u16>() + size_of::<u64>()..size as usize]) {
                Ok(payload) => Ok(NetMessage { size, timestamp, payload }),
                Err(err) => Err(err),
            }

        } else {
            Err(EthosError::InvalidNetMessage)
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::msg::NetMessage;


    #[test]
    fn test(){
        //let aa = NetMessage::from_bytes(bytes);
    }

}