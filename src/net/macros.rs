/* 
Copyright (c) 2026  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/ethos-core

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


/// This macro generate payloads code for bytes serialization. This help adding new payload quickly.
/// 
/// # Note(s)
/// Each payload parameter must implement trait [std::default::Default] and #[derive(PartialEq)] for tests purpose.
#[doc(hidden)]
#[macro_export]
macro_rules! write_messages_payloads {
    ( $( $(#[$attr:meta])* $payload : ident $({ $( $(#[$attr_field:meta])* $pname : ident : $ptype : ident),* })? = $value:expr),+ ) => {

        /// 
        /// 
        /// Payload are meant to be packed when sent and received.
        #[repr(u16)]
        #[derive(Debug, PartialEq)]
        pub enum Payload {
            $(
                $(
                    #[$attr]
                )*
                $payload $({
                    $(
                         $(
                            #[$attr_field]
                        )*
                        $pname : $ptype
                    ),*
                })? = $value,
            )+
        }

        impl Tampon<Payload> for Payload {
            fn bytes_size(&self) -> usize {
                Payload::size_of_bytes_from_discriminant(self.discriminant())
            }

            fn serialize(&self, buffer : &mut [u8]) -> usize {

                // Pack payload to bytes
                match self {
                    $(
                        Payload::$payload $({
                            $(
                                $pname
                            ),*
                        })? => {
                            tampon::serialize!(buffer, size_written, (self.discriminant()):u16
                            $($(
                                ,(*$pname):$ptype
                            )*)?);
                            size_written
                        },
                    )+
                }

            }

            fn deserialize(buffer : &[u8]) -> (Payload, usize) {
                
                // Read discriminant
                tampon::deserialize!(buffer, (discriminant):u16);

                match discriminant {
                    $(
                        $value => {
                            $(
                                tampon::deserialize!(buffer[size_of::<u16>()..], 
                                    $(
                                        ($pname):$ptype
                                    ),* 
                                );
                            )?
                            

                            let pl = Payload::$payload $({
                                $(
                                    $pname
                                ),*
                            })?;
                            let bs = pl.bytes_size();
                            (pl, bs)

                        },
                    )+
                    _ =>  (Payload::Invalid, 0) // Invalid payload
                }


            }
        }

        impl Payload {
            /// Returns a value uniquely identifying the enum variant
            /// 
            /// # See also
            /// *<https://doc.rust-lang.org/std/mem/fn.discriminant.html>*
            pub fn discriminant(&self) -> u16 {
                unsafe { *(self as *const Self as *const u16)}
            }

            
            /// Get the packed payload size from the discriminant
            pub(crate) const fn size_of_bytes_from_discriminant(discriminant : u16) -> usize {
                size_of::<u16>() + 
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
        }

        /// This module include tests for each [Payload] enum.
        /// 
        /// # Verification(s)
        /// V1 : [Payload] can be created with default values
        /// V2 : [Payload::serialize] buffer can write in adequate buffer.
        /// V3 : [Payload::serialize] size given must equal [Payload::bytes_size]
        /// V4 : [Payload::deserialize] give back the original payload.
        /// V5 : [Payload::deserialize] size given must equal [Payload::bytes_size].
        /// V6 : [Payload::deserialize] should give [Payload::Invalid] for invalid message.
        /// V7 : [Payload::deserialize] should give size of 0 for invalid message.
        #[cfg(test)]
        mod tests {
            use tampon::Tampon;
            use super::Payload;

            $(
                concat_idents::concat_idents!(test_name = payload_, $payload {
                    #[test]
                    #[allow(non_snake_case)]
                    fn test_name() {
                        // V1 : [Payload] can be created with default values
                        let payload = super::Payload::$payload $({
                            $(
                                $pname : $ptype::default()
                            ),*
                        })?;

                        // V2 : [Payload::serialize] buffer write.
                        let mut buffer = [0u8; size_of::<Payload>()];    // Create big enough buffer
                        let size = payload.serialize(&mut buffer);

                        // V3 : [Payload::serialize] size given must equal [Payload::bytes_size]
                        assert_eq!(size, payload.bytes_size());

                        // V4 : [Payload::deserialize] give back the original payload.
                        let (deserialized, size) = Payload::deserialize(&buffer);
                        assert_eq!(payload, deserialized);

                        // V5 : [Payload::deserialize] size given must equal [Payload::bytes_size].
                        assert_eq!(size, payload.bytes_size());
                    }
                });
            )+


            #[test]
            fn payload_deserialize_invalid(){
                // V6 : [Payload::deserialize] should give [Payload::Invalid] for invalids message.
                let buffer = [255u8, 254, 123, 254, 255, 124];    // Create unknown type buffer
                let (deserialized, size) = Payload::deserialize(&buffer);
                assert_eq!(deserialized, Payload::Invalid, "Wrong deserialized payload type should be Invalid");

                // V7 : [Payload::deserialize] should give size of [u16] for invalid message.
                assert_eq!(size, 0);
            }

        }
    };
}