# derive-masked

**`derive-masked`** is a procedural macro crate for deriving `Display` and `Debug` implementations that mask sensitive fields, ensuring secure and controlled output of struct data.

## Features

- Derive `DisplayMasked` for masked `Display` trait implementation.
- Derive `DebugMasked` for masked `Debug` trait implementation.
- Mask sensitive fields with the `#[masked]` attribute.

You can derive both `DisplayMasked` and `DebugMasked` if you want.

## Usage

Add to `Cargo.toml`:

```shell
cargo add derive-masked
```

## Example

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

For more examples, see the [examples](examples) directory.

## Getting Help

For support, file an issue on [GitHub](https://github.com/okcodes/derive-masked).

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at your option.
