// Copyright (C) 2014-2015 Mickaël Salaün
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

#![feature(core)]
#![feature(env)]
#![feature(libc)]
#![feature(old_io)]

extern crate env_logger;
extern crate iohandle;
extern crate libc;
#[macro_use]
extern crate log;
extern crate pty;
extern crate stemjail;

use std::env;
use std::old_io as io;
use stemjail::cmd;

fn get_usage() -> String {
    let mut args = std::env::args();
    let name = args.next().unwrap_or("stemjail-cli".to_string());
    format!("usage: {} {}", name, cmd::list_kage_cmd_names().connect("|"))
}

fn args_fail<T: Str>(msg: T) {
    let msg = format!("{}\n\n{}\n", msg.as_slice(), get_usage().as_slice());
    io::stderr().write_str(msg.as_slice()).unwrap();
    env::set_exit_status(1);
}

fn main() {
    env_logger::init().unwrap();

    let mut args = env::args().skip(1);
    match args.next() {
        Some(cmd_name) => {
            let cmd_args: Vec<String> = args.map(|x| x.to_string()).collect();
            let mut cmd = match cmd::get_kage_cmd(&cmd_name) {
                Some(c) => c,
                None => {
                    args_fail("No command with this name");
                    return;
                }
            };
            match cmd.call(&cmd_args) {
                Ok(_) => {
                    // TODO: Wait for the portal ack if PortalRequest::CreateTty
                }
                Err(e) => {
                    args_fail(format!("Command error: {}", e));
                    return;
                }
            }
        }
        None => {
            args_fail("No command");
            return;
        }
    }
}