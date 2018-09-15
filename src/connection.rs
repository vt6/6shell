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
use std::io::Write;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::exit;
use std::vec::Vec;

/// All input/output that is directed at the terminal has to go via this object.
/// It abstracts over the kind of connection (normal or multiplexed) and in
/// multiplexed mode joins message stream and I/O stream appropriately. In
/// normal mode it just forwards to stdout and the message stream.
pub struct IO {
    mode: ConnectionMode,
}

/// The mode the client is operating in and, if operating in normal mode,
/// the (connected) VT6 message stream
pub enum ConnectionMode {
    Normal(UnixStream),
    Multiplexed,
}

impl IO {

    /// Determines the connection mode and returns a new IO object
    pub fn new() -> Result<Connection, io::Error> {

        // use normal mode if VT6 environment variable is present
        if let Some(vt6_socket_var) = vars().find(|var| var.0 == "VT6") {
            return Ok(Connection { mode: ConnectionMode::Normal(
                // FIXME: Use SOCK_SEQPACKET to connect
                UnixStream::connect(PathBuf::from(vt6_socket_var.1))?
            )});
        }

        // use multiplexed mode if TERM environment variable is "vt6"
        if let Some(_) = vars().find(|var| *var == ("TERM".to_string(), "vt6".to_string())) {
            return Ok(Connection { mode: ConnectionMode::Multiplexed });
        }

        // else server is absent
        // for now this means exiting
        eprintln!("No VT6 server found. Exiting");
        exit(1);
    }

    pub fn is_multiplexed(&self) -> bool {
        self.mode == ConnectionMode::Multiplexed
    }

    /// Print to I/O stream
    pub fn print(&self, &str) {
        match self.mode {
            ConnectionMode::Normal(_) => print!(&str);
        }
    }

    /// Print to message stream. For the time being, this is a synchronous
    /// function, meaning that after sending it waits until a complete answer
    /// message is received.
    pub fn send_message(&mut self, msg: Vec<u8>) -> String {

        // send to the server
        match self.mode {

            // in normal mode write to message stream
            ConnectionMode::Normal(ref mut stream) => {
                stream.write_all(msg.as_slice()).unwrap();
            },

            // in multiplexed mode merge in I/O stream
            ConnectionMode::Multiplexed => {
                // TODO
                panic!("not yet implemented");
            },
        };

        // receive from the server
        match self.mode {

            // in normal mode read from message stream
            ConnectionMode::Normal(ref mut stream) => {
                use std::io::Read;
                let mut answer = String::with_capacity(24);
                let _bytes_read = stream.read_to_string(&mut answer).unwrap();
                return answer;
            },

            // in multiplexed mode parse from I/O stream
            ConnectionMode::Multiplexed => {
                // TODO
                panic!("not yet implemented");
            }
        };
    }
}
