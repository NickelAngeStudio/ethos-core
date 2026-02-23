
/// This macro generate message code since client and server share same code but with differents parameters.
///
///
/// # Note(s)
/// * Each message parameter must #[derive(PartialEq)] for tests purpose.
#[doc(hidden)]
#[macro_export]
macro_rules! write_messages_struct {

     ( $cons_header:ident, $cons_tail:ident, $max_size:expr, $(#[$comment:meta])* $struct_name:ident <$payload_type:ident> $(, $(#[$ex_attr:meta])* $ex_vis : vis $ex_pname : ident : $ex_ptype : ident )* ) => {
        
        // Size of the message header (size + discriminant)
        const $cons_header : usize = $crate::net::DISCRIMINANT_TYPE_SIZE + $crate::net::PAYLOAD_SIZE_TYPE_SIZE;
        // Size of tail
        const $cons_tail : usize = 0 $(+ size_of::<$ex_ptype>())*;
        

        $( #[$comment] )*
        #[derive(Debug, PartialEq)]
        pub struct $struct_name {

            /// Packed size of the message in bytes including payload and size itself.
            pub size : u16,

            /// Message content sent between client and server.
            pub payload : $payload_type,

            // Extra fields
            $(
                $(
                    #[$ex_attr]
                )*
                $ex_vis $ex_pname : $ex_ptype,
            )*

        }

        impl $struct_name {
            /// Create a new [`Message`](Self) from payload.
            /// 
            /// Size is automatically calculated.
            /// 
            /// # Returns
            ///  [`Message`](Self) created.
            pub fn new($($ex_pname  : $ex_ptype,)* payload : $payload_type ) -> $struct_name {
                let size : u16 = payload.bytes_size() as u16 + $crate::net::PAYLOAD_SIZE_TYPE_SIZE as u16 + $cons_tail as u16;
                $struct_name { size, $($ex_pname,)* payload  }
            }

            /// Pack the [`Message`](Self) in little-endian bytes in a given buffer.
            /// 
            /// # Important
            /// Make sure to use a buffer large enough to contain the packed message.
            /// 
            /// # Returns
            /// [`Result`] which is:
            /// - [`Ok`]: [`usize`] which represent size of bytes packed.
            ///     1. [`Error::BufferSizeTooSmall`](`crate::net::Error::BufferSizeTooSmall`) if buffer is too small to pack message.
            pub fn pack_bytes(&self, buffer : &mut [u8]) -> Result<usize, $crate::net::Error> {

                // Make sure buffer is big enough to pack
                if buffer.len() >= self.payload.bytes_size() + $crate::net::PAYLOAD_SIZE_TYPE_SIZE + $cons_tail {
                    tampon::serialize!(buffer, size, (self.size):u16, (self.payload):$payload_type $(,(self.$ex_pname):$ex_ptype)*);
                    Ok(size + $crate::net::PAYLOAD_SIZE_TYPE_SIZE)
                } else {
                    Err($crate::net::Error::BufferSizeTooSmall)
                }

            }


            /// Extract a [`Message`](Self) from an array of bytes. 
            /// 
            /// # Returns
            /// [`Result`] which is:
            /// - [`Ok`]: [`Message`](Self) properly extracted from bytes.
            /// - [`Err`]:
            ///     1. [`Error::InvalidMessage`](crate::net::Error::InvalidMessage) for malformed [`Message`](Self).
            ///     2. [`Error::IncompleteMessage`](crate::net::Error::IncompleteMessage) for buffer too short to read [`Message`](Self) entirely.
            ///     3. [`Error::MessageSizeInvalid`](crate::net::Error::MessageSizeInvalid) when given size doesn't match content size.
            ///     4. [`Error::MessageSizeGreaterThanLimit`](crate::net::Error::MessageSizeGreaterThanLimit) when size exceed limit.
            pub fn from_bytes(bytes : &[u8]) -> Result<$struct_name, $crate::net::Error> {

                // Make sure we can read size and discriminant
                if bytes.len() >= $cons_header {
                    // Read size and discriminant
                    tampon::deserialize!(bytes, (size_from_header):u16, (discriminant):u16);

                    // Validate discriminant
                    if <$payload_type>::is_valid(discriminant) {
                        // Get size of deserialization
                        match tampon::deserialize_size!(bytes[$cons_header..], $max_size, 
                            (payload):$payload_type $(,($ex_pname):$ex_ptype)*) {
                            Ok(size_from_ds) => {
                                // Make sure size given matches size of header
                                if size_from_header == size_from_ds as u16 + $crate::net::PAYLOAD_SIZE_TYPE_SIZE as u16 {
                                    // Deserialize and return massage
                                    tampon::deserialize!(bytes[$crate::net::PAYLOAD_SIZE_TYPE_SIZE..], (payload):$payload_type $(,($ex_pname):$ex_ptype)*);
                                    Ok($struct_name { size : size_from_header,  payload $(,$ex_pname)* })

                                } else {
                                    Err($crate::net::Error::MessageSizeInvalid)
                                }
                            },
                            Err(err) => match err {
                                tampon::TamponError::DeserializeSizeBufferIncomplete =>  Err($crate::net::Error::IncompleteMessage),
                                tampon::TamponError::DeserializeSizeGreaterThanMax => Err($crate::net::Error::MessageSizeGreaterThanLimit),
                            },
                        }
                    } else {
                        Err($crate::net::Error::InvalidMessage)
                    }
                } else {
                    Err($crate::net::Error::IncompleteMessage)
                }
            }
        }
     }
}



/// This module test the write_messages_struct! macro.
/// 
/// # Verification(s)
/// V1 : [Message::new] create a new [Message].
/// V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE] and returns Ok(size).
/// V3 : [Message::pack_bytes] returns [`Error::BufferSizeTooSmall`] when buffer is too small.
/// V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
/// V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for malformed message.
/// V6 : [Message::from_bytes] must return [`Error::IncompleteMessage`] for buffer too short to read `Message` entirely.
/// V7 : [Message::from_bytes] must return [`Error::MessageSizeInvalid`] when given size doesn't match content size.
/// V8 : [Message::from_bytes] must return [`Error::MessageSizeGreaterThanLimit`] when size exceed limit.
#[cfg(test)]
mod tests_messages {
    use std::{u8, u16, u32, u64, u128};
    use tampon::{Tampon, deserialize, deserialize_size, serialize};
    use crate::net::Error;

    const DISC_VAL : u16 = u16::MAX / 2 + 2;
    const P1_VAL : u8 = u8::MAX / 2;
    const P2_VAL : u16 = u16::MAX / 2;
    const P3_VAL : u32 = u32::MAX / 2;
    const P4_VAL : u64 = u64::MAX / 2;
    const P5_VAL : u128 = u128::MAX / 2;

    const SIZE_VAL : usize = size_of::<u16>() + size_of::<u8>() + size_of::<u16>() + size_of::<u32>() + size_of::<u64>() + size_of::<u128>();
    const SIZE_MSG2_VAL : usize = size_of::<u64>();
    const SIZE_MSG3_VAL : usize = size_of::<u64>() + size_of::<u8>() + size_of::<u16>() + size_of::<u32>() + size_of::<u64>() + size_of::<u128>();

    const PACK_BUFFER_SIZE : usize = SIZE_MSG3_VAL * 2;    

    fn generate_test_msgs() -> ( MessageTestNoExtra, MessageTestOneExtra, MessageTestMultiExtra) {
        (MessageTestNoExtra::new(PayloadTest::new()),
        MessageTestOneExtra::new(DISC_VAL as u64, PayloadTest::new()),
        MessageTestMultiExtra::new(DISC_VAL as u64, P1_VAL, P2_VAL, P3_VAL, P4_VAL, P5_VAL, PayloadTest::new()))
    }

    fn assert_payload(payload : &PayloadTest) {
        assert_eq!(payload.discriminant, DISC_VAL);
        assert_eq!(payload.p1, P1_VAL);
        assert_eq!(payload.p2, P2_VAL);
        assert_eq!(payload.p3, P3_VAL);
        assert_eq!(payload.p4, P4_VAL);
        assert_eq!(payload.p5, P5_VAL);
    }

    #[test]
    fn v1_message_new(){
        // V1 : [Message::new] create a new [Message].
        let (msg1, msg2, msg3) = generate_test_msgs();

        assert_eq!(msg1.size as usize,  SIZE_VAL + crate::net::DISCRIMINANT_TYPE_SIZE, "msg1 size invalid!");
        assert_eq!(msg2.size as usize,  SIZE_VAL + crate::net::DISCRIMINANT_TYPE_SIZE + SIZE_MSG2_VAL, "msg2 size invalid!");
        assert_eq!(msg3.size as usize,  SIZE_VAL + crate::net::DISCRIMINANT_TYPE_SIZE + SIZE_MSG3_VAL, "msg3 size invalid!");

        assert_payload(&msg1.payload);
        assert_payload(&msg2.payload);
        assert_payload(&msg3.payload);
        
        assert_eq!(msg2.timestamp, DISC_VAL as u64);
        assert_eq!(msg3.timestamp, DISC_VAL as u64);
        assert_eq!(msg3.ex1, P1_VAL);
        assert_eq!(msg3.ex2, P2_VAL);
        assert_eq!(msg3.ex3, P3_VAL);
        assert_eq!(msg3.ex4, P4_VAL);
        assert_eq!(msg3.ex5, P5_VAL);
    }

    #[test]
    fn v2_message_pack_bytes(){
        // V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE].
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        let (msg1, msg2, msg3) = generate_test_msgs();


        match msg1.pack_bytes(&mut buffer) {
            Ok(size) => assert_eq!(size, msg1.size as usize),
            Err(_) => panic!("msg1.pack_bytes() should not Err!"),
        }

        match msg2.pack_bytes(&mut buffer) {
            Ok(size) => assert_eq!(size, msg2.size as usize),
            Err(_) => panic!("msg2.pack_bytes() should not Err!"),
        }

        match msg3.pack_bytes(&mut buffer) {
            Ok(size) => assert_eq!(size, msg3.size as usize),
            Err(_) => panic!("msg3.pack_bytes() should not Err!"),
        }
    }

    #[test]
    fn v3_message_pack_bytes_buffer_too_small(){
        // V3 : [Message::pack_bytes] returns [`Error::BufferSizeTooSmall`] when buffer is too small.
        
        let mut buffer = [0u8; SIZE_VAL];
        let (msg1, msg2, msg3) = generate_test_msgs();

        match msg1.pack_bytes(&mut buffer) {
            Ok(_) => panic!("msg1 should return Err BufferSizeTooSmall"),
            Err(err) => assert_eq!(err,Error::BufferSizeTooSmall),
        }

        match msg2.pack_bytes(&mut buffer) {
            Ok(_) => panic!("msg2 should return Err BufferSizeTooSmall"),
            Err(err) => assert_eq!(err,Error::BufferSizeTooSmall),
        }

        match msg3.pack_bytes(&mut buffer) {
            Ok(_) => panic!("msg3 should return Err BufferSizeTooSmall"),
            Err(err) => assert_eq!(err,Error::BufferSizeTooSmall),
        }
    }


    #[test]
    fn v4_message_pack_bytes_from_bytes(){
        // V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        let (ctrl_msg1, ctrl_msg2, ctrl_msg3) = generate_test_msgs(); 

        match ctrl_msg1.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestNoExtra::from_bytes(&buffer) {
                    Ok(msg) => assert_eq!(ctrl_msg1, msg),
                    Err(err) => panic!("ctrl_msg1.from_bytes() should not Err({:?})!", err),
            },
            Err(err) => panic!("ctrl_msg1.pack_bytes() should not Err({:?})!", err),
        }

        match ctrl_msg2.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestOneExtra::from_bytes(&buffer) {
                    Ok(msg) => assert_eq!(ctrl_msg2, msg),
                    Err(err) => panic!("ctrl_msg2.from_bytes() should not Err({:?})!", err),
            },
            Err(err) => panic!("ctrl_msg2.from_bytes() should not Err({:?})!", err),
        }

        match ctrl_msg3.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestMultiExtra::from_bytes(&buffer) {
                    Ok(msg) => assert_eq!(ctrl_msg3, msg),
                    Err(err) => panic!("ctrl_msg3.from_bytes() should not Err({:?})!", err),
            },
            Err(err) => panic!("ctrl_msg3.from_bytes() should not Err({:?})!", err),
        }

    }

    #[test]
    fn v5_message_from_bytes_invalid_message(){

        let msg_invalid = MessageTestInvalid::new(DISC_VAL as u64, PayloadTestInvalid::new());
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        match msg_invalid.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestInvalid::from_bytes(&buffer) {
                    Ok(_) => panic!("msg_invalid.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::InvalidMessage),
            },
            Err(err) => panic!("msg_invalid.pack_bytes() should not Err({:?})!", err),
        }

    }

    #[test]
    fn v6_message_from_bytes_incomplete_message(){
        // V6 : [Message::from_bytes] must return [`Error::IncompleteMessage`] for buffer too short to read `Message` entirely.
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        let (ctrl_msg1, ctrl_msg2, ctrl_msg3) = generate_test_msgs(); 

        match ctrl_msg1.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestNoExtra::from_bytes(&buffer[..2]) {
                    Ok(_) => panic!("ctrl_msg1.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::IncompleteMessage),
            },
            Err(err) => panic!("ctrl_msg1.pack_bytes() should not Err({:?})!", err),
        }

        match ctrl_msg2.pack_bytes(&mut buffer) {
            Ok(size) => match MessageTestOneExtra::from_bytes(&buffer[..(size/2)]) {
                    Ok(_) => panic!("ctrl_msg2.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::IncompleteMessage),
            },
            Err(err) => panic!("ctrl_msg2.from_bytes() should not Err({:?})!", err),
        }

        match ctrl_msg3.pack_bytes(&mut buffer) {
            Ok(size) => match MessageTestMultiExtra::from_bytes(&buffer[..(size/2)]) {
                    Ok(_) => panic!("ctrl_msg3.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::IncompleteMessage),
            },
            Err(err) => panic!("ctrl_msg3.from_bytes() should not Err({:?})!", err),
        }
        
    }

    #[test]
    fn v7_message_from_bytes_size_invalid(){
        // V7: [Message::from_bytes] must return [`Error::MessageSizeInvalid`] when given size doesn't match content size.
        
        
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        let (ctrl_msg1, ctrl_msg2, ctrl_msg3) = generate_test_msgs(); 

        match ctrl_msg1.pack_bytes(&mut buffer) {
            Ok(size) => {
                let size = (size -1) as u16;
                // Override size
                serialize!(buffer, (size):u16);
                match MessageTestNoExtra::from_bytes(&buffer) {
                    Ok(_) => panic!("ctrl_msg1.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::MessageSizeInvalid),
            }},
            Err(err) => panic!("ctrl_msg1.pack_bytes() should not Err({:?})!", err),
        }

         match ctrl_msg2.pack_bytes(&mut buffer) {
            Ok(_) => {
                // Override size
                serialize!(buffer, (0):u16);
                match MessageTestOneExtra::from_bytes(&buffer) {
                    Ok(_) => panic!("ctrl_msg2.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::MessageSizeInvalid),
            }},
            Err(err) => panic!("ctrl_msg1.pack_bytes() should not Err({:?})!", err),
        }

         match ctrl_msg3.pack_bytes(&mut buffer) {
            Ok(size) => {
                // Override size
                let size = (size / 2) as u16;
                serialize!(buffer, (size):u16);
                match MessageTestMultiExtra::from_bytes(&buffer) {
                    Ok(_) => panic!("ctrl_msg3.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::MessageSizeInvalid),
            }},
            Err(err) => panic!("ctrl_msg1.pack_bytes() should not Err({:?})!", err),
        }

    
    }

    #[test]
    fn v8_message_from_bytes_greater_than_limit(){
        // V8 : [Message::from_bytes] must return [`Error::MessageSizeGreaterThanLimit`] when size exceed limit.
        
        let msg_small = MessageTestSmallMax::new(DISC_VAL as u64, PayloadTest::new());
        let mut buffer = [0u8; PACK_BUFFER_SIZE];

        match msg_small.pack_bytes(&mut buffer) {
            Ok(_) => match MessageTestSmallMax::from_bytes(&buffer) {
                    Ok(_) => panic!("msg_small.from_bytes() should not be Ok!"),
                    Err(err) => assert_eq!(err, Error::MessageSizeGreaterThanLimit),
            },
            Err(err) => panic!("msg_small.pack_bytes() should not Err({:?})!", err),
        }

    }

    #[derive(Debug, PartialEq)]
    pub struct  PayloadTest {
        discriminant : u16,
        p1 : u8, p2 : u16,  p3 : u32, p4 : u64, p5: u128
    }

    impl Tampon for PayloadTest {
        fn bytes_size(&self) -> usize {
            SIZE_VAL
        }
    
        fn deserialize_size(buffer : &[u8], max_size : usize) -> Result<usize, tampon::TamponError> {
            deserialize_size!(buffer, max_size,  (discriminant):u16, (p1):u8, (p2):u16, (p3):u32, (p4):u64, (p5):u128)
        }
    
        fn serialize(&self, buffer : &mut [u8]) -> usize {
            serialize!(buffer, bytes_written, (self.discriminant):u16, (self.p1):u8, (self.p2):u16, (self.p3):u32, (self.p4):u64, (self.p5):u128);
            bytes_written
        }
    
        fn deserialize(buffer : &[u8]) -> (Self, usize) where Self: Sized {
            deserialize!(buffer, bytes_read, (discriminant):u16, (p1):u8, (p2):u16, (p3):u32, (p4):u64, (p5):u128);
            (PayloadTest{ discriminant, p1, p2, p3, p4, p5 }, bytes_read)
        }
    }

    impl PayloadTest {
        pub fn new() -> PayloadTest {
            PayloadTest { discriminant: DISC_VAL , p1: P1_VAL, p2: P2_VAL, p3: P3_VAL, p4: P4_VAL, p5: P5_VAL }
        }
        pub fn is_valid(_disc : u16) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct  PayloadTestInvalid {
        discriminant : u16,
        p1 : u8, p2 : u16,  p3 : u32, p4 : u64, p5: u128
    }

    impl Tampon for PayloadTestInvalid {
        fn bytes_size(&self) -> usize {
            SIZE_VAL
        }
    
        fn deserialize_size(buffer : &[u8], max_size : usize) -> Result<usize, tampon::TamponError> {
            deserialize_size!(buffer, max_size,  (discriminant):u16, (p1):u8, (p2):u16, (p3):u32, (p4):u64, (p5):u128)
        }
    
        fn serialize(&self, buffer : &mut [u8]) -> usize {
            serialize!(buffer, bytes_written, (self.discriminant):u16, (self.p1):u8, (self.p2):u16, (self.p3):u32, (self.p4):u64, (self.p5):u128);
            bytes_written
        }
    
        fn deserialize(buffer : &[u8]) -> (Self, usize) where Self: Sized {
            deserialize!(buffer, bytes_read, (discriminant):u16, (p1):u8, (p2):u16, (p3):u32, (p4):u64, (p5):u128);
            (PayloadTestInvalid{ discriminant, p1, p2, p3, p4, p5 }, bytes_read)
        }
    }

    impl PayloadTestInvalid {
        pub fn new() -> PayloadTestInvalid {
            PayloadTestInvalid{ discriminant: DISC_VAL , p1: P1_VAL, p2: P2_VAL, p3: P3_VAL, p4: P4_VAL, p5: P5_VAL }
        }
        pub fn is_valid(_disc : u16) -> bool {
            false
        }
    }

    // No extra
    write_messages_struct!{ MSG1_HEADER, MSG1_TAIL, PACK_BUFFER_SIZE,
        MessageTestNoExtra < PayloadTest >
        
    }

    // One extra
    write_messages_struct!{ MSG2_HEADER, MSG2_TAIL, PACK_BUFFER_SIZE,
        MessageTestOneExtra < PayloadTest >,
            pub timestamp : u64

        
    }

    // Multiple extra
    write_messages_struct!{ MSG3_HEADER, MSG3_TAIL, PACK_BUFFER_SIZE,
        MessageTestMultiExtra < PayloadTest >,
            pub timestamp : u64,
            pub ex1:u8,
            pub ex2:u16,
            pub ex3 : u32,
            pub ex4 : u64,
            pub ex5 : u128
    }

    // One extra
    write_messages_struct!{ INVALID_MSG_HEADER, INVALID_MSG_TAIL, PACK_BUFFER_SIZE,
        MessageTestInvalid < PayloadTestInvalid >,
            pub timestamp : u64

        
    }

    // One extra
    write_messages_struct!{ SMALL_MSG_HEADER, SMALL_MSG_TAIL, 2,
        MessageTestSmallMax < PayloadTest >,
            pub timestamp : u64

        
    }

}