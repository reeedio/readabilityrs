//! Post-processing functions for article content after extraction.
//!
//! This module implements Mozilla's _prepArticle pipeline, which cleans
//! the extracted article content by removing unwanted elements.

use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};

/// Remove nav-heavy wrappers by descending into content-like children.
fn unwrap_nav_wrappers(html: &str) -> String {
    static WRAPPER_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"(?is)<div[^>]+class="[^"]*(?:navbar|nav|menu|sidebar|widget|header)[^"]*"[^>]*>.*?</div>"#,
        )
        .unwrap()
    });

    WRAPPER_REGEX.replace_all(html, "").to_string()
}

/// Remove the title element from the article content if it matches the extracted title.
///
/// Finds the first h1 or h2 element whose text content matches the given title
/// (after normalization) and removes it from the HTML.
///
/// # Arguments
/// * `html` - The article HTML content
/// * `title` - The extracted article title to match against
///
/// # Returns
/// The HTML with the matching title element removed, or the original HTML if no match found
pub fn remove_title_from_content(html: &str, title: &str) -> String {
    let doc = Html::parse_fragment(html);

    // Normalize the title for comparison
    let normalized_title = normalize_text(title);
    if normalized_title.is_empty() {
        return html.to_string();
    }

    // Try to find h1 or h2 elements that match the title
    let selector = Selector::parse("h1, h2").unwrap();

    for element in doc.select(&selector) {
        let element_text: String = element.text().collect();
        let normalized_element_text = normalize_text(&element_text);

        // Check if the heading text matches the title (exact or near match)
        if titles_match(&normalized_title, &normalized_element_text) {
            // Get the outer HTML of this element and remove it
            let element_html = element.html();
            // Replace only the first occurrence
            if let Some(pos) = html.find(&element_html) {
                let mut result = String::with_capacity(html.len());
                result.push_str(&html[..pos]);
                result.push_str(&html[pos + element_html.len()..]);
                return result;
            }
        }
    }

    html.to_string()
}

/// Normalize text for title comparison: lowercase, collapse whitespace, trim
fn normalize_text(text: &str) -> String {
    static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    WHITESPACE_REGEX
        .replace_all(text.trim(), " ")
        .to_lowercase()
}

/// Check if two normalized titles match (exact or one contains the other)
fn titles_match(title1: &str, title2: &str) -> bool {
    if title1 == title2 {
        return true;
    }

    // Allow for slight variations - one contains the other
    // This handles cases where the h1 might have extra text or vice versa
    let len1 = title1.len();
    let len2 = title2.len();

    // If lengths are similar (within 20%), check if one contains the other
    if len1 > 0 && len2 > 0 {
        let ratio = len1.min(len2) as f64 / len1.max(len2) as f64;
        if ratio > 0.8 && (title1.contains(title2) || title2.contains(title1)) {
            return true;
        }
    }

    false
}

/// Prepare extracted article content for final output
///
/// This implements Mozilla's _prepArticle() pipeline using regex-based cleaning
pub fn prep_article(html: &str) -> String {
    let mut html = html.to_string();

    // Unwrap nav wrappers before removing elements
    html = unwrap_nav_wrappers(&html);

    // Step 1: Remove unwanted elements
    html = remove_unwanted_elements(&html);

    // Step 2: Remove share buttons and social widgets
    html = remove_share_elements(&html);

    // Step 2b: Remove navigation lists/menus
    html = remove_navigation_elements(&html);

    // Step 3: Remove empty paragraphs
    html = remove_empty_paragraphs(&html);

    html
}

