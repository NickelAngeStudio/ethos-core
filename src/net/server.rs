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


use tampon::Tampon;

use crate::write_messages_struct;

const SERVER_MAX_SIZE : usize = u16::MAX as usize;

write_messages_struct!{ SERVER_MSG_HEADER, SERVER_MSG_TAIL, SERVER_MAX_SIZE,
        /// Message sent from server to client.
        ServerMessage < ServerPayload >,
            /// Timestamp of the message in milliseconds when it happened on server.
            /// 
            /// - Use [Instant](std::time::Instant) and [Duration](std::time::Duration) value to fill.
            /// - DO NOT USE [std::time::SystemTime] since it is not monotonic.
            pub timestamp : u64

        
    }

// IMPORTANT : Add a unique u16 value to each new payload.
crate::write_messages_payloads!{
    /// Payload sent from server to client.
    /// 
    /// Payload are packed for smaller transfer size.
    ServerPayload,

     /// An error message sent by the server to the client.
    Error {
        /// Possible error index according to the server error chart. 
        err : u32 
    } = 65534,


    /// Invalid or malformed payload that are suspicious.
    /// 
    /// Since client to server communications are always handled by
    /// TCP, no loss or modification of data should have arised.
    Invalid = 65535
}