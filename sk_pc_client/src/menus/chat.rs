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


use anyhow::Result;

use super::Menu;


#[derive(Clone)]
pub(crate) struct Chat {
  name: String,
  address: String,
  key: String,
}


impl Menu for Chat {
  fn show_menu(&self) -> Result<()> {
    todo!("show_menu")
  }


  fn process_action(&self) -> Result<Box<dyn Menu>> {
    todo!("process_action")
  }
}


impl Chat {
  pub(crate) fn new(name: String, address: String, key: String) -> Self {
    Self {
      name,
      address,
      key,
    }
  }


  pub(crate) fn get_name(&self) -> &String {
    &self.name
  }


  pub(crate) fn get_menu(&self) -> Box<dyn Menu> {
    Box::new(self.clone())
  }
}
