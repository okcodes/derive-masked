use derive_masked::{DebugMasked, DisplayMasked};

#[derive(DebugMasked, DisplayMasked)]
struct User {
    #[masked]
    password: String,
}

fn main() {
    let u = User {
        password: "super_secret".to_string(),
    };

    // Should print masked password when printed with different formatters.

    // Explicit access to masked field should print the actual value of the field.
    println!("Password:\n{}", u.password);
    println!("Display:\n{}", u);
    println!("Debug:\n{:?}", u);
    println!("Pretty Debug:\n{:#?}", u);
}
