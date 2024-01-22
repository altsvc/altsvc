use altsvc::{AltAuthority, Service};

#[test]
fn test_parse() {
    let test_cases = vec![
        (
            "clear",
            vec![Service {
                clear: true,
                protocol_id: None,
                alt_authority: None,
                max_age: None,
                persist: None,
            }],
        ),
        (
            "h2=\":443\"; ma=2592000;",
            vec![Service {
                clear: false,
                protocol_id: Some("h2".to_string()),
                alt_authority: Some(AltAuthority {
                    host: None,
                    port: Some("443".to_string()),
                }),
                max_age: Some(2592000),
                persist: None,
            }],
        ),
        (
            "h2=\":443\"; ma=2592000; persist=1",
            vec![Service {
                clear: false,
                protocol_id: Some("h2".to_string()),
                alt_authority: Some(AltAuthority {
                    host: None,
                    port: Some("443".to_string()),
                }),
                max_age: Some(2592000),
                persist: Some(1),
            }],
        ),
        (
            "h2=\"alt.example.com:443\", h2=\":443\"",
            vec![
                Service {
                    clear: false,
                    protocol_id: Some("h2".to_string()),
                    alt_authority: Some(AltAuthority {
                        host: Some("alt.example.com".to_string()),
                        port: Some("443".to_string()),
                    }),
                    max_age: None,
                    persist: None,
                },
                Service {
                    clear: false,
                    protocol_id: Some("h2".to_string()),
                    alt_authority: Some(AltAuthority {
                        host: None,
                        port: Some("443".to_string()),
                    }),
                    max_age: None,
                    persist: None,
                },
            ],
        ),
        (
            "h3-25=\":443\"; ma=3600, h2=\":443\"; ma=3600",
            vec![
                Service {
                    clear: false,
                    protocol_id: Some("h3-25".to_string()),
                    alt_authority: Some(AltAuthority {
                        host: None,
                        port: Some("443".to_string()),
                    }),
                    max_age: Some(3600),
                    persist: None,
                },
                Service {
                    clear: false,
                    protocol_id: Some("h2".to_string()),
                    alt_authority: Some(AltAuthority {
                        host: None,
                        port: Some("443".to_string()),
                    }),
                    max_age: Some(3600),
                    persist: None,
                },
            ],
        ),
        // Add more test cases here.
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
        // Add more test cases here.
    ];

    for (input, error_prefix) in test_cases {
        let result = Service::parse(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with(error_prefix));
    }
}
