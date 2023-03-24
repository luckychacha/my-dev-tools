use clap::{Arg, Command};

pub trait CommandExt {
    fn add_basic_info(self) -> Self;
    fn add_base64_subcommands(self) -> Self;
    fn add_sorted_fields_subcommands(self) -> Self;
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

    // add sorted fields subcommands.
    fn add_sorted_fields_subcommands(self) -> Self {
        self.subcommand(
            Command::new("json-to-sorted-kv-string")
                .about("Try to convert a json string into a sorted kv pairs spiltted with &")
                .arg(Arg::new("input")),
        )
    }
}
