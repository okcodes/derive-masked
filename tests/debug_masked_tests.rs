#[cfg(test)]
mod tests {
    use derive_masked::DebugMasked;

    #[derive(DebugMasked, PartialEq, Eq, Clone)]
    struct User {
        name: String,
        #[masked]
        password: String,
    }

    #[derive(DebugMasked)]
    struct ComplexStruct {
        title: String,
        #[masked]
        api_key: String,
        #[masked]
        expired_api_keys: Vec<String>,
        #[masked]
        port: u16,
        public_users: Vec<User>,
        #[masked]
        private_users: Vec<User>,
    }

    #[test]
    fn test_debug_masked() {
        let user = User {
            name: "John Doe".to_string(),
            password: "secret".to_string(),
        };

        // The struct must be printed using the non-pretty debug formatter.
        assert_eq!(
            format!("{:?}", user),
            r#"User { name: "John Doe", password: ***** }"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(user.password, user.password);
    }

    #[test]
    fn test_debug_masked_pretty_formatter() {
        let user = User {
            name: "John Doe".to_string(),
            password: "secret".to_string(),
        };

        // The struct must be printed using the pretty debug formatter.
        assert_eq!(
            format!("{:#?}", user),
            r#"User {
    name: "John Doe",
    password: *****
}"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(user.password, user.password);
    }

    #[test]
    fn test_debug_masked_complex_struct() {
        let public_users = vec![User {
            name: "user one".to_string(),
            password: "password one".to_string(),
        }];

        let private_users = vec![User {
            name: "user two".to_string(),
            password: "password two".to_string(),
        }];

        let complex = ComplexStruct {
            title: "my complex struct".to_string(),
            api_key: "12345".to_string(),
            expired_api_keys: vec!["key-1".to_string(), "key-2".to_string()],
            port: 22,
            public_users: public_users.clone(),
            private_users: private_users.clone(),
        };

        // The struct must be printed using the non-pretty debug formatter.
        assert_eq!(
            format!("{:?}", complex),
            r#"ComplexStruct { title: "my complex struct", api_key: *****, expired_api_keys: *****, port: *****, public_users: [User { name: "user one", password: ***** }], private_users: ***** }"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(complex.api_key, "12345".to_string());
        assert_eq!(
            complex.expired_api_keys,
            vec!["key-1".to_string(), "key-2".to_string()]
        );
        assert_eq!(complex.port, 22);
        assert_eq!(complex.public_users, public_users);
        assert_eq!(complex.private_users, private_users);
    }

    #[test]
    fn test_debug_masked_complex_struct_pretty_formatter() {
        let public_users = vec![User {
            name: "user one".to_string(),
            password: "password one".to_string(),
        }];

        let private_users = vec![User {
            name: "user two".to_string(),
            password: "password two".to_string(),
        }];

        let complex = ComplexStruct {
            title: "my complex struct".to_string(),
            api_key: "12345".to_string(),
            expired_api_keys: vec!["key-1".to_string(), "key-2".to_string()],
            port: 22,
            public_users: public_users.clone(),
            private_users: private_users.clone(),
        };

        // The struct must be printed using the pretty debug formatter.
        // TODO: The 'public_users' field is not being printed using the pretty formatter.
        assert_eq!(
            format!("{:#?}", complex),
            r#"ComplexStruct {
    title: "my complex struct",
    api_key: *****,
    expired_api_keys: *****,
    port: *****,
    public_users: [User { name: "user one", password: ***** }],
    private_users: *****
}"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(complex.api_key, "12345".to_string());
        assert_eq!(
            complex.expired_api_keys,
            vec!["key-1".to_string(), "key-2".to_string()]
        );
        assert_eq!(complex.port, 22);
        assert_eq!(complex.public_users, public_users);
        assert_eq!(complex.private_users, private_users);
    }
}
