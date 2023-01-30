pub mod base64;
pub mod error;

use clap::{command, Arg, ArgMatches, Command, Parser};

use crate::base64::{Base64Input, Base64Output};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Base64Encode { input: String },
    Base64Decode { input: String },
}

pub fn command_parse() {
    let matches = Command::new("my-dev-tools")
        .add_basic_info()
        .add_base64_subcommands()
        .get_matches();

    matches.exec();
}

pub trait CommandExt {
    fn add_basic_info(self) -> Self;
    fn add_base64_subcommands(self) -> Self;
}

pub trait SubCommandExt {
    fn exec(self);
    fn base64_tools(self);
}

impl SubCommandExt for ArgMatches {
    fn exec(self) {
        match self.subcommand_name() {
            Some("base64-encode") | Some("base64-decode") => self.base64_tools(),
            _ => {}
        }
    }

    fn base64_tools(self) {
        if let Some(matches) = self.subcommand_matches("base64-encode") {
            if let Some(s) = matches.get_one::<String>("input") {
                if let Ok(base64_encoded) = s.as_str().parse::<Base64Output>() {
                    println!("{base64_encoded}");
                }
            }
        } else if let Some(matches) = self.subcommand_matches("base64-decode") {
            if let Some(s) = matches.get_one::<String>("input") {
                let result = s.as_str().parse::<Base64Input>();
                match result {
                    Ok(base64_decoded) => {
                        println!("{base64_decoded}")
                    }
                    Err(error) => {
                        println!("Parse input error: {error:?}")
                    }
                }
            }
        }
    }
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
