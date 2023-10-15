/*!
 * illumos SMF wrapper command
 *
 * Author: Dave Eddy <dave@daveeddy.com>
 * Date: October 09, 2023
 * License: MIT
 */

use anyhow::Result;

mod arguments;
mod commands;
mod util;

use arguments::SubCommands;

fn main() -> Result<()> {
    let args = arguments::parse();

    match args.command {
        SubCommands::List(cmd) => commands::list::run(cmd),
        SubCommands::Log(cmd) => commands::log::run(cmd),
        SubCommands::Status(cmd) => commands::status::run(cmd),
        SubCommands::Enable { .. } => commands::enable::run(),
        SubCommands::Disable { .. } => commands::disable::run(),
    }
}
