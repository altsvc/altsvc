use crate::Service;
use crate::AltAuthority;

#[test]
fn test_parse() {
    let test_cases = vec![
        ("clear", vec![Service { clear: true, protocol_id: None, alt_authority: None, max_age: None, persist: None }]),
        // Add more test cases here
    ];

    for (input, expected) in test_cases {
        let result = Service::parse(input).unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_parse_errors() {
    let test_cases = vec![
        ("", "invalid parameter"),
        // Add more test cases here
    ];

    for (input, error_prefix) in test_cases {
        let result = Service::parse(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with(error_prefix));
    }
}
