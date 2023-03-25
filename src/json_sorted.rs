use crate::types::{my_arg_matches::MyArgMatches, sorted_fields::SortedFields};

impl MyArgMatches {
    pub fn sorted_fields_tools(self) {
        if let Some(matches) = self.0.subcommand_matches("json-to-sorted-kv-string") {
            if let Some(s) = matches.get_one::<String>("input") {
                println!("Got input json: {s}");
                println!("sorted kv pairs string is: {}", s.sorted_fields());
            }
        }
    }
}
