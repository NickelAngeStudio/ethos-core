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
#[doc(hidden)]
pub mod error;

#[doc(hidden)]
mod macros;

pub mod server;
pub mod client;

// Re-export
pub use error::Error as Error;

/// Discriminant type size of payload
const DISCRIMINANT_TYPE_SIZE : usize = size_of::<u16>();

/// Recommended buffer size (1mb) to read datas from net stream.
pub const READ_BUFFER_SIZE : usize = 1024*1024;


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