#[cfg(test)]
mod tests {
    use derive_masked::{DebugMasked, DisplayMasked};

    #[derive(DebugMasked, DisplayMasked, PartialEq, Eq, Clone)]
    struct User {
        name: String,
        #[masked]
        password: String,
    }

    #[derive(DebugMasked, DisplayMasked)]
    struct ComplexStruct {
        title: String,
        #[masked]
        api_key: String,
        #[masked]
        expired_api_keys: Vec<String>,
        #[masked]
        port: u16,
        public_user: User,
        #[masked]
        masked_user: User,
    }

    #[test]
    fn test_masked() {
        let user = User {
            name: "John Doe".to_string(),
            password: "secret".to_string(),
        };

        // The struct must be printed using the non-pretty "debug" formatter.
        assert_eq!(
            format!("{:?}", user),
            r#"User { name: "John Doe", password: ***** }"#
        );

        // The struct must be printed using the non-pretty "display" formatter.
        assert_eq!(
            format!("{:}", user),
            r#"User { name: John Doe, password: ***** }"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(user.password, user.password);
    }

    #[test]
    fn test_masked_pretty() {
        let user = User {
            name: "John Doe".to_string(),
            password: "secret".to_string(),
        };

        // The struct must be printed using the pretty "debug" formatter.
        assert_eq!(
            format!("{:#?}", user),
            r#"User {
    name: "John Doe",
    password: *****
}"#
        );

        // The struct must be printed using the pretty "display" formatter.
        assert_eq!(
            format!("{:#}", user),
            r#"User {
    name: John Doe,
    password: *****
}"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(user.password, user.password);
    }

    #[test]
    fn test_masked_complex() {
        let public_user = User {
            name: "public user".to_string(),
            password: "password public user".to_string(),
        };

        let masked_user = User {
            name: "masked user".to_string(),
            password: "password masked user".to_string(),
        };

        let complex = ComplexStruct {
            title: "my complex struct".to_string(),
            api_key: "12345".to_string(),
            expired_api_keys: vec!["key-1".to_string(), "key-2".to_string()],
            port: 22,
            public_user: public_user.clone(),
            masked_user: masked_user.clone(),
        };

        // The struct must be printed using the non-pretty "debug" formatter.
        assert_eq!(
            format!("{:?}", complex),
            r#"ComplexStruct { title: "my complex struct", api_key: *****, expired_api_keys: *****, port: *****, public_user: User { name: "public user", password: ***** }, masked_user: ***** }"#
        );

        // The struct must be printed using the non-pretty "display" formatter.
        assert_eq!(
            format!("{:}", complex),
            r#"ComplexStruct { title: my complex struct, api_key: *****, expired_api_keys: *****, port: *****, public_user: User { name: public user, password: ***** }, masked_user: ***** }"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(complex.api_key, "12345".to_string());
        assert_eq!(
            complex.expired_api_keys,
            vec!["key-1".to_string(), "key-2".to_string()]
        );
        assert_eq!(complex.port, 22);
        assert_eq!(complex.public_user, public_user);
        assert_eq!(complex.masked_user, masked_user);
    }

    #[test]
    fn test_debug_masked_complex_pretty() {
        let public_user = User {
            name: "public user".to_string(),
            password: "password public user".to_string(),
        };

        let masked_user = User {
            name: "masked user".to_string(),
            password: "password masked user".to_string(),
        };

        let complex = ComplexStruct {
            title: "my complex struct".to_string(),
            api_key: "12345".to_string(),
            expired_api_keys: vec!["key-1".to_string(), "key-2".to_string()],
            port: 22,
            public_user: public_user.clone(),
            masked_user: masked_user.clone(),
        };

        // The struct must be printed using the pretty "debug" formatter.
        // TODO: The 'public_users' field is not being printed using the pretty formatter, but but with the normal formatter (still masked, so it's not a security risk).
        assert_eq!(
            format!("{:#?}", complex),
            r#"ComplexStruct {
    title: "my complex struct",
    api_key: *****,
    expired_api_keys: *****,
    port: *****,
    public_user: User { name: "public user", password: ***** },
    masked_user: *****
}"#
        );

        // The struct must be printed using the pretty "display" formatter.
        assert_eq!(
            format!("{:#}", complex),
            r#"ComplexStruct {
    title: my complex struct,
    api_key: *****,
    expired_api_keys: *****,
    port: *****,
    public_user: User { name: public user, password: ***** },
    masked_user: *****
}"#
        );

        // The underlying fields in the structs are intact.
        assert_eq!(complex.api_key, "12345".to_string());
        assert_eq!(
            complex.expired_api_keys,
            vec!["key-1".to_string(), "key-2".to_string()]
        );
        assert_eq!(complex.port, 22);
        assert_eq!(complex.public_user, public_user);
        assert_eq!(complex.masked_user, masked_user);
    }
}
