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

/// Encapsulates the server connection
pub struct Connection {
    stream: UnixStream,
    buffer: [u8; 1024],
}

impl Connection {

    /// finds the vt6 socket and connects to it, returning a new connection object
    pub fn new() -> Result<Connection, io::Error> {

        if let Some(vt6_socket_var) = vars().find(|var| var.0 == "VT6") {
            let con = Connection {
                stream: match UnixStream::connect(PathBuf::from(&vt6_socket_var.1)) {
                    Ok(stream) => stream,
                    Err(e) => { eprintln!("Cannot connect to socket '{}'", vt6_socket_var.1); panic!(e); },
                },
                buffer: [0; 1024],
            };
            return Ok(con);
        }

        Err(Error::new(ErrorKind::NotFound, "VT6 server connection not found."))
    }

    /// waits for a response (synchronously)
    pub fn send_and_receive(&mut self, msg: &str) -> (Message, usize) {
        loop {

            // send
            self.stream.write_all(msg.as_bytes()).unwrap(); // TODO: Use vt6 messages

            // read from stream...
            self.stream.read(&mut self.buffer).ok();

            // ...until there is something that can be parsed
            match Message::parse(&self.buffer) {
                // TODO: Why can't we do 'Ok(x) => return x,'???
                Ok(_) => break, // we cannot return immediately
                Err(_) => {}, // continue
            }
        }

        // return
        Message::parse(&self.buffer).unwrap()
    }
}
