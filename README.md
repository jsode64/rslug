# rslug 🐌

A simple, fast, and configurable Rust library for creating URL-friendly slugs from strings, with robust support for Unicode.

This library is inspired by popular [slugify](https://www.npmjs.com/package/slugify) utilities in other languages and aims to be a lightweight and ergonomic solution for any Rust application.

## Key Features

- Simple API: Get started instantly with the `slugify!` macro.
- Highly Configurable: Use the `Slugifier` builder for custom separators, case control, and more.
- Unicode Support: Non-ASCII characters are intelligently transliterated to their ASCII equivalents.
- Performance-Optimized ASCII: Includes a `slugify_ascii` method for maximum speed with ASCII-only input.
- Lightweight: `rslug` is tiny and has minimal dependencies.
- Fast: Built for performance, ideal for web servers and static site generators.
- Smart Truncation: Limits slug length without cutting words in half.

## Installation

Add `rslug` to your `Cargo.toml` file:

```bash
[dependencies]
rslug = "0.3.0"  # Check for the latest version on crates.io
```

Or

```bash
cargo add rslug
```

## Quick Start

The easiest way to generate a slug is with the `slugify!` macro, which uses the default settings (hyphen separator, lowercase output).

```rust
use rslug::slugify;

// Basic usage
let text = "Hello World! This is a test... 123?";
let slug = slugify!(text);
assert_eq!(slug, "hello-world-this-is-a-test-123");

// Unicode support
let unicode_text = "你好世界 & Rust";
let unicode_slug = slugify!(unicode_text);
assert_eq!(unicode_slug, "nihaoshijie-rust");
```

## Advanced Configuration

For more control over the output, create a `Slugifier` instance using its builder pattern. This allows you to customize the slug generation rules.

```rust
use rslug::Slugifier;

// Example 1: Using an underscore as a separator
let slugifier_underscore = Slugifier::new()
    .separator("_");

let text = "Custom Separator Example!";
let slug = slugifier_underscore.slugify(text);
assert_eq!(slug, "custom_separator_example");


// Example 2: Preserving the original case
let slugifier_case_sensitive = Slugifier::new()
    .to_lowercase(false);

let text_with_case = "Keep The Case";
let slug_with_case = slugifier_case_sensitive.slugify(text_with_case);
assert_eq!(slug_with_case, "Keep-The-Case");

let slugifier_truncate = Slugifier::new()
    .truncate(20);
    
let text_to_truncate = "this is a very long title that should be shortened";
let slug_with_truncate = slugifier_truncate.slugify(text_to_truncate);
assert_eq!(slug_with_truncate, "this-is-a-very-long");
```

## Performance: ASCII-Only Slugs

For performance-critical scenarios where you can guarantee the input is ASCII, you can use the `slugify_ascii` method. It operates directly on bytes (`&[u8]`) and avoids the overhead of Unicode transliteration, making it significantly faster.

```rust
use rslug::Slugifier;

let slugifier = Slugifier::new();

// Note the `b` prefix for a byte string literal
let ascii_text = b"This is ASCII-only, so it can be faster!";
let slug = slugifier.slugify_ascii(ascii_text);

assert_eq!(slug, "this-is-ascii-only-so-it-can-be-faster");
```

## Contributing 

Contributions are welcome! If you have a feature request, find a bug, or want to improve the code, please feel free to open an issue or submit a pull request.

## License

This library is open-source and available under the [MIT License](https://github.com/ezrantn/rslug/blob/main/LICENSE).