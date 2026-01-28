use std::vec;

use crate::error::{EthosError, EthosErrorDiscriminantSize};

macro_rules! write_payloads {
    ( $( $(#[$attr:meta])* $payload : ident $({ $($pname : ident : $ptype : ty),* })? = $value:expr),+ ) => {

        /// Union of possible message payload
        #[repr(u16)]
        #[derive(Debug)]
        pub enum EthosMessagePayload {
            $(
                $(
                    #[$attr]
                )*
                $payload $({
                    $(
                        $pname : $ptype
                    ),*
                })? = $value,
            )+
        }

        impl EthosMessagePayload {


        }



    };


}

write_payloads!{
    /// No payload
    None = 0,
    
    /// Action : enum, character, value, value
    Action {a : u16, b : u32, c : u32, d : u32 } = 1,

    /// An error message
    Error { err : MessageErrorSize } = 65535
}

/// Macro that read a buffer inline
macro_rules! read_buffer {
    ($t : ty, $bytes : expr, $start : expr) => {
        {
            let mut buf = [0 as u8; size_of::<$t>()];
            buf.copy_from_slice(&$bytes[ $start.. $start + size_of::<$t>()]);
            <$t>::from_le_bytes(buf)
        }

    };

}

/// Type of the discriminant that match each payload enum 
/// 
/// Note
/// Max 65535 enum, make u32 if not enough 
pub type PayloadDiscriminantType = u16;

 
/// Allow up to u32::MAX message of error
pub type MessageErrorSize = u32;

/*
/// Union of possible message payload
#[repr(u16)]
#[derive(Debug)]
pub enum EthosMessagePayload {
    None = 0,
    
    /// Action : enum, character, value, value 
    Action(u16, u32, u32, u32),

    Error(MessageErrorSize) = 65535,
} 

impl EthosMessagePayload {
    #[cfg(test)]
    pub fn create_empty(discriminant : PayloadDiscriminantType) -> EthosMessagePayload {
        match discriminant {
            1 => Self::Action(0, 0, 0, 0),
            65535 => Self::Error(0),
            _ => EthosMessagePayload::None,
        }
    }

    /// Returns a value uniquely identifying the enum variant
    /// 
    /// Ref
    /// https://doc.rust-lang.org/std/mem/fn.discriminant.html
    pub fn discriminant(&self) -> PayloadDiscriminantType {
        unsafe { *(self as *const Self as *const PayloadDiscriminantType)}
    }

    /// Pack message into a vector of bytes in little-endian byte order.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes  = vec![0 ; self.size_of_bytes()];

        // Write type to bytes
        bytes[0..2].copy_from_slice(&self.discriminant().to_le_bytes());

        // Pack payload to bytes
        match self {
            EthosMessagePayload::None => {},
            EthosMessagePayload::Action(a, b, c, d) => {
                bytes[2..4].copy_from_slice(&a.to_le_bytes());
                bytes[4..8].copy_from_slice(&b.to_le_bytes());
                bytes[8..12].copy_from_slice(&c.to_le_bytes());
                bytes[12..16].copy_from_slice(&d.to_le_bytes());
            },
            EthosMessagePayload::Error(a) => {
                bytes[2..6].copy_from_slice(&a.to_le_bytes());
            },
        }

        bytes
    }

    /// Get packed size of bytes
    pub fn size_of_bytes(&self) -> usize {
        EthosMessagePayload::size_of_bytes_from_discriminant(self.discriminant())
        /*
        size_of::<PayloadDiscriminantSize>() + 
        match self {
            EthosMessagePayload::None => 0,
            EthosMessagePayload::Action(_, _, _, _) => 
                size_of::<u16>() + size_of::<u32>() + size_of::<u32>() + size_of::<u32>(),
            EthosMessagePayload::Error(_) => size_of::<EthosError>(),
        }
        */
    }

    const fn size_of_bytes_from_discriminant(discriminant : PayloadDiscriminantType) -> usize {
        size_of::<PayloadDiscriminantType>() + 
        match discriminant {
            1 =>  size_of::<u16>() + size_of::<u32>() + size_of::<u32>() + size_of::<u32>(),
            65535 =>  size_of::<MessageErrorSize>(),
            _ => 0
        }
    }


    /// Read bytes to create a [EthosMessagePayload] from buffer and size
    /// 
    /// Returns 
    /// Ok(EthosMessagePayload) if succesfull
    /// Err(InvalidPayloadType) if payload type is unknown
    /// Err(InvalidPayloadSize) if payload size doesnt match given buffer size
    pub fn from_bytes(bytes : &[u8], size : usize) -> Result<EthosMessagePayload, EthosError> {

        // Read discriminant
        let discriminant = read_buffer!(PayloadDiscriminantType, bytes, 0); 

        // Validate size of buffer
            if EthosMessagePayload::size_of_bytes_from_discriminant(discriminant) == size {

                match discriminant {
                    0 => Ok(EthosMessagePayload::None),
                    1 => Ok(EthosMessagePayload::Action(
                        read_buffer!(u16, bytes, size_of::<PayloadDiscriminantType>()), 
                        read_buffer!(u32, bytes, size_of::<PayloadDiscriminantType>() + size_of::<u16>()), 
                        read_buffer!(u32, bytes, size_of::<PayloadDiscriminantType>() + size_of::<u16>() + size_of::<u32>()), 
                        read_buffer!(u32, bytes, size_of::<PayloadDiscriminantType>() + size_of::<u16>() + size_of::<u32>() + size_of::<u32>())
                    )), // TODO : Read bytes buffer
                    65535 => Ok(EthosMessagePayload::Error(
                        read_buffer!(MessageErrorSize, bytes, size_of::<PayloadDiscriminantType>())
                    )),
                    _ =>  Err(EthosError::InvalidPayloadType) // Payload discriminant is invalid
                }

            } else {
                Err(EthosError::InvalidPayloadSize) // Payload size doesn't match discriminant
            }

    }

    /*
    /// Returns a payload read from bytes
    //pub fn from_bytes(bytes : &[u8]) -> Result<EthosMessagePayload, EthosCoreNetError> {

    //}

    /// Write payload bytes to buffer
    /// 
    /// Returns size of bytes written
    //pub fn write_bytes(&self, buffer : &mut [u8]) -> usize {
      //  self.discriminant() as usize
    //}
    */
}

