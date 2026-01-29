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
/// TODO: Generate serialize/deserialize test for each payload type.
#[macro_export]
macro_rules! write_payloads {
    ( $( $(#[$attr:meta])* $payload : ident $({ $($pname : ident : $ptype : ident),* })? = $value:expr),+ ) => {

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

        impl Tampon<EthosMessagePayload> for EthosMessagePayload {
            fn bytes_size(&self) -> usize {
                EthosMessagePayload::size_of_bytes_from_discriminant(self.discriminant())
            }

            fn serialize(&self, buffer : &mut [u8]) -> usize {

                // Pack payload to bytes
                match self {
                    $(
                        EthosMessagePayload::$payload $({
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

            fn deserialize(buffer : &[u8]) -> (EthosMessagePayload, usize) {
                
                // Read discriminant
                tampon::deserialize!(buffer, (discriminant):u16);

                match discriminant {
                    $(
                        $value => {
                            $(
                                tampon::deserialize!(buffer[size_of::<PayloadDiscriminantType>()..], 
                                    $(
                                        ($pname):$ptype
                                    ),* 
                                );
                            )?
                            

                            let pl = EthosMessagePayload::$payload $({
                                $(
                                    $pname
                                ),*
                            })?;
                            let bs = pl.bytes_size();
                            (pl, bs)

                        },
                    )+
                    _ =>  (EthosMessagePayload::Invalid, 0) // Invalid payload
                }


            }
        }

        impl EthosMessagePayload {
            /// Returns a value uniquely identifying the enum variant
            /// 
            /// Ref
            /// https://doc.rust-lang.org/std/mem/fn.discriminant.html
            pub fn discriminant(&self) -> PayloadDiscriminantType {
                unsafe { *(self as *const Self as *const PayloadDiscriminantType)}
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
        }
    };
}