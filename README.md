## About

Trash is a Rust library that provides functionality to move files to the operating system's Recycle Bin.

The library currently supports Windows and Linux. Contribution to MacOS support would be very much apprectiated.

## Usage

```rust
extern crate trash;
use std::fs::File;
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