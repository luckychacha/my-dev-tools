pub mod base64;
pub mod error;
pub mod types;
use clap::{Arg, Command};
use types::my_arg_matches::{MyArgMatches, SubCommandExt};

pub fn command_parse() {
    let matches = Command::new("my-dev-tools")
        .add_basic_info()
        .add_base64_subcommands()
        .get_matches();

    let my_matches = MyArgMatches(matches);
    my_matches.exec();
}

pub trait CommandExt {
    fn add_basic_info(self) -> Self;
    fn add_base64_subcommands(self) -> Self;
}

impl CommandExt for Command {
    fn add_basic_info(self) -> Self {
        self.author("luckychacha")
            .version("0.1.0")
            //         .help_template("\
            // {before-help}{name} {version}
            // {author-with-newline}{about-with-newline}
            // {usage-heading} {usage}
            // {all-args}{after-help}
            // ")
            .about("Try to make a tool sets for daily develop.")
            .arg_required_else_help(true)
    }

    fn add_base64_subcommands(self) -> Self {
        self.subcommand(
            Command::new("base64-encode")
                .about("Try to generate a base64 encoded string. Such as \"my-dev-tools base64-encode 'hello world!'\"")
                .arg(Arg::new("input")),
        )
        .subcommand(
            Command::new("base64-decode")
                .about("Parse a base64 encoded string. Such as \"my-dev-tools base64-decode 'aGVsbG8gd29ybGQh'\"")
                .arg(Arg::new("input")),
        )
    }
}
