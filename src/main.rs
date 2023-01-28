pub mod base64;
pub mod error;

use clap::Parser;

use crate::base64::{Base64Input, Base64Output};

#[derive(clap::Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Base64Decode { input: String },
    Base64Encode { input: String },
}
fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    match args.action {
        Action::Base64Decode { input } => {
            let result = input.as_str().parse::<Base64Input>();
            match result {
                Ok(base64_decoded) => {
                    println!(
                        "{}",
                        base64_decoded
                            .translate_into_human_readable_content()
                            .unwrap_or(String::from("Bytes convert to string error."))
                    )
                }
                Err(error) => {
                    println!("Parse input error: {:?}", error)
                }
            }
        }
        Action::Base64Encode { input } => {
            if let Ok(base64_encoded) = input.as_str().parse::<Base64Output>() {
                println!("Input is: {}", base64_encoded.raw);
                println!(
                    "Base64 encoded result is: {}",
                    base64_encoded.base64_encoded
                );
            }
        }
    }
}
