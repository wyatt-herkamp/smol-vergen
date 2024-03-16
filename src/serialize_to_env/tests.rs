use serde::Serialize;

use super::serialize_to_map;

#[derive(Serialize)]
pub struct BasicSerializeStruct {
    name: String,
    email: String,
}
#[test]
pub fn test() {
    let s = BasicSerializeStruct {
        name: "John Doe".to_owned(),
        email: "test@example.com".to_owned(),
    };
    let result = serialize_to_map("TEST", &s).unwrap();
    assert_eq!(result.get("TEST_NAME").unwrap(), "John Doe");
    assert_eq!(result.get("TEST_EMAIL").unwrap(), "test@example.com");
}
