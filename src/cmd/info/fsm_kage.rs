// Copyright (C) 2015 Mickaël Salaün
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

/// Finite-state machine for a `KageCommand` call

use bufstream::BufStream;
use cmd::PortalCall;
use cmd::util::{recv, send};
use PORTAL_SOCKET_PATH;
use std::marker::PhantomData;
use super::{DotRequest, DotResponse, InfoAction};
use unix_socket::UnixStream;

macro_rules! fsm_next {
    ($myself: expr) => {
        KageFsm {
            bstream: $myself.bstream,
            _state: PhantomData,
        }
    }
}


// Private states
mod state {
    #[allow(dead_code)]
    pub struct Init;
    #[allow(dead_code)]
    pub struct RecvDot;
}

pub struct KageFsm<T> {
    bstream: BufStream<UnixStream>,
    _state: PhantomData<T>,
}

// Dummy FSM for now, but help to keep it consistent and enforce number of actions
impl KageFsm<state::Init> {
    pub fn new() -> Result<KageFsm<state::Init>, String> {
        let server = PORTAL_SOCKET_PATH;
        let bstream = match UnixStream::connect(&server) {
            Ok(s) => BufStream::new(s),
            Err(e) => return Err(format!("Failed to connect: {}", e)),
        };
        Ok(KageFsm {
            bstream: bstream,
            _state: PhantomData,
        })
    }

    pub fn send_dot_request(mut self, req: DotRequest) -> Result<KageFsm<state::RecvDot>, String> {
        let action = PortalCall::Info(InfoAction::GetDot(req));
        try!(send(&mut self.bstream, action));
        Ok(fsm_next!(self))
    }
}

impl KageFsm<state::RecvDot> {
    pub fn recv_dot_response(mut self) -> Result<DotResponse, String> {
        recv(&mut self.bstream)
    }
}
