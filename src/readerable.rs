//! Quick readability check without full parsing.

use scraper::{Html, Selector};

/// Options for the isProbablyReaderable check
#[derive(Debug, Clone)]
pub struct ReaderableOptions {
    /// Minimum content length to consider (default: 140)
    pub min_content_length: usize,
    /// Minimum score to consider readerable (default: 20)
    pub min_score: f64,
}

impl Default for ReaderableOptions {
    fn default() -> Self {
        Self {
            min_content_length: 140,
            min_score: 20.0,
        }
    }
}

/// Quick check to determine if a document is likely to be readerable
/// Returns true if Readability.parse() is likely to succeed
pub fn is_probably_readerable(html: &str, options: Option<ReaderableOptions>) -> bool {
    let options = options.unwrap_or_default();
    let document = Html::parse_document(html);

    // TODO: Implement full isProbablyReaderable logic
    // For now, just do a basic check

    let p_selector = Selector::parse("p, pre, article").unwrap();
    let paragraphs: Vec<_> = document.select(&p_selector).collect();

    if paragraphs.is_empty() {
        return false;
    }

    let mut score = 0.0;

    for p in paragraphs {
        let text = p.text().collect::<String>();
        let text_len = text.trim().len();

        if text_len < options.min_content_length {
            continue;
        }

        score += ((text_len - options.min_content_length) as f64).sqrt();

        if score > options.min_score {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_probably_readerable() {
        let html = r#"
            <html>
                <body>
                    <article>
                        <p>This is a long enough paragraph that should make the content readerable.
                        It has sufficient content to pass the minimum threshold check. Adding more text here to ensure
                        we definitely exceed the 140 character minimum requirement for each paragraph element.</p>
                        <p>Another paragraph with more content to increase the score. This paragraph also needs to be
                        long enough to contribute to the overall readability score calculation and help us pass the test.</p>
                    </article>
                </body>
            </html>
        "#;

        assert!(is_probably_readerable(html, None));
    }

    #[test]
    fn test_not_readerable() {
        let html = r#"
            <html>
                <body>
                    <p>Short</p>
                </body>
            </html>
        "#;

        assert!(!is_probably_readerable(html, None));
    }
}
