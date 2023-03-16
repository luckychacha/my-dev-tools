use std::collections::HashMap;

pub trait SortedFields {
    fn get_fields(&self, fields: &mut HashMap<String, String>);

    fn sorted_fields(&self, token: &str) -> String {
        let mut fields = HashMap::new();
        self.get_fields(&mut fields);
        let mut field_names = fields.keys().collect::<Vec<_>>();
        field_names.sort_unstable();
        let mut query_string: String =
            field_names
                .iter()
                .fold(String::new(), |mut query_string: String, &key| {
                    query_string.push_str(&format!("{key}={}", fields.get(key).unwrap()));
                    query_string
                });
        query_string.push_str(&format!("token={}", token));
        query_string
    }
}

// todo: add tests