/// Remove unwanted elements that are never part of article content
///
/// Removes: forms, fieldsets, footer, aside, object, embed, iframe,
/// input, textarea, select, button
fn remove_unwanted_elements(html: &str) -> String {
    let mut result = html.to_string();
    let tags = vec![
        ("form", r"(?is)<form\b[^>]*?>.*?</form>"),
        ("fieldset", r"(?is)<fieldset\b[^>]*?>.*?</fieldset>"),
        ("footer", r"(?is)<footer\b[^>]*?>.*?</footer>"),
        ("aside", r"(?is)<aside\b[^>]*?>.*?</aside>"),
        ("object", r"(?is)<object\b[^>]*?>.*?</object>"),
        (
            "embed",
            r"(?is)<embed\b[^>]*?>.*?</embed>|<embed\b[^>]*?/?>",
        ),
        ("iframe", r"(?is)<iframe\b[^>]*?>.*?</iframe>"),
        (
            "input",
            r"(?is)<input\b[^>]*?>.*?</input>|<input\b[^>]*?/?>",
        ),
        ("textarea", r"(?is)<textarea\b[^>]*?>.*?</textarea>"),
        ("select", r"(?is)<select\b[^>]*?>.*?</select>"),
        ("button", r"(?is)<button\b[^>]*?>.*?</button>"),
        ("link", r"(?is)<link\b[^>]*?>.*?</link>|<link\b[^>]*?/?>"),
    ];

    for (_name, pattern) in tags {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").to_string();
    }

    result
}

/// Remove share buttons and social widgets
///
/// Removes elements with "share" or "social" in their class/id
fn remove_share_elements(html: &str) -> String {
    let mut result = html.to_string();
    let tags = vec!["div", "span", "aside", "section"];
    let keywords = vec!["share", "social", "sharedaddy"];

    for tag in &tags {
        for keyword in &keywords {
            let class_pattern = format!(
                r#"(?is)<{tag}\b[^>]*?class="[^"]*?{keyword}[^"]*?"[^>]*?>.*?</{tag}>"#
            );
            let re = Regex::new(&class_pattern).unwrap();
            result = re.replace_all(&result, "").to_string();

            let id_pattern = format!(
                r#"(?is)<{tag}\b[^>]*?id="[^"]*?{keyword}[^"]*?"[^>]*?>.*?</{tag}>"#
            );
            let re = Regex::new(&id_pattern).unwrap();
            result = re.replace_all(&result, "").to_string();
        }
    }

    result
}

/// Remove navigation lists and menu sections
fn remove_navigation_elements(html: &str) -> String {
    let mut result = html.to_string();

    static NAV_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?is)<nav\b[^>]*?>.*?</nav>").unwrap());
    result = NAV_REGEX.replace_all(&result, "").to_string();

    let tags = vec!["div", "section", "ul", "ol"];
    let keywords = vec!["nav", "navbar", "menu", "breadcrumbs"];

    for tag in &tags {
        for keyword in &keywords {
            let class_pattern = format!(
                r#"(?is)<{tag}\b[^>]*?class="[^"]*?{keyword}[^"]*?"[^>]*?>.*?</{tag}>"#
            );
            let re = Regex::new(&class_pattern).unwrap();
            result = re.replace_all(&result, "").to_string();

            let id_pattern = format!(
                r#"(?is)<{tag}\b[^>]*?id="[^"]*?{keyword}[^"]*?"[^>]*?>.*?</{tag}>"#
            );
            let re = Regex::new(&id_pattern).unwrap();
            result = re.replace_all(&result, "").to_string();
        }
    }

    result
}

