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

/// Encapsulates the message connection to the VT6 server
pub struct Connection {
    stream: UnixStream,
}

impl Connection {

    /// finds the vt6 socket and connects to it
    pub fn new() -> Result<Connection, io::Error> {

        if let Some(vt6_socket_var) = vars().find(|var| var.0 == "VT6") {
            return Ok(Connection {
                stream: UnixStream::connect(PathBuf::from(vt6_socket_var.1))?,
            });
        }

        Err(Error::new(ErrorKind::NotFound, "VT6 server connection not found."))
    }

    pub fn send(&mut self, msg: &str) {
        self.stream.write_all(msg.as_bytes()).unwrap();
    }

    pub fn read(&mut self) {
        use std::io::Read;
        let mut answer = String::with_capacity(24); // FIXME: Always read to message end, not into a buffer of fixed size
        let read_bytes = self.stream.read_to_string(&mut answer).unwrap();
        println!("Read {} bytes: {}", read_bytes, answer);
    }
}
