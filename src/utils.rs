/* 
utils.rs
Rusified in 2021 Copyright Israel Dahl. All rights reserved.

       /VVVV\
     /V      V\
   /V          V\
  /      0 0     \
  \|\|\</\/\>/|/|/
       \_/\_/

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License version 2 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

*/
use std::env;
/// This returns Some(value) OR None a.k.a. `Option<String>`
///
/// env::var returns Result, so we look for a language based on env::var(LANG||LANGUAGE) stripping the utf encoding and return as `Some(lang_for_desktop_files)`
pub fn get_language() -> Option<String> {
	let lang = "LANG";
	match env::var(lang) {
		Ok(value) => {
			let mut lang_var: String = value;
			let pos = lang_var.chars().position(|c| c == '.');
			if pos.is_some() {
				let posi = pos.unwrap();
				if posi < lang_var.len() {
					let _junk = lang_var.split_off(posi);
				}
			}
			return Some(lang_var)
		},
		Err(_error) => {
			let langu = "LANGUAGE";
			match env::var(langu) {
				Err(_error) => return None,
				Ok(value) => return Some(value),
			}
				
		},
	}
}
