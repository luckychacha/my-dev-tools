use std::collections::HashMap;

use crate::error::ToolErrors;

pub trait SortedFields {
    fn get_fields(&self) -> Result<HashMap<String, String>, ToolErrors>;

    fn sorted_fields(&self) -> String {
        // let fields = self.get_fields().unwrap();

        // let mut field_names = fields.keys().collect::<Vec<_>>();
        // field_names.sort_unstable();
        // let query_string: String =
        //     field_names
        //         .iter()
        //         .fold(String::new(), |mut query_string: String, &key| {
        //             query_string.push_str(&format!("{key}={}&", fields.get(key).unwrap()));
        //             query_string
        //         });
        // query_string
        let mut tmp = self
            .get_fields()
            .unwrap()
            .iter()
            .map(|(key, value)| format!("{}={}&", key, value))
            .collect::<Vec<String>>();
        tmp.sort_unstable();
        tmp.join("")
    }

    fn generate_md5_sign(&self, sign_key: &str, token_key: &str, token: &str) -> String {
        let sorted_fields = self.sorted_fields();
        let digest = md5::compute(sorted_fields + &format!("{token_key}={token}"));
        format!("{sign_key}={digest:x}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use serde_json::Value;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct User {
        pub id: usize,
        pub name: String,
        pub age: usize,
        pub balance: i32,
    }

    impl SortedFields for User {
        fn get_fields(&self) -> Result<HashMap<String, String>, ToolErrors> {
            let mut fields = HashMap::new();
            fields.insert(String::from("id"), 1.to_string());
            fields.insert(String::from("name"), String::from("test"));
            fields.insert(String::from("age"), 20.to_string());
            fields.insert(String::from("balance"), 1000.to_string());
            Ok(fields)
        }
    }

    impl SortedFields for String {
        fn get_fields(&self) -> Result<HashMap<String, String>, ToolErrors> {
            // if let Ok(Value::Object(obj)) = serde_json::from_str(self) {
            //     let res = obj
            //         .iter()
            //         .fold(HashMap::<String, String>::new(), |mut res, value| {
            //             res.insert(value.0.to_string(), value.1.to_string());
            //             res
            //         });
            //     Ok(res)
            // } else {
            //     Err(ToolErrors::ConvertJsonToHashMapError(self.clone()))
            // }
            serde_json::from_str(self)
                .map_err(|_| ToolErrors::ConvertJsonToHashMapError(self.clone()))
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

        assert_eq!(user.sorted_fields(), "age=20&balance=1000&id=1&name=test&");
    }

    #[test]
    fn test_sorted_fields() {
        let json_str = r#"{"name": "John", "age": 30, "city": "New York}".to_string();

        let expected_fields = vec![
            ("age".to_string(), "30".to_string()),
            ("city".to_string(), "New York".to_string()),
            ("name".to_string(), "John".to_string()),
        ]
        .into_iter()
        .collect::<HashMap<String, String>>();

        let actual_fields = json_str.get_fields().unwrap();

        assert_eq!(actual_fields, expected_fields);
    }

    #[test]
    fn test_sorted_fields_invalid_json() {
        let json_str = r#"{"name": "John", "age": 30, "city": "New York}"#
            .to_string();

        let result = json_str.get_fields();

        assert!(result.is_err());
    }

    #[test]
    fn generate_md5_sign_should_work() {
        let user = User {
            id: 1,
            name: "test".to_string(),
            age: 20,
            balance: 1000,
        };

        assert_eq!(user.sorted_fields(), "age=20&balance=1000&id=1&name=test&");
        assert_eq!(
            user.generate_md5_sign("sign", "token", "123"),
            "sign=9746b373a2e410c31142a1c039dbfa29"
        );
    }
}