*/

/*
/// Shortcut function to read buffer
#[inline(always)]
const fn read_buffer<T : Default>(bytes : &[u8], start : usize) -> T {
    let mut buf = [0 as u8, 0];
    buf.copy_from_slice(&bytes[start..start + size_of::<T>()]);
    T::from_le_bytes(buf)
}
    */

#[cfg(test)]
mod tests {

    use std::{u16, u32};

    use crate::payload::EthosMessagePayload;


    #[test]
    fn test() {
        let payloads = [
            EthosMessagePayload::None,
            EthosMessagePayload::Action(1, 2, 3, 4),
            EthosMessagePayload::Action(5, 6, 7, 8),
            EthosMessagePayload::Action(111, 112, 113, 114),
            EthosMessagePayload::Action(u16::MAX /2, u32::MAX /2, u32::MAX /2, u32::MAX /2),
            EthosMessagePayload::Action(u16::MAX - 1, u32::MAX - 1, u32::MAX - 1, u32::MAX - 1),
            EthosMessagePayload::Action(u16::MAX, u32::MAX, u32::MAX, u32::MAX),
            EthosMessagePayload::None,
            EthosMessagePayload::Error(0),
            EthosMessagePayload::Error(1),
            EthosMessagePayload::Error(2),
            EthosMessagePayload::Error(3),
            EthosMessagePayload::Error(u32::MAX),
            EthosMessagePayload::None,

        ];
        for pl in payloads {
             println!("Payload={:?}, bytes={:?}", pl, pl.to_le_bytes());
        }

    }

    /// Create a payload from bytes and test parameters
    #[test]
    fn payload_action_from_bytes(){
        let bytes = [1 as u8, 0, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127];
        
        match EthosMessagePayload::from_bytes(&bytes, 16) {
            Ok(payload) => {
                match payload {
                    EthosMessagePayload::Action(a, b, c, d) => {
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