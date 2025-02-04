//  sk_pc_client. sk messanger pc client.
//  Copyright (C) 2024  Andrew Kozmin
//  
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published
//  by the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//  
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//  
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.


mod menus;


use anyhow::Result;
use menus::{Menu, main::Main};


fn main() {
  loop {
    match main_loop() {
      Ok(_) => break,
      Err(error) => eprintln!("CRITICAL ERROR: {error}"),
    }
  }
}


fn main_loop() -> Result<()> {
  let mut menu: Box<dyn Menu> = Main::default_menu();

  loop {
    menu.show_menu()?;
    menu = menu.process_action()?;
  }
}
