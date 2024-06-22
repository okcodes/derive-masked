# derive-masked

A Rust procedural macro crate for deriving `Display` and `Debug` implementations that automatically mask sensitive fields, ensuring secure and controlled output of struct data.

## Features

- **`DisplayMasked` Macro**: Derives a `Display` implementation for your struct that masks sensitive fields.
- **`DebugMasked` Macro**: Derives a `Debug` implementation for your struct with similar masking functionality.
- **Attribute-Based Masking**: Use the `#[masked]` attribute on fields that should be masked in the output.

## Installation

Add `derive-masked` to your `Cargo.toml`:

```toml
[dependencies]
derive-masked = "0.1"
```

## Usage

### Deriving `DisplayMasked` and `DebugMasked`

- Use the `DisplayMasked` macro to automatically implement the `Display` trait for your struct, with sensitive fields masked:
- Use the `DebugMasked` macro to automatically implement the `Debug` trait for your struct, masking sensitive fields:
- You can derive both `DisplayMasked` and `DebugMasked` or only one, it's up to your needs.
- Use the `#[masked]` attribute on fields that you want to be masked in the output. You can use this on multiple fields:

```rust
use derive_masked::{DebugMasked, DisplayMasked};

#[derive(DebugMasked, DisplayMasked)]
struct User {
    name: String,
    #[masked]
    password: String,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        password: "super_secret".to_string(),
    };

    // Uses DisplayMasked. Output: User { name: Alice, password: ***** }
    println!("{}", user);

    // Uses DebugMasked. Output: User { name: "Alice", password: ***** }
    println!("{:?}", user);

    // Uses DebugMasked with pretty print formatter. Output:
    // User {
    //     name: "Alice",
    //     password: *****
    // }
    println!("{:#?}", user);
}
```

## Examples

Find more examples in the [examples](examples) directory. Each example demonstrates different use cases and functionalities of the macros.

## Contribution

We welcome contributions! Please follow these steps to contribute to the project:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Write your code and tests.
4. Submit a pull request.

Make sure to adhere to the coding standards and include tests for any new features or bug fixes.

## License

`derive-masked` is licensed under the [MIT License](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE), at your option.

## Contact

For any issues, suggestions, or questions, please open an issue on GitHub.

---

Happy coding with `derive-masked`! 🎉
