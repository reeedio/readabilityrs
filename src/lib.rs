//! # ReadabilityRS
//!
//! A Rust port of Mozilla's Readability library for extracting article content from web pages.
//!
//! This library is a faithful port of the [Mozilla Readability](https://github.com/mozilla/readability)
//! JavaScript library, used in Firefox Reader View.
//!
//! ## Example
//!
//! ```rust,no_run
//! use readabilityrs::{Readability, ReadabilityOptions};
//!
//! let html = r#"<html><body><article><h1>Title</h1><p>Content...</p></article></body></html>"#;
//! let url = "https://example.com/article";
//!
//! let options = ReadabilityOptions::default();
//! let readability = Readability::new(html, Some(url), Some(options)).unwrap();
//!
//! if let Some(article) = readability.parse() {
//!     println!("Title: {:?}", article.title);
//!     println!("Content: {:?}", article.content);
//! }
//! ```

mod article;
mod cleaner;
mod constants;
mod content_extractor;
mod dom_utils;
mod error;
mod metadata;
mod options;
mod post_processor;
mod readability;
mod readerable;
mod scoring;
mod utils;

// Public exports
pub use article::Article;
pub use error::{ReadabilityError, Result};
pub use options::ReadabilityOptions;
pub use readability::Readability;
pub use readerable::is_probably_readerable;
