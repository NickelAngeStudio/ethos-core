use crate::error::EthosCoreNetError;

// TODO: Automate via macros

/// Possible types of message sent between client and server
#[repr(u16)]
pub enum NetMessageType {
    Action = 0,

    Error = 65535
}


impl NetMessageType {
    pub const fn from_bytes(bytes: &[u8; 2]) -> Result<NetMessageType, EthosCoreNetError>{ 
        match u16::from_le_bytes(*bytes) {
            0 => Ok(NetMessageType::Action),
            65535 => Ok(NetMessageType::Error),
            _ => Err(EthosCoreNetError::ECNetErrorInvalidNetMessageType)
        }
    }
}

pub struct NetMessage {
    /// Type of message sent
    message_type : NetMessageType,
    /// Real size of the message in bytes
    size_bytes : usize,
    /// Payload of the message
    payload : NetMessagePayload
}


/// Message sent via TCP or UDP
/// Those are currently templates
pub enum NetMessagePayload {
    Error(EthosCoreNetError),
    /// Action : enum, character, value, value 
    Action(u16, u32, u32, u32),
}



impl NetMessage {

    pub const fn from_bytes(bytes: &[u8]) -> Result<NetMessage, EthosCoreNetError>{

        match <&[u8; 2]>::try_from(bytes) {

        } // bytes[0..2] u16::from_le_bytes(bytes.);

        match NetMessageType::from_bytes(bytes[0..2]){
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }


    pub const fn to_bytes(&self) -> &[u8] {
        
        
    }

   
}

#[cfg(test)]
mod tests {
    use crate::msg::NetMessageType;


     #[test]
    fn create_message_from_bytes() {
        let bytes =  [0 as u8,1,1,1];
    
        let b2 = &bytes[0..1];
        let test = u16::from_le_bytes(<[u8; 2]>::try_from(&bytes[0..std::mem::size_of::<NetMessageType>()]).unwrap());

        println!("Test={}", test)
    }

}