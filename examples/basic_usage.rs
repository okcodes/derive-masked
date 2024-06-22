use derive_masked::{DebugMasked, DisplayMasked};

#[derive(DebugMasked, DisplayMasked)]
struct User {
    #[masked]
    password: String,
}

fn main() {
    let user = User {
        password: "super_secret".to_string(),
    };

    // Should print masked password when printed with different formatters.

    // Explicit access to masked field should print the actual value of the field.
    println!("Password:\n{}", user.password);
    println!("Display:\n{}", user);
    println!("Debug:\n{:?}", user);
    println!("Pretty Debug:\n{:#?}", user);
}
