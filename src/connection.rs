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

/// Encapsulates the server connection
pub struct Connection {
    stream: UnixStream,
    buffer: Vec<u8>,
}

impl Connection {

    /// finds the vt6 socket and connects to it, returning a new connection object
    pub fn new() -> Result<Connection, io::Error> {

        if let Some(vt6_socket_var) = vars().find(|var| var.0 == "VT6") {
            return Ok(Connection {
                stream: UnixStream::connect(PathBuf::from(vt6_socket_var.1))?,
                buffer: vec![0;1024],
            });
        }

        Err(Error::new(ErrorKind::NotFound, "VT6 server connection not found."))
    }

    /// sends a message and waits for the response synchronously
    pub fn send_and_receive(&mut self, msg: &str) {

        // send
        // TODO: Use vt6 messages
        self.stream.write_all(msg.as_bytes()).unwrap();

        // receive
        use std::io::Read;
        let read_bytes: usize;
        let read_result: (Message, usize);
        loop {

            // read from stream
            match self.stream.read(self.buffer) {
                Ok(amount) => read_bytes = amount,
                Err(e) => panic!(e),
            }

            // try to parse and end reading as soon as parsing successful
            match Message::parse(&self.buffer) {
                Ok(result) => {
                    read_result = result;
                    break;
                },
                Err(_) => {}, // continue
            }
        }

        println!("Read {} bytes: {:?}", read_bytes, read_result.0);
    }
}
