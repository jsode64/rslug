//! This is an example binary to demonstrate the usage of the `slugify-rs` library.
//! It shows both the simple `slugify!` macro and the configurable `Slugifier` struct.

// We need to bring the library crate into scope. The name is determined by `[package].name` in Cargo.toml.
use slugify_rs::{Slugifier, slugify};

fn main() {
    println!("--- Basic Usage with slugify! macro ---");

    let text1 = "Hello World! This is a test... 123?";
    let slug1 = slugify!(text1);
    println!("Original: '{}'", text1);
    println!("Slug    : '{}'\n", slug1);
    assert_eq!(slug1, "hello-world-this-is-a-test-123");

    let text2 = "你好世界 & Rust (Unicode Example)";
    let slug2 = slugify!(text2);
    println!("Original: '{}'", text2);
    println!("Slug    : '{}'\n", slug2);
    assert_eq!(slug2, "nihaoshijie-rust-unicode-example");

    let text3 = "--- lots of --- separators ---";
    let slug3 = slugify!(text3);
    println!("Original: '{}'", text3);
    println!("Slug    : '{}'\n", slug3);
    assert_eq!(slug3, "lots-of-separators");

    println!("--- Advanced Usage with Slugifier Builder ---");

    // Create a custom slugifier that uses an underscore `_` as a separator
    // and keeps the original casing.
    let custom_slugifier = Slugifier::new().separator("_").to_lowercase(false);

    let text4 = "Example With Custom_Settings!";
    let slug4 = custom_slugifier.slugify(text4);
    println!("Original: '{}'", text4);
    println!("Slug    : '{}' (using custom slugifier)\n", slug4);
    assert_eq!(slug4, "Example_With_Custom_Settings");
}
