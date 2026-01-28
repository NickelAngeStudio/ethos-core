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

/// Macros
mod macros;

/// Messages
pub mod msg;

/// Payload of message
pub mod payload;

/// Errors
pub mod error;

/// Size of a messagetype in bytes
//pub const ETHOS_NET_MSG_TYPE_SIZE : usize = std::mem::size_of::<NetMessageType>();

/// Size of a message in bytes
//pub const ETHOS_NET_MSG_SIZE : usize = std::mem::size_of::<NetMessage>(); // ETHOS_NET_MSG_TYPE_SIZE + std::mem::size_of::<NetMessage>();

/// Recommended buffer size to read datas (1mb)
pub const ETHOS_NET_BUFFER_SIZE : usize = 1024*1024;

/// Ethos TCP port is 3847 ('eths' on a phone keyboard)
pub const ETHOS_TCP_PORT : u16 = 3847;

/// Ethos UDP port is 38467 ('ethos' on a phone keyboard)
pub const ETHOS_UDP_PORT : u16 = 38467;