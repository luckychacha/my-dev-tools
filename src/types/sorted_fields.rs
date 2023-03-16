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
                    query_string.push_str(&format!("{key}={}&", fields.get(key).unwrap()));
                    query_string
                });
        query_string.push_str(&format!("token={token}"));
        query_string
    }
}

// todo: add tests
#[cfg(test)]
mod test {
    use super::*;

    struct User {
        pub id: usize,
        pub name: String,
        pub age: usize,
        pub balance: i32,
    }

    impl SortedFields for User {
        fn get_fields(&self, fields: &mut HashMap<String, String>) {
            fields.insert(String::from("id"), 1.to_string());
            fields.insert(String::from("name"), String::from("test"));
            fields.insert(String::from("age"), 20.to_string());
            fields.insert(String::from("balance"), 1000.to_string());
        }
    }

    #[test]
    fn sorted_fields_should_work() {
        let user = User {
            id: 1,
            name: "test".to_string(),
            age: 20,
            balance: 1000,
        };

        assert_eq!(
            user.sorted_fields("123"),
            "age=20&balance=1000&id=1&name=test&token=123"
        );
    }
}
