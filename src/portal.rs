// Copyright (C) 2014 Mickaël Salaün
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#![crate_name = "portal"]
#![crate_type = "bin"]
#![desc = "stemjail Portal"]
#![license = "LGPL-3.0"]

extern crate stemjail;
extern crate serialize;

use std::io::fs;
use std::io::{Listener, Acceptor};
use std::io::net::unix::{UnixListener, UnixStream};
use serialize::json;
use self::stemjail::plugins;

fn handle_client(mut stream: UnixStream) -> Result<(), String> {
    let encoded_str = match stream.read_to_string() {
        Ok(s) => s,
        Err(e) => {
                return Err(format!("Error reading client: {}", e));
        }
    };
    // FIXME: task '<main>' failed at 'called `Option::unwrap()` on a `None` value', .../rust/src/libcore/option.rs:265
    let decoded: plugins::PortalPluginCommand = match json::decode(encoded_str.as_slice()) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };
    println!("Portal got: {}", decoded);
    Ok(())
}

fn main() {
    let server = Path::new(stemjail::PORTAL_SOCKET_PATH);
    // FIXME: Use libc::SO_REUSEADDR for unix socket instead of removing the file
    let _ = fs::unlink(&server);
    let stream = UnixListener::bind(&server);
    for stream in stream.listen().incoming() {
        match stream {
            Ok(s) => {
                spawn(proc() {
                    match handle_client(s) {
                        Ok(_) => {},
                        Err(e) => println!("Error reading client: {}", e),
                    }
                });
            }
            Err(e) => {
                println!("Connection error: {}", e);
                return;
            }
        }
    }
}
