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

#[doc(hidden)]
pub mod payload;
// Re-export
/// Payload of [Message] sent from client.
pub use payload::Payload as Payload;

/// Size of the message header before the payload
const MESSAGE_HEADER_SIZE : usize = 0;

/// Recommended buffer size for client [Message::pack_bytes].
pub const PACK_BUFFER_SIZE : usize = size_of::<Message>();

use crate::net::Error;
use tampon::Tampon;

/// Message sent from client to server.
/// 
/// # Client vs Server
/// Client message size are fixed and known by the server to prevent manipulation.
/// Client message don't have timestamp for now since it seem irrelevant.
/// 
/// # Note(s)
/// Client message is only a wrapper to it's payload for now.
#[derive(Debug, PartialEq)]
pub struct Message {

    /// Message content sent between client and server.
    pub payload : Payload,

}

impl Message {

    /// Create a new client `Message` from [Payload].
    /// 
    /// # Returns
    /// `Message` from created from payload.
    pub fn new(payload : Payload) -> Message {
        Message { payload }
    }

    /// Pack the [Message] in little-endian bytes in a given buffer.
    /// 
    /// Make sure to use a buffer with at least [PACK_BUFFER_SIZE] bytes.
    /// 
    /// # Returns
    /// Size of packed bytes as [usize].
    /// 
    /// # Panics
    /// Will panic if buffer is too small to contain the message.
    pub fn pack_bytes(&self, buffer : &mut [u8]) -> usize {
        tampon::serialize!(buffer, size, (self.payload):Payload);
        size
    }

    /// Extract a message from an array of bytes. 
    /// 
    /// # Returns
    /// [`Result`] which is:
    /// - [`Ok`]: [`Message`] properly extracted from bytes.
    /// - [`Err`]:
    ///     1. [`Error::InvalidMessage`] for malformed `Message`.
    ///     2. [`Error::InvalidBufferSize`] for buffer too short to read `Message` entirely.
    pub fn from_bytes(bytes : &[u8]) -> Result<Message, Error> {

        // Get discriminant
        tampon::deserialize!(bytes[MESSAGE_HEADER_SIZE..], (discriminant):u16);

        // Get needed size from discriminant
        let size = Payload::size_of_bytes_from_discriminant(discriminant);

        // Size received should be bigger than 0, else it's invalid.
        if size > 0 {
            let size = MESSAGE_HEADER_SIZE + size; // Add header size to total size
            if bytes.len() > size { // Make sure we can at least read the payload to prevent panic
                tampon::deserialize!(bytes, (payload):Payload);

                if let Payload::Invalid = payload { // Message received is invalid
                    Err(Error::InvalidMessage)
                } else {
                    Ok(Message { payload })
                }
            } else {
                Err(Error::InvalidBufferSize)
            }
        } else {    // Message received is invalid
            Err(Error::InvalidMessage)
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
    use std::u128;

    use crate::net::{Error, client::Message, client::PACK_BUFFER_SIZE, client::Payload};


    #[test]
    fn message_new(){
        // V1 : [Message::new] create a new [Message].
        Message::new(Payload::Key { key: u128::MAX / 2 });
    }

    #[test]
    fn message_pack_bytes(){
        // V2 : [Message::pack_bytes] pack the [Message] correctly with a buffer of size [super::PACK_BUFFER_SIZE].
        let mut buffer = [0u8; PACK_BUFFER_SIZE];
        let msg = Message::new(Payload::Key { key: u128::MAX / 2 });
        msg.pack_bytes(&mut buffer);
    }

    #[test]
    #[should_panic]
    fn message_pack_bytes_panic(){
        // V3 : [Message::pack_bytes] should [panic!] when buffer is too small.
        let mut buffer = [0u8; 2];
        let msg = Message::new(Payload::Key { key: u128::MAX / 2 });
        msg.pack_bytes(&mut buffer);
    }

    #[test]
    fn message_pack_bytes_from_bytes(){
        // V4 : [Message::pack_bytes] then [Message::from_bytes] should contains the same message.
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
        // V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for unknown message type.
        let bytes = [255u8,252,255,231,214,226,231,222,123,023,123,012];
        if let Err(Error::InvalidMessage) = Message::from_bytes(&bytes){
            // Correct behaviour
        } else {
            panic!(" V5 : [Message::from_bytes] must return [`Error::InvalidMessage`] for unknown message type.")
        }

    }

    #[test]
    fn message_from_bytes_invalid_buffer_size(){
        let mut bytes = [0u8; 2];
        // V6 : [Message::from_bytes] must return [`Error::InvalidBufferSize`] when reading a smaller buffer.
        let error_type = 0u16;
        tampon::serialize!(bytes, (error_type):u16);
        if let Err(Error::InvalidBufferSize) = Message::from_bytes(&bytes){
            // Correct behaviour
        } else {
            panic!("V6 : [Message::from_bytes] must return [`Error::InvalidBufferSize`] when reading a smaller buffer.")
        }
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