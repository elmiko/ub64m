#[cfg(test)]
mod tests {
    use ub64m::manifest_from_filename;

    #[test]
    #[should_panic(expected = "supplied path must reference a YAML file.")]
    fn bad_file() {
        match manifest_from_filename(String::from("badfile")) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    #[test]
    #[should_panic(
        expected = "no YAML documents found in manifest file, please check source file format."
    )]
    fn empty_string() {
        match manifest_from_filename(String::from("./tests/data/emptyfile")) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    #[test]
    #[should_panic(
        expected = "more than one YAML document found in manifest file, only single document manifests are supported."
    )]
    fn multiple_documents() {
        match manifest_from_filename(String::from("./tests/data/multiple-docs.yaml")) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
