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

extern crate serialize;
extern crate toml;

pub mod profile;

// TODO: Check for absolute path only
pub fn get_config<T: serialize::Decodable<toml::Decoder, toml::Error>>(config_file: &Path) -> Result<T, String> {
    let root = match toml::parse_from_file(format!("{}", config_file.display()).as_slice()) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error parsing config file: {}", e)),
    };
    let config: Result<T, toml::Error> = toml::from_toml(root);
    match config {
        Ok(c) => Ok(c),
        Err(toml::ParseError) => {
            Err("Parsing error".to_string())
        },
        Err(toml::ParseErrorInField(field)) => {
            Err(format!("Parsing error in field: {}", field))
        },
        Err(toml::IOError(e)) => {
            Err(format!("I/O error: {}", e))
        },
    }
}
