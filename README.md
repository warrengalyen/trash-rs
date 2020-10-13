## About

The `trash` is a Rust library for moving files and folders to the operating system's Recycle Bin or Trash or Rubbish Bin or what have you

The library supports Windows, Mac and Linux.

## Usage

```toml
# In Cargo.toml
[dependencies]
trash = "1.0.1"
```

```rust
// In main.rs
use std::fs::File;
use trash;
fn main() {
    // Let's create and remove a single file
    File::create("remove-me").unwrap();
    trash::remove("remove-me").unwrap();
    assert!(File::open("remove-me").is_err());

    // Now let's remove multiple files at once
    let the_others = ["remove-me-too", "dont-forget-about-me-either"];
    for name in the_others.iter() {
        File::create(name).unwrap();
    }
    trash::remove_all(&the_others).unwrap();
    for name in the_others.iter() {
        assert!(File::open(name).is_err());
    }
}
```