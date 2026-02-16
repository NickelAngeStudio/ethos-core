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

//! Communications sent from server.
//! 
//! Server send a message [Message] containing a [Payload].
//! 
//! Server communications are handled differently than client since their size 
//! can vary and server won't tamper communications.

#[doc(hidden)]
pub mod payload;
// Re-export
pub use payload::Payload as Payload;

use tampon::Tampon;
use crate::net::Error;


/// Size of the message header before the payload
const MESSAGE_HEADER_SIZE : usize = size_of::<u16>() + size_of::<u64>();

/// Recommended buffer size for server [Message::pack_bytes].
pub const PACK_BUFFER_SIZE : usize = u16::MAX as usize;

/// Message sent from server to client.
/// 
/// Client message size are fixed and known by the server.
#[derive(Debug, PartialEq)]
pub struct Message {

    /// Packed size of the message in bytes including payload.
    pub size : u16,

    /// Timestamp of the message in milliseconds.
    /// 
    /// - Use [Instant](std::time::Instant) and [Duration](std::time::Duration) value to fill.
    /// - DO NOT USE [std::time::SystemTime] since it is not monotonic.
    pub timestamp : u64,

    /// Message content sent between client and server.
    pub payload : Payload,

}

impl Message {

    /// Create a new `Message` from timestamp and [Payload].
    /// 
    /// # Returns
    /// `Message` from timestamp and payload with size.
    pub fn new(timestamp : u64, payload : Payload) -> Message {
        Message { size: size_of::<u16>() as u16 + size_of::<u64>() as u16 + payload.bytes_size() as u16, timestamp, payload }
    }

    /// Pack the [Message] in little-endian bytes in a given buffer.
    /// 
    /// Make sure to use a buffer with at least [PACK_BUFFER_SIZE] bytes.
    /// 
    /// # Panics
    /// Will panic if buffer is too small to contain the message.
    pub fn pack_bytes(&self, buffer : &mut [u8]) -> usize {
        // Test buffer size to be PACK_BUFFER_SIZE to prevent panic!
        #[cfg(debug_assertions)]
        {
            if buffer.len() < PACK_BUFFER_SIZE {
                
                println!("Warning : Pack buffer size should at least be PACK_BUFFER_SIZE({})!", PACK_BUFFER_SIZE)
            }
        }

        tampon::serialize!(buffer, (self.size):u16, (self.timestamp):u64, (self.payload):Payload);
        self.size as usize
    }

    /// Extract a message from an array of bytes. 
    /// 
    /// # Returns
    /// [`Result`] which is:
    /// - [`Ok`]: [`Message`] properly extracted from bytes.
    /// - [`Err`]:
    ///     1. [`Error::IncompleteMessage`] when buffer is missing part of the message.
    pub fn from_bytes(bytes : &[u8]) -> Result<Message, Error> {

        // Verify if size can be read (buffer.len()>=type_of message.size)
        if bytes.len() >= size_of::<u16>() {
            // Read size
             tampon::deserialize!(bytes, (size):u16);

            if bytes.len() >= size as usize { // Make sure message is complete
                tampon::deserialize!(bytes, (timestamp):u64, (payload):Payload);
                Ok(Message { size : size as u16, timestamp, payload })
            } else {    // Message is incomplete
                Err(Error::IncompleteMessage)
            }

        } else {    // Message is incomplete
            Err(Error::IncompleteMessage)
        }
    }

}


/// This module test the [Message] struct.
/// 
/// # Verification(s)
/// V1 : [Message::new] create a new [Message].
/// V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE].
/// V3 : [Message::pack_bytes] should [panic!] when buffer is too small.
/// V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
/// V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for malformed message.
/// V6 : [Message::from_bytes] must return [`Error::InvalidBufferSize`] when reading a smaller buffer.
/// V7 : Stress test  [Message::pack_bytes] / [Message::from_bytes] 1024*1024 times. Should be faster than 1 secs.
#[cfg(test)]
mod tests_messages {
    use crate::net::{Error, server::Message, server::PACK_BUFFER_SIZE, server::Payload};


    #[test]
    fn message_new(){
        // V1 : [Message::new] create a new [Message].
        Message::new(0, Payload::Error { err: u32::MAX / 2 });
    }

    #[test]
    fn message_pack_bytes(){
        // V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE].
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        let msg = Message::new(0, Payload::Error { err: u32::MAX / 2 });
        msg.pack_bytes(&mut buffer);
    }

    #[test]
    #[should_panic]
    fn message_pack_bytes_panic(){
        // V3 : [Message::pack_bytes] should [panic!] when buffer is too small.
        let mut buffer = [0u8; 2];
        let msg = Message::new(0, Payload::Error { err: u32::MAX / 2 });
        msg.pack_bytes(&mut buffer);
    }

    #[test]
    fn message_pack_bytes_from_bytes(){
        // V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        let control = Message::new(u64::MAX / 2, Payload::Error { err: u32::MAX / 2 });
        control.pack_bytes(&mut buffer);
        match Message::from_bytes(&buffer){
            Ok(target) => assert_eq!(control, target, "V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
            Err(_) => panic!("V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message."),
        }

    }

    #[test]
    fn message_from_bytes_invalid_message(){
        // V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for malformed message.
        let bytes = [0, 5, 255u8,252,255,231,214,226,231,222,123,023,123,012];
        println!("MSG={:?}", Message::from_bytes(&bytes));
        if let Err(Error::InvalidMessage) = Message::from_bytes(&bytes){
            // Correct behaviour
        } else {
            panic!(" V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for malformed message.")
        }

    }

    #[test]
    fn message_from_bytes_invalid_buffer_size(){
        // V6 : [Message::from_bytes] must return [`Error::InvalidBufferSize`] when reading a smaller buffer.
        todo!()
    }

    #[test]
    fn message_stess(){
        // V7 : Stress test  [Message::pack_bytes] / [Message::from_bytes] 1024*1024 times. Should be faster than 1 secs.
        todo!()
    }

}