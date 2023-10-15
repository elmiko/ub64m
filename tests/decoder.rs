#[cfg(test)]
mod tests {
    use ub64m::decode_yaml_in_place;
    use yaml_rust::yaml::Hash;
    use yaml_rust::Yaml;

    #[test]
    fn decode_real() {
        let mut src = Yaml::Real(String::from("0.1"));
        decode_yaml_in_place(&mut src);
        assert_eq!(src.as_f64().unwrap(), 0.1);
    }

    #[test]
    fn decode_integer() {
        let mut src = Yaml::Integer(100);
        decode_yaml_in_place(&mut src);
        assert_eq!(src.as_i64().unwrap(), 100);
    }

    #[test]
    fn decode_non_base64_string() {
        let mut src = Yaml::String(String::from("Hello World!"));
        decode_yaml_in_place(&mut src);
        assert_eq!(src.as_str().unwrap(), "Hello World!");
    }

    #[test]
    fn decode_base64_string() {
        let mut src = Yaml::String(String::from("SGVsbG8gV29ybGQhCg=="));
        decode_yaml_in_place(&mut src);
        assert_eq!(src.as_str().unwrap(), "Hello World!");
    }

    #[test]
    fn decode_boolean() {
        let mut src = Yaml::Boolean(true);
        decode_yaml_in_place(&mut src);
        assert_eq!(src.as_bool().unwrap(), true);
    }

    #[test]
    fn decode_array() {
        let mut src = Yaml::Array(vec![Yaml::Integer(1), Yaml::String(String::from("Foo!"))]);
        decode_yaml_in_place(&mut src);
        assert!(src.is_array());
    }

    #[test]
    fn decode_hash() {
        let mut h = Hash::new();
        h.insert(
            Yaml::String(String::from("foo")),
            Yaml::String(String::from("bar")),
        );
        let mut src = Yaml::Hash(h);
        decode_yaml_in_place(&mut src);
        assert!(src.as_hash().is_some());
    }

    #[test]
    fn decode_null() {
        let mut src = Yaml::Null;
        decode_yaml_in_place(&mut src);
        assert!(src.is_null());
    }

    #[test]
    fn decode_badvalue() {
        let mut src = Yaml::BadValue;
        decode_yaml_in_place(&mut src);
        assert!(src.is_badvalue());
    }
}
