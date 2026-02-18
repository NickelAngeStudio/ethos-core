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

//! Communications sent from client.
//! 
//! Client send a message [Message] containing a [Payload].
//! 
//! Client communications are handled differently than server since their size are fixed and 
//! known by the server. This prevent oversized tampered client communications.

use crate::net::{CLIENT_MSG_MAX_SIZE, DISCRIMINANT_TYPE_SIZE, Error, PAYLOAD_SIZE_TYPE_SIZE};
use tampon::Tampon;

#[doc(hidden)]
pub mod payload;
// Re-export
/// Payload of [Message] sent from client.
pub use payload::Payload as Payload;

/// Recommended buffer size for client [Message::pack_bytes].
pub const PACK_BUFFER_SIZE : usize = CLIENT_MSG_MAX_SIZE;

/// Size of the message header (size + discriminant)
const MESSAGE_HEADER_SIZE : usize = DISCRIMINANT_TYPE_SIZE + PAYLOAD_SIZE_TYPE_SIZE;

/// Message sent from client to server.
/// 
/// Client and server message uses different enumeration to prevent
/// client from sending server messages.
/// 
/// # Client vs Server
/// * Client message don't have timestamp for now since it seem irrelevant.
#[derive(Debug, PartialEq)]
pub struct Message {

    /// Packed size of the message in bytes including payload.
    pub size : u16,

    /// Message content sent between client and server.
    pub payload : Payload,

}

impl Message {

    /// Create a new client `Message` from [Payload].
    /// 
    /// # Returns
    /// `Message` from created from payload.
    pub fn new(payload : Payload) -> Message {
        let size : u16 = payload.bytes_size() as u16 + PAYLOAD_SIZE_TYPE_SIZE as u16;
        Message { size, payload }
    }

    /// Pack the [Message] in little-endian bytes in a given buffer.
    /// 
    /// Make sure to use a buffer with at least [PACK_BUFFER_SIZE] bytes.
    /// 
    /// # Returns
    /// [`Result`] which is:
    /// - [`Ok`]: [`usize`] which represent size of bytes packed.
    ///     1. [`Error::BufferSizeTooSmall`] if buffer is too small to pack message.
    pub fn pack_bytes(&self, buffer : &mut [u8]) -> Result<usize, Error> {

        // Make sure buffer is big enough to pack
        if buffer.len() >= self.payload.bytes_size() + PAYLOAD_SIZE_TYPE_SIZE {
            tampon::serialize!(buffer, size, (self.payload):Payload);
            Ok(size + PAYLOAD_SIZE_TYPE_SIZE)
        } else {
            Err(Error::BufferSizeTooSmall)
        }

    }

    /// Constant function that read the message size from buffer
    /// 
    /// # Returns
    ///  [`Result`] which is:
    /// - [`Ok`]: [`usize`] which represent size from bytes.
    /// - [`Err`]:
    ///     1. [`Error::IncompleteMessage`] if buffer is too small to read the size.
    ///     2. [`Error::MessageSizeGreaterThanLimit`] if size > limit
    pub fn size_from_bytes(bytes : &[u8]) -> Result<usize, Error> {
        
         // Make sure we can read size
        if bytes.len() >= PAYLOAD_SIZE_TYPE_SIZE {
            // Read size
            tampon::deserialize!(bytes, (size_from_header):u16);

            if size_from_header <= CLIENT_MSG_MAX_SIZE as u16 {
                Ok(size_from_header as usize)
            } else {
                Err(Error::MessageSizeGreaterThanLimit)
            }
        } else {
            Err(Error::IncompleteMessage)
        }

    }

    /// Extract a message from an array of bytes. 
    /// 
    /// # Returns
    /// [`Result`] which is:
    /// - [`Ok`]: [`Message`] properly extracted from bytes.
    /// - [`Err`]:
    ///     1. [`Error::InvalidMessage`] for malformed `Message`.
    ///     2. [`Error::IncompleteMessage`] for buffer too short to read `Message` entirely.
    ///     3. [`Error::MessageSizeInvalid`] when given size doesn't match content size.
    ///     4. [`Error::MessageSizeGreaterThanLimit`] when size exceed limit.
    pub fn from_bytes(bytes : &[u8]) -> Result<Message, Error> {

        // Make sure we can read size and discriminant
        if bytes.len() >= MESSAGE_HEADER_SIZE {
            // Read size and discriminant
            tampon::deserialize!(bytes, (size_from_header):u16, (discriminant):u16);

            // Validate discriminant
            if Payload::is_valid(discriminant) {
                // Get size of deserialization
                match tampon::deserialize_size!(bytes[DISCRIMINANT_TYPE_SIZE..], CLIENT_MSG_MAX_SIZE, (payload):Payload) {
                    Ok(size_from_ds) => {
                        // Make sure size given matches size of header
                        if size_from_header == size_from_ds as u16 + PAYLOAD_SIZE_TYPE_SIZE as u16 {

                            // Deserialize and return massage
                            tampon::deserialize!(bytes, (payload):Payload);
                            Ok(Message { size : size_from_header,  payload })

                        } else {
                            Err(Error::MessageSizeInvalid)
                        }
                    },
                    Err(err) => match err {
                        tampon::TamponError::DeserializeSizeBufferIncomplete =>  Err(Error::IncompleteMessage),
                        tampon::TamponError::DeserializeSizeGreaterThanMax => Err(Error::MessageSizeGreaterThanLimit),
                    },
                }
            } else {
                Err(Error::InvalidMessage)
            }
        } else {
            Err(Error::IncompleteMessage)
        }
       

    }

}


