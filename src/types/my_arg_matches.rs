use clap::ArgMatches;

pub struct MyArgMatches(pub ArgMatches);

pub trait SubCommandExt {
    fn exec(self);
}

impl SubCommandExt for MyArgMatches {
    fn exec(self) {
        match self.0.subcommand_name() {
            Some("base64-encode") | Some("base64-decode") => self.base64_tools(),
            Some("json-to-sorted-kv-string") => self.sorted_fields_tools(),
            _ => {}
        }
    }
}
