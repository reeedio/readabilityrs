//! Article data structure representing the parsed output.

use serde::{Deserialize, Serialize};

/// Represents a successfully parsed article
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    /// Article title
    pub title: Option<String>,

    /// HTML content of the processed article
    pub content: Option<String>,

    /// Plain text content with all HTML tags removed
    pub text_content: Option<String>,

    /// Length of the article in characters
    pub length: usize,

    /// Article description or short excerpt from the content
    pub excerpt: Option<String>,

    /// Author metadata
    pub byline: Option<String>,

    /// Content direction (ltr, rtl, auto)
    pub dir: Option<String>,

    /// Name of the site
    pub site_name: Option<String>,

    /// Content language
    pub lang: Option<String>,

    /// Published time
    pub published_time: Option<String>,

    /// Raw HTML before any post-processing
    pub raw_content: Option<String>,
}

impl Default for Article {
    fn default() -> Self {
        Self {
            title: None,
            content: None,
            text_content: None,
            length: 0,
            excerpt: None,
            byline: None,
            dir: None,
            site_name: None,
            lang: None,
            published_time: None,
            raw_content: None,
        }
    }
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}
