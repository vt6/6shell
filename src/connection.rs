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
use std::process::exit;
use vt6::core::msg::Message;

/// Encapsulates all connections to the VT6 server, that is the I/O stream (most
/// commonly stdin and stdout) and the VT6 message stream (message input,
/// message output) and abstracts over the kind of connection (normal mode or
/// multiplexed mode)
pub struct Connection {
    mode: ConnectionMode,
}

/// The mode the client is operating in and, if operating in normal mode,
/// the (connected) VT6 message stream
pub enum ConnectionMode {
    Normal(UnixStream),
    Multiplexed,
}

impl Connection {

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

    // FIXME: msg: Message instead of &str
    pub fn send_and_receive(&mut self, msg: &str) -> String {

        // send
        match self.mode {

            // in normal mode write to VT6 message stream
            ConnectionMode::Normal(ref mut stream) => {
                // FIXME
                stream.write_all(msg.as_bytes()).unwrap();
            },

            // in multiplexed mode write to stdout
            ConnectionMode::Multiplexed => {
                // TODO
            },
        };

        // read
        match self.mode {

            // in normal mode read from VT6 message stream
            ConnectionMode::Normal(ref mut stream) => {
                use std::io::Read;
                let mut answer = String::with_capacity(24);
                let _bytes_read = stream.read_to_string(&mut answer).unwrap();
                return answer;
            },

            // in multiplexed mode read from stdin
            ConnectionMode::Multiplexed => {
                panic!("not yet implemented");
            }
        };
    }
}
