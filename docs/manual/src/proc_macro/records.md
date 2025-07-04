# The `uniffi::Record` derive

The `Record` derive macro exposes a `struct` with named fields over FFI. All types that are
supported as parameter and return types by `#[uniffi::export]` are also supported as field types
here.

It is permitted to use this macro on a type that is also defined in the UDL file, as long as all
field types are UniFFI builtin types; user-defined types might be allowed in the future. You also
have to maintain a consistent field order between the Rust and UDL files (otherwise compilation
will fail).

```rust
#[derive(uniffi::Record)]
pub struct MyRecord {
    pub field_a: String,
    pub field_b: Option<Arc<MyObject>>,
    // Fields can have default values
    #[uniffi(default = "hello")]
    pub greeting: String,
    #[uniffi(default = true)]
    pub some_flag: bool,
    // As with function args, you can omit the default value if you want the default for the type.
    #[uniffi(default)]
    pub other_flag: bool, // defaults to `false`
    // Use other named objects.
    #[uniffi(default)]
    pub optional: OptionalRecord,
}

#[derive(uniffi::Record)]
pub struct OptionalRecord {
    #[uniffi(default)]
    name: String,
}

```

Most types can have default values - all defaults described for [function arguments](./functions.md#default-values) apply here too.
