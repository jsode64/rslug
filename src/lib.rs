//! # rslug
//!
//! A simple, fast, and configurable Rust library to create URL-friendly slugs from strings,
//! with great support for Unicode.
//!
//! This library is inspired by popular `slugify` libraries in other languages and aims to be
//! a robust solution for Rust applications.
//!
//! ## Quick Start
//!
//! The easiest way to use the library is with the `slugify!` macro.
//!
//! ```
//! use rslug::slugify;
//!
//! let text = "Hello World! This is a test... 123?";
//! let slug = slugify!(text);
//! assert_eq!(slug, "hello-world-this-is-a-test-123");
//!
//! let unicode_text = "你好世界 & Rust";
//! let unicode_slug = slugify!(unicode_text);
//! assert_eq!(unicode_slug, "nihaoshijie-rust");
//! ```
//!
//! ## Advanced Configuration
//!
//! For more control over the slug generation, you can use the `Slugifier` builder.
//! This allows you to set a custom separator, control case, and more.
//!
//! ```
//! use rslug::Slugifier;
//!
//! // Create a custom slugifier with an underscore separator
//! let slugifier = Slugifier::new()
//!     .separator("_");
//!
//! let text = "Custom Separator Example!";
//! let slug = slugifier.slugify(text);
//!
//! assert_eq!(slug, "custom_separator_example");
//! ```

/// A configurable slug generator.
///
/// Use the builder pattern to create an instance with custom settings.
#[derive(Debug, Clone)]
pub struct Slugifier {
    separator: String,
    to_lowercase: bool,
}

impl Default for Slugifier {
    /// Creates a default `Slugifier` instance.
    /// Default separator: `-`
    /// Default lowercase: `true`
    fn default() -> Self {
        Self {
            separator: "-".to_string(),
            to_lowercase: true,
        }
    }
}

impl Slugifier {
    /// Creates a new `Slugifier` with default settings.
    ///
    /// This is an alias for `Slugifier::default()`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the separator for the slug.
    ///
    /// # Arguments
    ///
    /// * `separator` - The string slice to use as a separator.
    ///
    /// # Example
    ///
    /// ```
    /// use rslug::Slugifier;
    /// let slugifier = Slugifier::new().separator("_");
    /// assert_eq!(slugifier.slugify("hello world"), "hello_world");
    /// ```
    pub fn separator(mut self, separator: &str) -> Self {
        self.separator = separator.to_string();
        self
    }

    /// Sets whether the output slug should be lowercased.
    ///
    /// # Arguments
    ///
    /// * `to_lowercase` - A boolean indicating if the slug should be lowercased.
    ///
    /// # Example
    ///
    /// ```
    /// use rslug::Slugifier;
    /// let slugifier = Slugifier::new().to_lowercase(false);
    /// assert_eq!(slugifier.slugify("Hello World"), "Hello-World");
    /// ```
    pub fn to_lowercase(mut self, lowercase: bool) -> Self {
        self.to_lowercase = lowercase;
        self
    }

    /// Generates a slug from the given text based on the current configuration.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice to convert into a slug.
    pub fn slugify(&self, text: &str) -> String {
        // Transliterate Unicode characters to their ASCII equivalent.
        let text = any_ascii::any_ascii(text);

        let mut slug = String::with_capacity(text.len());
        // Start by assuming we are at a boundary, to prevent leading separators.
        let mut last_char_was_separator = true;

        for c in text.chars() {
            // Check if the character is one we want to keep.
            if c.is_alphanumeric() {
                if self.to_lowercase {
                    slug.push(c.to_ascii_lowercase());
                } else {
                    slug.push(c);
                }
                last_char_was_separator = false;
            } else {
                // If the last char wasn't a separator, we can add one now.
                if !last_char_was_separator {
                    slug.push_str(&self.separator);
                    last_char_was_separator = true;
                }
            }
        }

        // If the slug ends with a separator, trim it.
        if slug.ends_with(&self.separator) {
            slug.truncate(slug.len() - self.separator.len());
        }

        slug
    }

    /// Generates a slug from the given ASCII text.
    ///
    /// # Examples
    /// ```
    /// use rslug::Slugifier;
    ///
    /// let slugifier = Slugifier::new();
    /// let a = slugifier.slugify_ascii(b"Hello, World!");
    /// let b = slugifier.slugify_ascii(b"Slugs are slow, but cool");
    ///
    /// assert_eq!(a, "hello-world");
    /// assert_eq!(b, "slugs-are-slow-but-cool");
    /// ```
    pub fn slugify_ascii(&self, text: &[u8]) -> String {
        let mut slug = String::new();
        let mut just_sep = true;

        for &c in text {
            if c.is_ascii_alphanumeric() {
                slug.push(if self.to_lowercase {
                    c.to_ascii_lowercase()
                } else {
                    c
                } as char);
                just_sep = false;
            } else if !just_sep {
                // Need separator.
                slug.push_str(&self.separator);
                just_sep = true;
            }
        }

        // Trim any ending separator.
        if slug.ends_with(&self.separator) {
            slug.truncate(slug.len() - self.separator.len());
        }

        slug
    }
}

/// A convenient macro to slugify a string with default settings.
///
/// This is a shortcut for `Slugifier::new().slugify(text)`.
#[macro_export]
macro_rules! slugify {
    ($text:expr) => {
        $crate::Slugifier::new().slugify($text)
    };
}

#[macro_export]
macro_rules! slugify_ascii {
    ($text:expr) => {
        $crate::Slugifier::new().slugify_ascii($text)
    };
}

// Unit tests for the library
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slug() {
        assert_eq!(slugify!("Hello World"), "hello-world");
    }

    #[test]
    fn test_with_punctuation() {
        assert_eq!(
            slugify!("Hello, World! This is a test... 123?"),
            "hello-world-this-is-a-test-123"
        );
    }

    #[test]
    fn test_unicode_slug() {
        assert_eq!(slugify!("你好世界 & Rust"), "nihaoshijie-rust");
    }

    #[test]
    fn test_leading_and_trailing_hyphens() {
        assert_eq!(slugify!("--leading and trailing--"), "leading-and-trailing");
    }

    #[test]
    fn test_multiple_hyphens() {
        assert_eq!(slugify!("multiple---hyphens"), "multiple-hyphens");
    }

    #[test]
    fn test_custom_separator() {
        let slugifier = Slugifier::new().separator("_");
        assert_eq!(slugifier.slugify("custom separator"), "custom_separator");
    }

    #[test]
    fn test_no_lowercase() {
        let slugifier = Slugifier::new().to_lowercase(false);
        assert_eq!(slugifier.slugify("No Lowercase"), "No-Lowercase");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(slugify!(""), "");
    }

    #[test]
    fn test_ascii_only() {
        let slugifier = Slugifier::new();
        assert_eq!(slugifier.slugify_ascii(b"Hello, World!"), "hello-world");
        assert_eq!(
            slugifier.slugify_ascii(b"Slugs are slow, but cool"),
            "slugs-are-slow-but-cool"
        );
    }

    #[test]
    fn test_slugify_ascii_no_lowercase() {
        let slugifier = Slugifier::new().to_lowercase(false);
        assert_eq!(slugifier.slugify_ascii(b"Keep-Case"), "Keep-Case");
    }
}
