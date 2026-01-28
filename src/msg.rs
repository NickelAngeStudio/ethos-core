use std::{io::Read, time::Duration};

use crate::{error::EthosError, payload::EthosMessagePayload};

// TODO: Automate via macros

/// Message sent between client and server
pub struct EthosMessage {

    /// Size of the message including payload
    size : u16,

    /// Type of message
    msg_type : u16,

    /// Special shared key used to validate message
    key  :u128,

    /// Timestamp of the message (usually in sync with server)
    timestamp : u64,

    /// Payload of the message
    payload : EthosMessagePayload,

}

/*
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

    pub const fn from_bytes(bytes: &Vec<u8>) -> Result<NetMessage, EthosCoreNetError>{
        let aa = bytes[0];
        let aa = <&[u8; 2]>::try_from(&bytes[0..2]);

        match NetMessageType::from_bytes(<&[u8; 2]>::try_from(bytes[0..2])) {
            Ok(_) => todo!(),
            Err(_) => Err(EthosCoreNetError::ECNetErrorInvalidNetMessageType),  // Invalid message type
        }


        let aa = <&[u8; 2]>::try_from(bytes[0..2]);
        match <&[u8; 2]>::try_from(bytes) {
            Ok(msg_type) => {
                match NetMessageType::from_bytes(msg_type) {
                    Ok(_) => todo!(),
                    Err(_) => Err(EthosCoreNetError::ECNetErrorInvalidNetMessageType),  // Invalid message type
                }

            },
            Err(_) => { // Malformed message
                Err(EthosCoreNetError::ECNetErrorInvalidNetMessage)
            }
        } 

    }


    pub const fn to_bytes(&self) -> &[u8] {
        
        
    }

   
}
*/

#[cfg(test)]
mod tests {

    /*
    use crate::msg::{self, EthosMessage, EthosMessageType};

      #[test]
    fn message_to_bytes() {
        let msg = EthosMessage::Action(8, 12, 12, 12);

        println!("Test={:?}", msg.to_bytes());

        let msg = EthosMessage::Error(crate::error::EthosCoreNetError::ECNetErrorInvalidNetMessage);

        println!("Test={:?}", msg.to_bytes());
    }


     #[test]
    fn create_message_from_bytes() {
        let msg1 = [1 as u8, 0, 8, 0, 12, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0];
        let msg2 = [0 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 196, 170, 85];
        let msg3 = [10 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 196, 170, 85];

        
    
        match EthosMessage::from_bytes(&msg3){
                Ok(msg) => {
                    println!("MSG={:?}", msg);
                    match msg {
                    EthosMessage::Error(ethos_core_net_error) => panic!("WRONG"),
                    EthosMessage::Action(a,b_, c, d) => assert_eq!(*a, 8),
                    _ => panic!("WTFF!!")
                }
            },
            Err(_) => todo!(),
        }

    }
    */

}