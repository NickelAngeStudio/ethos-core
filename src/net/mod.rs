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

//! Communication between client and server.
//! 
//! Communication is achieved by sending and receiving [Message] containing a [Payload].

#[doc(hidden)]
pub mod error;

#[doc(hidden)]
mod macros;

#[doc(hidden)]
pub mod payload;

#[doc(hidden)]
pub mod msg;

// Re-export
pub use msg::Message as Message;
pub use payload::Payload as Payload;
pub use error::Error as Error;

/// Recommended buffer size (1mb) to read datas from net stream.
pub const READ_BUFFER_SIZE : usize = 1024*1024;

/// Recommended buffer size for [Message::pack_bytes].
pub const PACK_BUFFER_SIZE : usize = size_of::<Message>();

/// Ethos TCP port 3847.
/// 
/// Note
/// ('eths' on a phone keyboard)
pub const TCP_PORT : u16 = 3847;

/// Ethos UDP port 38467.
/// 
/// Note
/// ('ethos' on a phone keyboard)
pub const UDP_PORT : u16 = 38467;