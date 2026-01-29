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


use crate::net::{ Error, Payload };
use tampon::Tampon;

/// Message sent between client and server.
/// 
/// Messages are the only form of communication used between client and server over TCP and UDP connection.
pub struct Message {

    /// Packed size of the message including payload.
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
    /// Make sure to use a buffer with at least [PACK_BUFFER_SIZE](super::PACK_BUFFER_SIZE) bytes.
    /// 
    /// # Panics
    /// Will panic if buffer is too small to contain the message.
    pub fn pack_bytes(&self, buffer : &mut [u8]) -> usize {
        tampon::serialize!(buffer, (self.size):u16, (self.timestamp):u64, (self.payload):Payload);
        self.size as usize
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

        tampon::deserialize!(bytes, (size):u16);

        // Make sure buffer is big enough to read to prevent panic.
        // It is better to drop malformed data than crashing the whole thing.
        if bytes.len() > size as usize {
            tampon::deserialize!(bytes[std::mem::size_of::<u16>()..], (timestamp):u64, (payload):Payload);

            if let Payload::Invalid = payload { // Message received is invalid
                Err(Error::InvalidMessage)
            } else {
                Ok(Message { size, timestamp, payload })
            }
        } else {
            Err(Error::InvalidBufferSize)
        }
        
    }

}


#[cfg(test)]
mod tests {
    //use crate::msg::NetMessage;


    #[test]
    fn test(){
        //let aa = NetMessage::from_bytes(bytes);
    }

}