/// This module test the [Message] struct.
/// 
/// # Verification(s)
/// V1 : [Message::new] create a new [Message].
/// V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE] and returns Ok(size).
/// V3 : [Message::pack_bytes] returns [`Error::BufferSizeTooSmall`] when buffer is too small.
/// V4 : [Message::size_from_bytes] returns Ok(usize) from correctly formed message.
/// V5 : [Message::size_from_bytes] returns [`Error::IncompleteMessage`] from incomplete message.
/// V6 : [Message::size_from_bytes] returns [`Error::MessageSizeGreaterThanLimit`] if read size is bigger than limit.
/// V7 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
/// V8 : [Message::from_bytes] must return [`Error::InvalidMessage`] for malformed message.
/// V9 : [Message::from_bytes] must return [`Error::IncompleteMessage`] for buffer too short to read `Message` entirely.
/// V10 : [Message::from_bytes] must return [`Error::MessageSizeInvalid`] when given size doesn't match content size.
/// V11 : [Message::from_bytes] must return [`Error::MessageSizeGreaterThanLimit`] when size exceed limit.
/// V12 : Stress test  [Message::pack_bytes] / [Message::from_bytes] 1024*1024 times. Should be faster than 1 secs.
#[cfg(test)]
mod tests_messages {
    use std::{u8, u16, u32, u64, u128};

    use crate::net::{Error, client::Message, client::PACK_BUFFER_SIZE, client::Payload};

    const P1_VAL : u8 = u8::MAX / 2;
    const P2_VAL : u16 = u16::MAX / 2;
    const P3_VAL : u32 = u32::MAX / 2;
    const P4_VAL : u64 = u64::MAX / 2;
    const P5_VAL : u128 = u128::MAX / 2;



    fn create_test_message() -> Message {
        Message::new(Payload::Test { p1: P1_VAL, p2: P2_VAL, p3: P3_VAL, p4: P4_VAL, p5: P5_VAL })
    }

    #[test]
    fn message_new(){
        // V1 : [Message::new] create a new [Message].
        let msg = create_test_message();

        match msg.payload {
            Payload::Test { p1, p2, p3, p4, p5 } => 
                assert!(p1 == P1_VAL && p2 == P2_VAL && p3 == P3_VAL && p4 == P4_VAL && p5 == P5_VAL)
            ,
            _ => panic!("Wrong payload for test!"),
        }
    }

    #[test]
    fn message_pack_bytes(){
        // V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE].
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        let msg = create_test_message();
        msg.pack_bytes(&mut buffer);
    }

    #[test]
    fn message_pack_bytes_buffer_too_small(){
        // V3 : [Message::pack_bytes] returns [`Error::BufferSizeTooSmall`] when buffer is too small.
        
        todo!()
        //let mut buffer = [0u8; 2];
        //let msg = Message::new(Payload::Key { key: u128::MAX / 2 });
        //msg.pack_bytes(&mut buffer);
    }

    #[test]
    fn message_size_from_bytes_ok(){
        // V4 : [Message::size_from_bytes] returns Ok(usize) from correctly formed message.
        todo!()
    }

    #[test]
    fn message_size_from_bytes_incomplete(){
        // V5 : [Message::size_from_bytes] returns [`Error::IncompleteMessage`] from incomplete message.
        todo!()
    }

    #[test]
    fn message_size_from_bytes_greater_than_limit(){
        // V6 : [Message::size_from_bytes] returns [`Error::MessageSizeGreaterThanLimit`] if read size is bigger than limit.
        todo!()
    }

    #[test]
    fn message_pack_bytes_from_bytes(){
        // V7 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        let control = Message::new(Payload::Key { key: u128::MAX / 2 });
        control.pack_bytes(&mut buffer);
        match Message::from_bytes(&buffer){
            Ok(target) => assert_eq!(control, target, "V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
            Err(_) => panic!("V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
        }

    }

    #[test]
    fn message_from_bytes_invalid_message(){
        // V8 : [Message::from_bytes] must return [`Error::InvalidMessage`] for unknown message type.
        let bytes = [255u8,252,255,231,214,226,231,222,123,023,123,012];
        if let Err(Error::InvalidMessage) = Message::from_bytes(&bytes){
            // Correct behaviour
        } else {
            panic!(" V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for unknown message type.")
        }

    }

    #[test]
    fn message_from_bytes_incomplete_message(){
        // V9 : [Message::from_bytes] must return [`Error::IncompleteMessage`] for buffer too short to read `Message` entirely.
        todo!()
        
        /*
        let mut bytes = [0u8; 2];
       
        let error_type = 0u16;
        tampon::serialize!(bytes, (error_type):u16);
        if let Err(Error::BufferSizeTooSmall) = Message::from_bytes(&bytes){
            // Correct behaviour
        } else {
            panic!("V6 : [Message::from_bytes] must return [`Error::InvalidBufferSize`] when reading a smaller buffer.")
        }
        */
    }

    #[test]
    fn message_from_bytes_size_invalid(){
        // V10 : [Message::from_bytes] must return [`Error::MessageSizeInvalid`] when given size doesn't match content size.
        todo!()
    }

    #[test]
    fn message_from_bytes_greater_than_limit(){
        // V11 : [Message::from_bytes] must return [`Error::MessageSizeGreaterThanLimit`] when size exceed limit.
        todo!()
    }

    #[test]
    #[ignore = "Manual run for stress test."]
    fn message_stess(){
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        
        // V7 : Stress test  [Message::pack_bytes] / [Message::from_bytes] 1024*1024 times. Should be faster than 1 secs.
        for i in 0..1024*1024 {
            let control = Message::new(Payload::Key { key: i as u128 });
            control.pack_bytes(&mut buffer);
            match Message::from_bytes(&buffer){
                Ok(target) => assert_eq!(control, target, "V7 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
                Err(_) => panic!("V7 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
            }
        }
    }

}