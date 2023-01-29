pub mod base64;
pub mod error;

use clap::{command, Arg, Command, Parser};

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
fn main() {
    let app = Command::new("my-dev-tools")
        .author("luckychacha")
        .version("0.1.0")
//         .help_template("\
// {before-help}{name} {version}
// {author-with-newline}{about-with-newline}
// {usage-heading} {usage}

// {all-args}{after-help}
// ")      
        .about("Try to make a tool sets for daily develop.")
        .subcommand(
            Command::new("Base64Encode")
                .about("Try to generate a base64 encoded string. Such as \"my-dev-tools base64-encode 'hello world!'\"")
                .arg(Arg::new("input")),
        )
        .subcommand(
            Command::new("Base64Decode")
                .about("Parse a base64 encoded string. Such as \"my-dev-tools base64-decode 'aGVsbG8gd29ybGQh'\"")
                .arg(Arg::new("input")),
        ).arg_required_else_help(true);
    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("Base64Encode") {
        if let Some(s) = matches.get_one::<String>("input") {
            if let Ok(base64_encoded) = s.as_str().parse::<Base64Output>() {
                println!("Input is: {}", base64_encoded.raw);
                println!(
                    "Base64 encode result is: \"{}\"",
                    base64_encoded.base64_encoded
                );
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("Base64Decode") {
        if let Some(s) = matches.get_one::<String>("input") {
            let result = s.as_str().parse::<Base64Input>();
            match result {
                Ok(base64_decoded) => {
                    println!(
                        "Base64 decode result is: {}",
                        base64_decoded
                            .translate_into_human_readable_content()
                            .unwrap_or(String::from("Bytes convert to string error."))
                    )
                }
                Err(error) => {
                    println!("Parse input error: {error:?}")
                }
            }
        }
    }
    // let args = Args::parse();
    // match args.action {
    //     Action::Base64Decode { input } => {
    //         let result = input.as_str().parse::<Base64Input>();
    //         match result {
    //             Ok(base64_decoded) => {
    //                 println!(
    //                     "Base64 decode result is: {}",
    //                     base64_decoded
    //                         .translate_into_human_readable_content()
    //                         .unwrap_or(String::from("Bytes convert to string error."))
    //                 )
    //             }
    //             Err(error) => {
    //                 println!("Parse input error: {error:?}")
    //             }
    //         }
    //     }
    //     Action::Base64Encode { input } => {
    //         if let Ok(base64_encoded) = input.as_str().parse::<Base64Output>() {
    //             println!("Input is: {}", base64_encoded.raw);
    //             println!(
    //                 "Base64 encode result is: \"{}\"",
    //                 base64_encoded.base64_encoded
    //             );
    //         }
    //     }
    // }
}
