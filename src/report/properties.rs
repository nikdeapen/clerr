use crate::Severity::Info;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};

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
        let max_len: usize = properties
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);
        let count: usize = properties.len();
        let color: Color = Info.color();
        let max_spaces: String = util::char_count(' ', max_len + 2);
        let mut result: Vec<ColoredString> = Vec::with_capacity(count * 6);
        for (i, (property, value)) in properties.iter().enumerate() {
            result.push("    ".normal());
            result.push(property.color(color));
            result.push(":".color(color));
            result.push(max_spaces[..(max_len - property.len() + 2)].normal());
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
    use crate::{Code, Properties, Report};

    #[test]
    fn properties() {
        let properties: Properties = Properties::default()
            .with_property("one", "two")
            .with_property("three", "four")
            .with_property("five", "six");
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::from(code).with_entry(properties);
        println!("properties:\n{}\n", report);
    }
}
