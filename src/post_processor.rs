//! Post-processing functions for article content after extraction.
//!
//! This module implements Mozilla's _prepArticle pipeline, which cleans
//! the extracted article content by removing unwanted elements.

use once_cell::sync::Lazy;
use regex::Regex;

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
}
