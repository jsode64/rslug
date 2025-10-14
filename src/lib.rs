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
    truncate: Option<usize>,
    sanitize_replacement: String,
}

impl Default for Slugifier {
    /// Creates a default `Slugifier` instance.
    /// Default separator: `-`
    /// Default lowercase: `true`
    fn default() -> Self {
        Self {
            separator: "-".to_string(),
            to_lowercase: true,
            truncate: None,
            sanitize_replacement: String::new(),
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

    /// Sets the maximum length of the final slug.
    ///
    /// This is a "smart" truncation that will attempt to cut the slug at the
    /// last full word (separator) before the specified length. If the first
    /// word itself is longer than the max length, it will be hard-truncated.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The maximum number of characters for the final slug.
    ///
    /// # Example
    ///
    /// ```
    /// use rslug::Slugifier;
    /// let slugifier = Slugifier::new().truncate(20);
    /// let text = "this is a very long title";
    /// assert_eq!(slugifier.slugify(text), "this-is-a-very-long");
    /// ```
    pub fn truncate(mut self, max_length: usize) -> Self {
        self.truncate = Some(max_length);
        self
    }

    /// Sets the replacement string for illegal filename characters.
    ///
    /// By default, illegal characters are simply removed.
    ///
    /// # Arguments
    ///
    /// * `replacement` - The string to use as a replacement (e.g., "_").
    ///
    /// # Example
    ///
    /// ```
    /// use rslug::Slugifier;
    /// let slugifier = Slugifier::new().sanitize_replacement("_");
    /// let text = "file/with:illegal*chars";
    /// assert_eq!(slugifier.sanitize_filename(text), "file_with_illegal_chars");
    ///
    pub fn sanitize_replacement(mut self, replacement: &str) -> Self {
        self.sanitize_replacement = replacement.to_string();
        self
    }

    /// Sanitizes a string to create a valid and safe filename.
    ///
    /// This method is more conservative than `slugify`. It preserves case and spaces,
    /// and only removes or replaces characters that are illegal in file paths on
    /// major operating systems (e.g., `/`, `\`, `:`, `*`, `?`, `"`).
    ///
    /// # Arguments
    ///
    /// * `filename` - The string to sanitize.
    pub fn sanitize_filename(&self, filename: &str) -> String {
        const ILLEGAL_FILENAME_CHARS: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        let mut sanitized = String::with_capacity(filename.len());
        let mut last_char_was_boundary = false;

        for c in filename.chars() {
            if ILLEGAL_FILENAME_CHARS.contains(&c) || c.is_whitespace() {
                if !last_char_was_boundary {
                    if c.is_whitespace() {
                        sanitized.push(' ');
                    } else {
                        sanitized.push_str(&self.sanitize_replacement);
                    }
                    last_char_was_boundary = true;
                }
            } else {
                sanitized.push(c);
                last_char_was_boundary = false;
            }
        }

        // Trim leading/trailing boundaries which manifest as spaces or replacements
        sanitized.trim().to_string()
    }

    /// Helper function to apply the truncation logic to a mutable slug string.
    pub fn apply_truncation(&self, slug: &mut String) {
        if let Some(max_len) = self.truncate
            && slug.len() > max_len
        {
            if !self.separator.is_empty()
                && let Some(last_sep_index) = slug[..max_len].rfind(&self.separator)
            {
                slug.truncate(last_sep_index);
                return;
            }

            // If no separator was found (or separator is empty), hard-truncate.
            slug.truncate(max_len);
        }
    }

    /// Generates a slug from the given text based on the current configuration.
    ///
    /// # Examples
    /// ```
    /// use rslug::Slugifier;
    ///
    /// let slugifier = Slugifier::new();
    /// let a = slugifier.slugify("Hello, World!");
    /// let b = slugifier.slugify("Slugs are slow, but cool");
    ///
    /// assert_eq!(a, "hello-world");
    /// assert_eq!(b, "slugs-are-slow-but-cool");
    /// ```
    pub fn slugify(&self, text: &str) -> String {
        use any_ascii::any_ascii;

        let text = any_ascii(text);
        let mut slug = String::new();
        let mut found_sep = false;

        for c in text.into_bytes() {
            if c.is_ascii_alphanumeric() {
                // If a separator was found before, add it before the character.
                if found_sep && !slug.is_empty() {
                    slug.push_str(&self.separator);
                }

                // Push the character.
                slug.push(if self.to_lowercase {
                    c.to_ascii_lowercase()
                } else {
                    c
                } as char);

                found_sep = false;
            } else {
                found_sep = true;
            }
        }

        self.apply_truncation(&mut slug);

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
        let mut found_sep = false;

        for &c in text {
            if c.is_ascii_alphanumeric() {
                // If a separator was found before, add it before the character.
                if found_sep && !slug.is_empty() {
                    slug.push_str(&self.separator);
                }

                // Push the character.
                slug.push(if self.to_lowercase {
                    c.to_ascii_lowercase()
                } else {
                    c
                } as char);

                found_sep = false;
            } else {
                found_sep = true;
            }
        }

        self.apply_truncation(&mut slug);

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
        assert_eq!(
            slugify_ascii!(b"--leading and trailing--"),
            "leading-and-trailing"
        );
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

    #[test]
    fn test_truncation_at_word_boundary() {
        let slugifier = Slugifier::new().truncate(20);
        let text = "this is a very long title that should be shortened";
        assert_eq!(slugifier.slugify(text), "this-is-a-very-long");
    }

    #[test]
    fn test_truncation_with_no_separator() {
        let slugifier = Slugifier::new().truncate(20);
        let text = "supercalifragilisticexpialidocious";
        assert_eq!(slugifier.slugify(text), "supercalifragilistic");
    }

    #[test]
    fn test_truncation_not_needed() {
        let slugifier = Slugifier::new().truncate(50);
        let text = "this title is short enough";
        assert_eq!(slugifier.slugify(text), "this-title-is-short-enough");
    }

    #[test]
    fn test_truncation_on_ascii_slug() {
        let slugifier = Slugifier::new().truncate(15);
        let text = b"An ASCII title that is long";
        assert_eq!(slugifier.slugify_ascii(text), "an-ascii-title");
    }

    #[test]
    fn test_sanitize_filename_default() {
        let slugifier = Slugifier::new();
        let unsafe_path = "Report / Section <2>?.docx";
        assert_eq!(
            slugifier.sanitize_filename(unsafe_path),
            "Report Section 2.docx"
        );
    }

    #[test]
    fn test_sanitize_filename_with_replacement() {
        let slugifier = Slugifier::new().sanitize_replacement("_");
        let unsafe_path = "My Docs/Report:Final?.pdf";
        assert_eq!(
            slugifier.sanitize_filename(unsafe_path),
            "My Docs_Report_Final_.pdf"
        );
    }

    #[test]
    fn test_sanitize_filename_no_changes_needed() {
        let slugifier = Slugifier::new();
        let safe_path = "A perfectly valid filename.txt";
        assert_eq!(slugifier.sanitize_filename(safe_path), safe_path);
    }

    #[test]
    fn test_sanitize_filename_empty() {
        let slugifier = Slugifier::new();
        assert_eq!(slugifier.sanitize_filename(""), "");
    }
}
