//! Configuration options for Readability parsing.

use regex::Regex;

/// Configuration options for the Readability parser
#[derive(Debug, Clone)]
pub struct ReadabilityOptions {
    /// Enable debug logging (default: false)
    pub debug: bool,

    /// Maximum number of elements to parse. 0 means no limit (default: 0)
    pub max_elems_to_parse: usize,

    /// Number of top candidates to consider when analyzing content (default: 5)
    pub nb_top_candidates: usize,

    /// Minimum number of characters required for article content (default: 500)
    pub char_threshold: usize,

    /// CSS classes to preserve during cleaning (default: ["page"])
    pub classes_to_preserve: Vec<String>,

    /// Keep all CSS classes (default: false)
    pub keep_classes: bool,

    /// Disable JSON-LD metadata extraction (default: false)
    pub disable_json_ld: bool,

    /// Custom regex for allowed video URLs (default: common video platforms)
    pub allowed_video_regex: Option<Regex>,

    /// Modifier for link density scoring (default: 0)
    pub link_density_modifier: f64,
}

impl Default for ReadabilityOptions {
    fn default() -> Self {
        Self {
            debug: false,
            max_elems_to_parse: 0,
            nb_top_candidates: 5,
            char_threshold: 500,
            classes_to_preserve: vec!["page".to_string()],
            keep_classes: false,
            disable_json_ld: false,
            allowed_video_regex: None,
            link_density_modifier: 0.0,
        }
    }
}

impl ReadabilityOptions {
    /// Creates a new builder for ReadabilityOptions
    pub fn builder() -> ReadabilityOptionsBuilder {
        ReadabilityOptionsBuilder::default()
    }
}

/// Builder for ReadabilityOptions
#[derive(Default)]
pub struct ReadabilityOptionsBuilder {
    debug: Option<bool>,
    max_elems_to_parse: Option<usize>,
    nb_top_candidates: Option<usize>,
    char_threshold: Option<usize>,
    classes_to_preserve: Option<Vec<String>>,
    keep_classes: Option<bool>,
    disable_json_ld: Option<bool>,
    allowed_video_regex: Option<Regex>,
    link_density_modifier: Option<f64>,
}

impl ReadabilityOptionsBuilder {
    /// Enable or disable debug logging
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
    }

    /// Set maximum number of elements to parse
    pub fn max_elems_to_parse(mut self, max: usize) -> Self {
        self.max_elems_to_parse = Some(max);
        self
    }

    /// Set number of top candidates to consider
    pub fn nb_top_candidates(mut self, nb: usize) -> Self {
        self.nb_top_candidates = Some(nb);
        self
    }

    /// Set character threshold
    pub fn char_threshold(mut self, threshold: usize) -> Self {
        self.char_threshold = Some(threshold);
        self
    }

    /// Set classes to preserve
    pub fn classes_to_preserve(mut self, classes: Vec<String>) -> Self {
        self.classes_to_preserve = Some(classes);
        self
    }

    /// Keep all CSS classes
    pub fn keep_classes(mut self, keep: bool) -> Self {
        self.keep_classes = Some(keep);
        self
    }

    /// Disable JSON-LD extraction
    pub fn disable_json_ld(mut self, disable: bool) -> Self {
        self.disable_json_ld = Some(disable);
        self
    }

    /// Set allowed video regex
    pub fn allowed_video_regex(mut self, regex: Regex) -> Self {
        self.allowed_video_regex = Some(regex);
        self
    }

    /// Set link density modifier
    pub fn link_density_modifier(mut self, modifier: f64) -> Self {
        self.link_density_modifier = Some(modifier);
        self
    }

    /// Build the ReadabilityOptions
    pub fn build(self) -> ReadabilityOptions {
        let defaults = ReadabilityOptions::default();
        ReadabilityOptions {
            debug: self.debug.unwrap_or(defaults.debug),
            max_elems_to_parse: self
                .max_elems_to_parse
                .unwrap_or(defaults.max_elems_to_parse),
            nb_top_candidates: self.nb_top_candidates.unwrap_or(defaults.nb_top_candidates),
            char_threshold: self.char_threshold.unwrap_or(defaults.char_threshold),
            classes_to_preserve: self
                .classes_to_preserve
                .unwrap_or(defaults.classes_to_preserve),
            keep_classes: self.keep_classes.unwrap_or(defaults.keep_classes),
            disable_json_ld: self.disable_json_ld.unwrap_or(defaults.disable_json_ld),
            allowed_video_regex: self.allowed_video_regex.or(defaults.allowed_video_regex),
            link_density_modifier: self
                .link_density_modifier
                .unwrap_or(defaults.link_density_modifier),
        }
    }
}
