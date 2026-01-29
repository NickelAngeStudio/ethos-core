use std::{u16::MAX, vec};

use crate::error::{EthosError, EthosErrorDiscriminantSize};
use tampon::Tampon;

/// Type of the discriminant that match each payload enum 
/// 
/// Note
/// Max 65535 enum, make u32 if not enough 
pub type PayloadDiscriminantType = u16;

/// Allow up to u32::MAX message of error
pub type MessageErrorType = u32;

crate::write_payloads!{
    /// Key used to authenticate with server
    Key { key : u128 } = 0,
    
    /// Action : enum, character, value, value
    Action {a : u16, b : u32, c : u32, d : u32 } = 1,

    /// An error message
    Error { err : u32 } = 65534,

    /// Invalid payload
    Invalid = 65535
}

/*
#[cfg(test)]
mod tests {

    use std::{u16, u32};

    use crate::payload::EthosMessagePayload;

    
    #[test]
    fn test() {
        let payloads = [

            EthosMessagePayload::Action { a: 0, b: 0, c: 0, d: 0 },
            EthosMessagePayload::Action { a: 5, b: 6, c: 7, d: 8},
            EthosMessagePayload::Action { a: 111, b: 112, c: 113, d: 114 },
            EthosMessagePayload::Action { a: u16::MAX /2, b: u32::MAX /2, c: u32::MAX /2, d: u32::MAX /2 },
            EthosMessagePayload::Action { a: u16::MAX - 1, b: u32::MAX -1, c: u32::MAX -1, d: u32::MAX -1 },
            EthosMessagePayload::Action { a: u16::MAX, b: u32::MAX, c: u32::MAX, d: u32::MAX },

            EthosMessagePayload::Error { err:0  },
            EthosMessagePayload::Error{ err:1  },
            EthosMessagePayload::Error{ err:2  },
            EthosMessagePayload::Error{ err:3  },
            EthosMessagePayload::Error{ err:u32::MAX  },


        ];

        let aa = EthosMessagePayload::Action{ a: 0, b: 0, c: 0, d:0 };

        for pl in payloads {
             println!("Payload={:?}, bytes={:?}", pl, pl.to_le_bytes());
        }

    }

    /// Create a payload from bytes and test parameters
    #[test]
    fn payload_action_from_bytes(){
        let bytes = [1 as u8, 0, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127];
        
        match EthosMessagePayload::from_bytes(&bytes) {
            Ok(payload) => {
                match payload {
                    EthosMessagePayload::Action { a, b, c, d } => {
                        assert_eq!(a, 32767);
                        assert_eq!(b, 2147483647);
                        assert_eq!(c, 2147483647);
                        assert_eq!(d, 2147483647);
                    },
                    _ => panic!("Wrong payload type!"),
                    
                }
            },
            Err(err) => panic!("Couldnt create payload action from bytes! Err={:?}", err),
        }

        



    }

}
    */
/*

/// Possible types of message sent between client and server
#[repr(u16)]
pub enum EthosMessageType {
    None = 0,
    Action,
    Error = 65535,
}

impl EthosMessageType {
     pub const fn from_bytes(bytes: &[u8; 2]) -> Result<EthosMessageType, EthosCoreNetError>{ 
        match u16::from_le_bytes(*bytes) {
            0 => Ok(EthosMessageType::Action),
            65535 => Ok(EthosMessageType::Error),
            _ => Err(EthosCoreNetError::ECNetErrorInvalidNetMessageType)
        }
    }
}





#[derive(Debug)]
pub enum NetMessage {
    Error(EthosCoreNetError),
    /// Action : enum, character, value, value 
    Action(u16, u32, u32, u32),

}


impl EthosMessage {
    pub const fn from_bytes(bytes: &[u8; ETHOS_NET_MSG_SIZE]) -> Result<&EthosMessage, EthosCoreNetError>{ 

        //let aa = split_two(bytes);

        //todo!()
        unsafe {
            Ok(std::mem::transmute::<&[u8; ETHOS_NET_MSG_SIZE], &EthosMessage>(bytes))

        }
        /*
        let aa =  <&[u8; 2]>::try_from(&bytes[0..2]);
        match <&[u8; 2]>::try_from(&bytes[0..2]){
            Ok(msg_type) =>  match NetMessageType::from_bytes(msg_type){
                Ok(msg_type) => {
                    match msg_type {
                        NetMessageType::Action => Ok(NetMessage::Error(EthosCoreNetError::ECNetErrorInvalidNetMessage)),
                        NetMessageType::Error => Ok(NetMessage::Error(EthosCoreNetError::ECNetErrorInvalidNetMessage)),
                    }
                },
                Err(_) => Err(EthosCoreNetError::ECNetErrorInvalidNetMessageType),  // Invalid message type
            },
            Err(_) => Err(EthosCoreNetError::ECNetErrorInvalidNetMessage),  // Invalid message
        }
        */

    }

    pub fn to_bytes(&self) -> &[u8; ETHOS_NET_MSG_SIZE] {
        unsafe {
            std::mem::transmute::<&EthosMessage, &[u8; ETHOS_NET_MSG_SIZE]>(self)
        }
        
    }




}
*/