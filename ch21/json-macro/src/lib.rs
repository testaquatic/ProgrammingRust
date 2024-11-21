#![recursion_limit = "256"]

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![$( json!($element) ),*])
    };
    ({ $( $key:tt : $value:tt),* }) => {
        Json::Object(Box::new(
            vec![$( ($key.to_string(), json!($value)) ),*].into_iter().collect()
        ))
    };
    ($other:tt) => {
        Json::from($other)
    };
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(value: &'a str) -> Self {
        Json::String(value.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

#[cfg(test)]
mod tests {
    use crate::Json;

    #[test]
    fn json_null() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn json_array_with_json_element() {
        let macro_generated_value = json!([
            {
                "pitch": 440.0
            }
        ]);

        let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
            [("pitch".to_string(), Json::Number(440.0))]
                .into_iter()
                .collect(),
        ))]);

        assert_eq!(macro_generated_value, hand_coded_value);
    }
}
