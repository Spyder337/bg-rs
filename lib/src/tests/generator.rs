use std::collections::HashMap;
use std::io::Empty;
use crate::generators::RustGenerator;
use crate::Generator;

#[test]
fn test_string_parsing() {
    let gen = RustGenerator;
    let libs = vec![
        String::from("web/"),
        String::from("web/rest/"),
        String::from("web/rest/wiki"),
        String::from("web/rest/rest"),
        String::from("gui/"),
    ];
    println!("{:?}", libs);
    let ret = gen.parse_lib_str(&libs);
    let mut expected: HashMap<String, Vec<String>> = HashMap::new();
    expected.insert("gui".to_string(), vec![]);
    expected.insert(
        "web".to_string(),
        vec![
            "rest/".to_string(),
            "rest/wiki".to_string(),
            "rest/rest".to_string(),
        ],
    );
    assert_eq!(ret, expected);
}

#[test]
fn test_validator() {
    let gen = RustGenerator;
    let mut res = gen.validate_input(crate::ProjectType::Empty, &vec![]);
    assert_eq!(res.is_ok(), true);
    res = gen.validate_input(crate::ProjectType::Empty, &vec!["gui".to_string()]);
    assert_eq!(res.is_ok(), true);
    res = gen.validate_input(crate::ProjectType::Nested, &vec![]);
    assert_eq!(res.is_err(), true);
    res = gen.validate_input(crate::ProjectType::NestedBin, &vec![]);
    assert_eq!(res.is_err(), true);
}
