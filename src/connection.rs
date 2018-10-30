/*******************************************************************************
 *
 * Copyright 2018 lærling <laerling@posteo.de>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <http://www.gnu.org/licenses/>.
 *
 *******************************************************************************/

use std::env::vars;
use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::io::Error;
use std::io::ErrorKind;
use vt6::common::core::msg::Message;
use std::io::Read;

const BUF_SIZE: usize = 1024;

/// Encapsulates the server connection
pub struct Connection {
    stream: UnixStream,
    buffer: [u8; BUF_SIZE],
}

impl Connection {

    /// finds the vt6 socket and connects to it, returning a new connection object
    pub fn new() -> Result<Connection, io::Error> {

        if let Some(vt6_socket_var) = vars().find(|var| var.0 == "VT6") {
            let con = Connection {
                stream: match UnixStream::connect(PathBuf::from(&vt6_socket_var.1)) {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprintln!("Cannot connect to socket '{}'", vt6_socket_var.1);
                        panic!(e); // TODO: Only in debug profile, else exit with 1
                    },
                },
                buffer: [0; BUF_SIZE],
            };
            return Ok(con);
        }

        Err(Error::new(ErrorKind::NotFound, "VT6 server socket not found."))
    }

    /// sends a message and waits for the response (synchronously)
    pub fn send_and_receive(&mut self, msg: &str) -> (Message, usize) {

        // send
        self.stream.write_all(msg.as_bytes()).unwrap(); // TODO: Use vt6 messages

        // read until there is something that can be parsed
        let mut buffer_offset: usize = 0;
        loop {

            // read into buffer...
            let bytes_read = match self.stream.read(&mut self.buffer[buffer_offset..]) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Connection to server socket lost");
                    panic!(e); // TODO: Only in debug profile, else exit with 1
                }
            };

            // adjust offset for next read
            buffer_offset += bytes_read;

            // ...until there is something that can be parsed
            match Message::parse(&self.buffer[..buffer_offset]) {
                Ok(_) => break,
                Err(_) => {}, // continue
            }
        }

        return Message::parse(&self.buffer[..buffer_offset]).unwrap();
    }
}
