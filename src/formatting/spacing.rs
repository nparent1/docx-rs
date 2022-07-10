#![allow(dead_code)]

use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

use super::line_rule::LineRule;

/// Spacing
/// 100 percent.
/// line is 240-based.
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let sz = Size::from(42usize);
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:spacing")]
pub struct Spacing {
    /// Spacing Above Paragraph In Line Units
    #[xml(attr = "w:beforeLines")]
    pub before_lines: Option<usize>,
    /// Spacing Above Paragraph
    #[xml(attr = "w:before")]
    pub before: Option<usize>,
    /// Automatically Determine Spacing Above Paragraph
    #[xml(attr = "w:beforeAutospacing")]
    pub before_auto_spacing: Option<bool>,
    #[xml(attr = "w:afterLines")]
    pub after_lines: Option<usize>,
    #[xml(attr = "w:after")]
    pub after: Option<usize>,
    #[xml(attr = "w:afterAutospacing")]
    pub after_auto_spacing: Option<bool>,
    #[xml(attr = "w:line")]
    pub line: Option<usize>,
    #[xml(attr = "w:lineRule")]
    pub line_rule: Option<LineRule>,
}

impl Spacing {
    fn before<T>(mut self, val: T) -> Self
    where
        usize: From<T>,
    {
        self.before = Some(val.into());
        self.before_lines = self.before;
        self
    }
    fn after<T>(mut self, val: T) -> Self
    where
        usize: From<T>,
    {
        self.after = Some(val.into());
        self.after_lines = self.before;
        self
    }

    __setter!(line: Option<usize>);
    __setter!(line_rule: Option<LineRule>);
}

__xml_test_suites!(
    Spacing,
    Spacing::default()
        .before(50usize)
        .after(50usize)
        .line(384usize)
        .line_rule(LineRule::Auto),
    r#"<w:spacing w:beforeLines="50" w:before="50" w:afterLines="50" w:after="50" w:line="384" w:lineRule="auto"/>"#,
);
