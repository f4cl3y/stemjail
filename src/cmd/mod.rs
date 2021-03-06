// Copyright (C) 2014-2016 Mickaël Salaün
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

/// # Serializable types
/// enum `PortalCall` -> custom cmd { enum Action -> struct Request }
/// enum `MonitorCall` -> custom cmd { enum Action -> struct Request }
/// struct `PortalAck` -> `PartalRequest`

use bincode::rustc_serialize::{DecodingResult, EncodingResult};

#[macro_use]
mod util;

// Commands
pub mod info;
mod mount;
mod run;
pub mod shim;


pub trait KageCommand {
    fn get_name<'a>(&'a self) -> &'a String;
    fn get_usage(&self) -> String;
    fn call(&mut self, args: &Vec<String>) -> Result<(), String>;
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum PortalCall {
    Run(run::RunAction),
    Info(info::InfoAction),
}
impl_encdec!(PortalCall);

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum MonitorCall {
    Mount(mount::MountAction),
    Shim(shim::ShimAction),
}
impl_encdec!(MonitorCall);

#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub struct PortalAck {
    pub request: PortalRequest,
}
impl_encdec!(PortalAck);

#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub enum PortalRequest {
    Nop,
    CreateTty,
}

fn list_kage_cmds<'a>() -> Vec<Box<KageCommand + 'a>> {
    vec!(
        Box::new(self::info::InfoKageCmd::new()) as Box<KageCommand>,
        Box::new(self::mount::MountKageCmd::new()) as Box<KageCommand>,
        Box::new(self::run::RunKageCmd::new()) as Box<KageCommand>,
        Box::new(self::shim::ShimKageCmd::new()) as Box<KageCommand>,
    )
}

pub fn get_kage_cmd(name: &String) -> Option<Box<KageCommand>> {
    for cmd in list_kage_cmds().into_iter() {
        if cmd.get_name() == name {
            return Some(cmd);
        }
    }
    None
}

pub fn list_kage_cmd_names() -> Vec<String> {
    list_kage_cmds().iter().map(|x| x.get_name().clone()).collect()
}
