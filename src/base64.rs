use std::{fmt::Display, str::FromStr};

use base64::{engine::general_purpose, Engine};

use crate::{error::ToolErrors, types::my_arg_matches::MyArgMatches};

#[derive(Debug, PartialEq, Eq)]
pub struct Base64Input {
    pub base64_encoded: String,
    pub base64_decoded: Vec<u8>,
}

impl Base64Input {
    pub fn new(input: &str, bytes: Vec<u8>) -> Self {
        Self {
            base64_encoded: String::from(input),
            base64_decoded: bytes,
        }
    }

    pub fn translate_into_human_readable_content(&self) -> Result<String, ToolErrors> {
        let human_readable_content = String::from_utf8(self.base64_decoded.clone())?;
        Ok(human_readable_content)
    }
}

impl FromStr for Base64Input {
    type Err = ToolErrors;

    fn from_str(s: &str) -> Result<Self, ToolErrors> {
        let bytes = general_purpose::STANDARD.decode(s)?;
        Ok(Base64Input::new(s, bytes))
    }
}

impl Display for Base64Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: {}\r\nBase64 Decoded: \"{}\"",
            self.base64_encoded,
            self.translate_into_human_readable_content()
                .unwrap_or(String::from("Bytes convert to string error."))
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Base64Output {
    pub raw: String,
    pub base64_encoded: String,
}

impl Base64Output {
    pub fn new(input: &str, base64_encoded: String) -> Self {
        Self {
            raw: String::from(input),
            base64_encoded,
        }
    }
}

impl Display for Base64Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: {}\r\nBase64 encoded: \"{}\"",
            self.raw, self.base64_encoded
        )
    }
}

impl FromStr for Base64Output {
    type Err = ToolErrors;

    fn from_str(s: &str) -> Result<Self, ToolErrors> {
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(s, &mut buf);
        Ok(Base64Output::new(s, buf))
    }
}

impl MyArgMatches {
    pub fn base64_tools(self) {
        if let Some(matches) = self.0.subcommand_matches("base64-encode") {
            if let Some(s) = matches.get_one::<String>("input") {
                if let Ok(base64_encoded) = s.as_str().parse::<Base64Output>() {
                    println!("{base64_encoded}");
                }
            }
        } else if let Some(matches) = self.0.subcommand_matches("base64-decode") {
            if let Some(s) = matches.get_one::<String>("input") {
                let result = s.as_str().parse::<Base64Input>();
                match result {
                    Ok(base64_decoded) => {
                        println!("{base64_decoded}")
                    }
                    Err(error) => {
                        println!("Base64 Parse input error: {error:?}. Input: \"{s}\"");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_input_and_decode_should_work() {
        let base64_encode = "YWJjZGU=";
        let base64_decoded: Result<Base64Input, ToolErrors> = base64_encode.parse();

        assert!(base64_decoded.is_ok());
        if let Ok(tmp) = base64_decoded {
            assert_eq!(
                tmp.translate_into_human_readable_content(),
                Ok(String::from("abcde"))
            )
        }
    }

    #[test]
    fn invalid_input_and_decode_should_reject() {
        let base64_encode = "abcde";
        let base64_decoded: Result<Base64Input, ToolErrors> = base64_encode.parse();

        assert!(base64_decoded.is_err());
        assert_eq!(
            base64_decoded,
            Err(ToolErrors::Base64DecodeError(
                base64::DecodeError::InvalidLength
            ))
        );
    }

    #[test]
    fn valid_input_and_encode_should_work() {
        let test = "abcde";
        assert_eq!(
            test.parse(),
            Ok(Base64Output {
                raw: String::from(test),
                base64_encoded: String::from("YWJjZGU=")
            })
        );
    }
}
