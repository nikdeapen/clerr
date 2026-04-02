use crate::Severity::Info;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};
use unicode_width::UnicodeWidthStr;

/// A properties entry.
///
/// # Display
///
/// ```text
///     file:      /etc/config.yml
///     expected:  utf-8
///     found:     binary
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Properties {
    properties: Vec<(String, String)>,
}

impl Properties {
    //! Properties

    /// Gets the properties.
    pub fn properties(&self) -> &[(String, String)] {
        self.properties.as_slice()
    }

    /// Adds the property.
    pub fn add_property<S0, S1>(&mut self, name: S0, value: S1)
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.properties.push((name.into(), value.into()));
    }

    /// Adds the property.
    #[must_use]
    pub fn with_property<S0, S1>(mut self, name: S0, value: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.add_property(name, value);
        self
    }
}

impl From<&Properties> for Vec<ColoredString> {
    fn from(properties: &Properties) -> Vec<ColoredString> {
        let properties: &[(String, String)] = properties.properties.as_slice();
        let max_width: usize = properties
            .iter()
            .map(|(name, _)| name.width())
            .max()
            .unwrap_or(0);
        let count: usize = properties.len();
        let color: Color = Info.color();
        let max_spaces: String = util::char_count(' ', max_width + 2);
        let mut result: Vec<ColoredString> = Vec::with_capacity(count * 6);
        for (i, (property, value)) in properties.iter().enumerate() {
            let padding: usize = max_width - property.width() + 2;
            result.push("    ".normal());
            result.push(property.color(color));
            result.push(":".color(color));
            result.push(max_spaces[..padding].normal());
            result.push(value.as_str().normal());
            if i + 1 < count {
                result.push("\n".normal());
            }
        }
        result
    }
}

impl From<Properties> for Vec<ColoredString> {
    fn from(props: Properties) -> Vec<ColoredString> {
        Vec::from(&props)
    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<ColoredString> = Vec::from(self);
        for string in &strings {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Properties;
    use colored::Colorize;

    #[test]
    fn display() {
        let properties: Properties = Properties::default()
            .with_property("one", "two")
            .with_property("three", "four")
            .with_property("five", "six");

        let result: String = properties.to_string();
        let expected: String = [
            "    ".normal(),
            "one".bright_blue(),
            ":".bright_blue(),
            "    ".normal(),
            "two".normal(),
            "\n".normal(),
            "    ".normal(),
            "three".bright_blue(),
            ":".bright_blue(),
            "  ".normal(),
            "four".normal(),
            "\n".normal(),
            "    ".normal(),
            "five".bright_blue(),
            ":".bright_blue(),
            "   ".normal(),
            "six".normal(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn display_unicode() {
        let properties: Properties = Properties::default()
            .with_property("one", "two")
            .with_property("你好", "hello");

        let result: String = properties.to_string();
        let expected: String = [
            "    ".normal(),
            "one".bright_blue(),
            ":".bright_blue(),
            "   ".normal(),
            "two".normal(),
            "\n".normal(),
            "    ".normal(),
            "你好".bright_blue(),
            ":".bright_blue(),
            "  ".normal(),
            "hello".normal(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(result, expected);
    }
}
