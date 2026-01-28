
/// Macro that read a buffer inline
#[macro_export]
macro_rules! read_buffer {
    ($t : ty, $bytes : expr, $start : expr) => {
        {
            let mut buf = [0 as u8; size_of::<$t>()];
            buf.copy_from_slice(&$bytes[ $start.. $start + size_of::<$t>()]);
            <$t>::from_le_bytes(buf)
        }

    };

}

/// This macro generate payloads code for bytes serialization. This help adding new payload quickly.
/// 
/// TODO: Write tests generation
#[macro_export]
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
            #[allow(unused)]
            pub fn from_bytes(bytes : &[u8]) -> Result<EthosMessagePayload, EthosError> {

                // Start of index read
                let mut start : usize = 0;

                // Read discriminant
                let discriminant = crate::read_buffer!(PayloadDiscriminantType, bytes, start); 
                start += size_of::<PayloadDiscriminantType>();

                // Validate size of buffer
                if EthosMessagePayload::size_of_bytes_from_discriminant(discriminant) == bytes.len() {

                    match discriminant {
                        $(
                            $value => {
                                $(
                                    $(
                                        let $pname = crate::read_buffer!($ptype, bytes, start);
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
