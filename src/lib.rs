pub mod base64;
pub mod error;
pub mod types;
use clap::Command;
use types::{
    command::CommandExt,
    my_arg_matches::{MyArgMatches, SubCommandExt},
};

pub fn command_parse() {
    let matches = Command::new("my-dev-tools")
        .add_basic_info()
        .add_base64_subcommands()
        .get_matches();

    let my_matches = MyArgMatches(matches);
    my_matches.exec();
}
