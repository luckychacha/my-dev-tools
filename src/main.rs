pub mod base64;
pub mod error;

use clap::Parser;

use crate::base64::{Base64Input, Base64Output};

/// Try to make an tool sets for daily develop.
#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Luckychacha <luckychachaa@gmail.com>")]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Try to generate a base64 encoded string. Such as "my-dev-tools base64-encode 'hello world!'"
    Base64Encode { input: String },
    /// Parse a base64 encoded string. Such as "my-dev-tools base64-decode 'aGVsbG8gd29ybGQh'"
    Base64Decode { input: String },
}
fn main() {
    let args = Args::parse();
    match args.action {
        Action::Base64Decode { input } => {
            let result = input.as_str().parse::<Base64Input>();
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
        Action::Base64Encode { input } => {
            if let Ok(base64_encoded) = input.as_str().parse::<Base64Output>() {
                println!("Input is: {}", base64_encoded.raw);
                println!(
                    "Base64 encode result is: \"{}\"",
                    base64_encoded.base64_encoded
                );
            }
        }
    }
}