/// Remove empty paragraphs (paragraphs with no text and no media elements)
fn remove_empty_paragraphs(html: &str) -> String {
    static EMPTY_P_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"<p[^>]*?>\s*</p>").unwrap());

    let mut html = html.to_string();
    loop {
        let new_html = EMPTY_P_REGEX.replace_all(&html, "").to_string();
        if new_html == html {
            break;
        }
        html = new_html;
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_unwanted_elements() {
        let html = r#"
            <article>
                <h1>Title</h1>
                <p>Content</p>
                <footer>Footer content</footer>
                <form><input type="text"></form>
            </article>
        "#;

        let cleaned = remove_unwanted_elements(html);

        assert!(cleaned.contains("<h1>Title</h1>"));
        assert!(cleaned.contains("<p>Content</p>"));
        assert!(!cleaned.contains("<footer"));
        assert!(!cleaned.contains("<form"));
    }

    #[test]
    fn test_remove_empty_paragraphs() {
        let html = r#"
            <div>
                <p>Good paragraph</p>
                <p></p>
                <p>   </p>
                <p>Another good one</p>
            </div>
        "#;

        let cleaned = remove_empty_paragraphs(html);

        assert!(cleaned.contains("<p>Good paragraph</p>"));
        assert!(cleaned.contains("<p>Another good one</p>"));
        assert!(!cleaned.contains("<p></p>"));
        assert!(!cleaned.contains("<p>   </p>"));
    }

    #[test]
    fn test_remove_share_elements() {
        let html = r##"
            <div>
                <p>Article content</p>
                <div class="share-buttons">
                    <a href="#">Share</a>
                </div>
                <div class="social-media">
                    <a href="#">Follow</a>
                </div>
            </div>
        "##;

        let cleaned = remove_share_elements(html);

        assert!(cleaned.contains("<p>Article content</p>"));
        assert!(!cleaned.contains("share-buttons"));
        assert!(!cleaned.contains("social-media"));
    }

    #[test]
    fn test_remove_navigation_elements() {
        let html = r##"
            <div>
                <nav>Nav content</nav>
                <div class="navbar menu">
                    <ul>
                        <li><a href="#">Home</a></li>
                        <li><a href="#">About</a></li>
                    </ul>
                </div>
                <p>Main article paragraph</p>
            </div>
        "##;

        let cleaned = remove_navigation_elements(html);

        assert!(cleaned.contains("<p>Main article paragraph</p>"));
        assert!(!cleaned.contains("<nav>"));
        assert!(!cleaned.contains("navbar"));
    }

    #[test]
    fn test_prep_article_full() {
        let html = r#"
            <article>
                <h1>Article Title</h1>
                <p>First paragraph</p>
                <p></p>
                <footer>Page footer</footer>
                <p>Second paragraph</p>
                <div class="share">Share this!</div>
                <form><input/></form>
            </article>
        "#;

        let cleaned = prep_article(html);

        assert!(cleaned.contains("<h1>Article Title</h1>"));
        assert!(cleaned.contains("<p>First paragraph</p>"));
        assert!(cleaned.contains("<p>Second paragraph</p>"));
        assert!(!cleaned.contains("<footer"));
        assert!(!cleaned.contains("<form"));
        assert!(!cleaned.contains("<p></p>"));
    }

    #[test]
    fn test_remove_title_from_content_h1() {
        let html = r#"
            <article>
                <h1>Article Title</h1>
                <p>First paragraph</p>
                <p>Second paragraph</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "Article Title");

        assert!(!cleaned.contains("<h1>"));
        assert!(!cleaned.contains("Article Title"));
        assert!(cleaned.contains("<p>First paragraph</p>"));
        assert!(cleaned.contains("<p>Second paragraph</p>"));
    }

    #[test]
    fn test_remove_title_from_content_h2() {
        let html = r#"
            <article>
                <h2>Article Title</h2>
                <p>First paragraph</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "Article Title");

        assert!(!cleaned.contains("<h2>"));
        assert!(!cleaned.contains("Article Title"));
        assert!(cleaned.contains("<p>First paragraph</p>"));
    }

    #[test]
    fn test_remove_title_from_content_with_whitespace() {
        let html = r#"
            <article>
                <h1>  Article   Title  </h1>
                <p>Content</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "Article Title");

        assert!(!cleaned.contains("<h1>"));
        assert!(cleaned.contains("<p>Content</p>"));
    }

    #[test]
    fn test_remove_title_from_content_case_insensitive() {
        let html = r#"
            <article>
                <h1>ARTICLE TITLE</h1>
                <p>Content</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "Article Title");

        assert!(!cleaned.contains("<h1>"));
        assert!(cleaned.contains("<p>Content</p>"));
    }

    #[test]
    fn test_remove_title_from_content_no_match() {
        let html = r#"
            <article>
                <h1>Different Title</h1>
                <p>Content</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "Article Title");

        // Should preserve the h1 when no match
        assert!(cleaned.contains("<h1>Different Title</h1>"));
        assert!(cleaned.contains("<p>Content</p>"));
    }

    #[test]
    fn test_remove_title_from_content_empty_title() {
        let html = r#"
            <article>
                <h1>Article Title</h1>
                <p>Content</p>
            </article>
        "#;

        let cleaned = remove_title_from_content(html, "");

        // Should preserve everything when title is empty
        assert!(cleaned.contains("<h1>Article Title</h1>"));
        assert!(cleaned.contains("<p>Content</p>"));
    }
}
