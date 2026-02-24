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

use crate::{net::CLIENT_MSG_MAX_SIZE, write_messages_struct};

// Create client message structure.
write_messages_struct!{ CLIENT_MSG_MAX_SIZE,
    /// Message sent from client to server.
    /// 
    /// Client and server message uses different enumeration to prevent
    /// client from sending / broadcasting server messages.
    /// 
    /// [`ClientMessage`] doesn't contain any timestamp for smaller package since client time depends on lot of factors
    /// whilst server time is absolute.
    ClientMessage < ClientPayload >

}


// IMPORTANT : Add a unique u16 value to each new payload.
crate::write_messages_payloads!{ 
    /// Payload sent from client to server.
    /// 
    /// Payload are packed for smaller transfer size.
    ClientPayload,
    /// Key used to authenticate with server
    Key { 
        /// Secret key shared between client and server to establish connection. 
        /// 
        /// # Notes
        /// Server will ban client for 15m upon providing incorrect key. 
        key : u128 
    } = 0,


    /// Invalid or malformed payload that are suspicious.
    /// 
    /// Since client to server communications are always handled by
    /// TCP, no loss or modification of data should have arised.
    Invalid = 65535
}

/*
#[cfg(test)]
mod tests_payloads {
    use crate::net::{ClientMessage, ClientPayload, MESSAGE_SIZE_TYPE_SIZE, READ_BUFFER_SIZE};


    #[test]
    fn in_out() {
        let msg = ClientMessage::new(ClientPayload::Key { key: 123 });
        let mut msg_buffer =  &mut [0 as u8; READ_BUFFER_SIZE];
        let mut size_buffer =  &mut [0 as u8; MESSAGE_SIZE_TYPE_SIZE];

        match msg.pack_bytes(buffer){
            Ok(_size) => {
                match ClientMessage::from_bytes(&buffer[MESSAGE_SIZE_TYPE_SIZE..MESSAGE_SIZE_TYPE_SIZE+_size]){
                    Ok(msg) => println!("Work!"),
                    Err(err) => println!("Err={:?}", err),
                }
            },
            Err(_) => todo!(),
        }
    }

}
    */