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


use std::{
  io::{stdin, stdout, Write},
  process::exit
};

use anyhow::{Context, Result};
use strum::{EnumIter, EnumMessage, IntoEnumIterator};

use super::{Menu, chats::Chats};


#[derive(EnumIter, EnumMessage)]
pub(crate) enum Main {
  #[strum(message = "Chats", detailed_message = "Communicating with other people")]
  Chats,
  #[strum(message = "Exit", detailed_message = "Exit the application")]
  Exit,
}


impl Default for Main {
  fn default() -> Self {
    Self::Chats
  }
}


impl Menu for Main {
  fn show_menu(&self) -> Result<()> {
    for (i, e) in Self::iter().enumerate() {
      let action_name: String = e.get_message().context("The name of the action was not found")?.to_string();
      let action_description: String = e.get_detailed_message().context("The description of the action was not found")?.to_string();
      println!("[{}] [{action_name}] -> {action_description}", i + 1);
    }
    print!("~$ ");
    stdout().flush()?;
    Ok(())
  }


  fn process_action(&self) -> Result<Box<dyn Menu>> {
    let mut action: String = String::new();
    stdin().read_line(&mut action)?;

    let index: usize = action.trim().parse().context("Couldn't parse the input")?;
    let action: Option<Main> = Self::iter().get(index.checked_sub(1).context("Couldn't convert the input")?);

    let menu: Box<dyn Menu> = match action {
      Some(Self::Chats) => {
        Self::chats_menu()
      },

      Some(Self::Exit) => {
        exit(0);
      },

      None => {
        println!("Unknown action");
        Self::default_menu()
      },
    };

    Ok(menu)
  }
}


impl Main {
  fn chats_menu() -> Box<Chats> {
    Chats::default_menu()
  }


  pub(crate) fn default_menu() -> Box<Self> {
    Box::new(Self::default())
  }
}
