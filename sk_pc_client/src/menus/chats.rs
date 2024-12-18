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


use std::io::{stdin, stdout, Write};

use anyhow::{Context, Result};

use super::{Menu, main::Main, chat::Chat};


pub(crate) struct Chats {
  chats: Vec<Chat>,
}


impl Default for Chats {
  fn default() -> Self {
    Self::new(
      Vec::default(),
    )
  }
}


impl Menu for Chats {
  fn show_menu(&self) -> Result<()> {
    println!("[1] - > Back");
    for (index, chat) in self.chats.iter().enumerate() {
      println!("[{}] -> {}", index + 2, chat.get_name());
    }
    print!("~$ ");
    stdout().flush()?;
    Ok(())
  }


  fn process_action(&self) -> Result<Box<dyn Menu>> {
    let mut action: String = String::new();
    stdin().read_line(&mut action)?;

    let index: usize = action.trim().parse().context("Couldn't parse the input")?;

    if index == 1 {
      return Ok(Main::default_menu());
    }

    let chat: Option<&Chat> = self.chats.get(index.checked_sub(1).context("Couldn't convert the input")?);

    Ok(chat.context("Couldn't find the specified chat")?.get_menu())
  }
}


impl Chats {
  fn new(chats: Vec<Chat>) -> Self {
    Self {
      chats,
    }
  }


  pub(crate) fn default_menu() -> Box<Self> {
    Box::new(Self::default())
  }
}
