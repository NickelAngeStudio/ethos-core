use std::vec;

use crate::error::{EthosError, EthosErrorDiscriminantSize};

/// Type of the discriminant that match each payload enum 
/// 
/// Note
/// Max 65535 enum, make u32 if not enough 
pub type PayloadDiscriminantType = u16;

 
/// Allow up to u32::MAX message of error
pub type MessageErrorSize = u32;


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
            /// Returns a value uniquely identifying the enum variant
            /// 
            /// Ref
            /// https://doc.rust-lang.org/std/mem/fn.discriminant.html
            pub fn discriminant(&self) -> PayloadDiscriminantType {
                unsafe { *(self as *const Self as *const PayloadDiscriminantType)}
            }

            /// Get packed size of bytes
            pub fn size_of_bytes(&self) -> usize {
                EthosMessagePayload::size_of_bytes_from_discriminant(self.discriminant())
            }

            /// Get the packed payload size from the discriminant
            const fn size_of_bytes_from_discriminant(discriminant : PayloadDiscriminantType) -> usize {
                size_of::<PayloadDiscriminantType>() + 
                match discriminant {
                     $(
                        $value => {
                            $(
                                $(
                                    size_of::<$ptype>() +
                                )*
                            )?
                            0
                        },
                    )+
                    _ => 0
                }
            }

            /// Pack message into a vector of bytes in little-endian byte order.
            #[allow(unused)]
            pub fn to_le_bytes(&self) -> Vec<u8> {
                let mut bytes  = vec![0 ; self.size_of_bytes()];
                let (mut start, mut end) : (usize, usize) = (0, 0);

                // Write type to bytes
                end += size_of::<PayloadDiscriminantType>();
                bytes[start..end].copy_from_slice(&self.discriminant().to_le_bytes());
                start = end;

                // Pack payload to bytes
                match self {
                    $(
                        EthosMessagePayload::$payload $({
                            $(
                                $pname
                            ),*
                        })? => {
                            $($(
                                end += size_of::<$ptype>();
                                bytes[start..end].copy_from_slice(&$pname.to_le_bytes());
                                start = end;
                            )*)?
                        },
                    )+
                }

                bytes
            }

            /// Read bytes to create a [EthosMessagePayload] from buffer and size
            /// 
            /// Returns 
            /// Ok(EthosMessagePayload) if succesfull
            /// Err(InvalidPayloadType) if payload type is unknown
            /// Err(InvalidPayloadSize) if payload size doesnt match given buffer size
            pub fn from_bytes(bytes : &[u8], size : usize) -> Result<EthosMessagePayload, EthosError> {

                // Start of index read
                let mut start : usize = 0;

                // Read discriminant
                let discriminant = read_buffer!(PayloadDiscriminantType, bytes, start); 
                start += size_of::<PayloadDiscriminantType>();

                // Validate size of buffer
                    if EthosMessagePayload::size_of_bytes_from_discriminant(discriminant) == size {

                        match discriminant {
                            $(
                                $value => {
                                    $(
                                        $(
                                            let $pname = read_buffer!($ptype, bytes, start);
                                            start += size_of::<$ptype>();
                                        )* 
                                    )?

                                    Ok(EthosMessagePayload::$payload $({
                                        $(
                                            $pname
                                        ),*
                                    })?)
                                },
                            )+
                            _ =>  Err(EthosError::InvalidPayloadType) // Payload discriminant is invalid
                        }

                    } else {
                        Err(EthosError::InvalidPayloadSize) // Payload size doesn't match discriminant
                    }

            }

            



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


#[cfg(test)]
mod tests {

    use std::{u16, u32};

    use crate::payload::EthosMessagePayload;


    #[test]
    fn test() {
        let payloads = [
            EthosMessagePayload::None,
            EthosMessagePayload::Action { a: 0, b: 0, c: 0, d: 0 },
            EthosMessagePayload::Action { a: 5, b: 6, c: 7, d: 8},
            EthosMessagePayload::Action { a: 111, b: 112, c: 113, d: 114 },
            EthosMessagePayload::Action { a: u16::MAX /2, b: u32::MAX /2, c: u32::MAX /2, d: u32::MAX /2 },
            EthosMessagePayload::Action { a: u16::MAX - 1, b: u32::MAX -1, c: u32::MAX -1, d: u32::MAX -1 },
            EthosMessagePayload::Action { a: u16::MAX, b: u32::MAX, c: u32::MAX, d: u32::MAX },
            EthosMessagePayload::None,
            EthosMessagePayload::Error { err:0  },
            EthosMessagePayload::Error{ err:1  },
            EthosMessagePayload::Error{ err:2  },
            EthosMessagePayload::Error{ err:3  },
            EthosMessagePayload::Error{ err:u32::MAX  },
            EthosMessagePayload::None,

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
        
        match EthosMessagePayload::from_bytes(&bytes, 16) {
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