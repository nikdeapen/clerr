use crate::Severity::Info;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};

/// A properties entry.
///
/// # Display
///
/// ```text
///     first:  value
///     second: another value
///     third:  third value
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

impl From<Properties> for Vec<ColoredString> {
    fn from(props: Properties) -> Vec<ColoredString> {
        let max_len: usize = props
            .properties
            .iter()
            .map(|(p, _)| p.len())
            .max()
            .unwrap_or(0);
        let len: usize = props.properties.len();
        let color: Color = Info.color();
        let mut result: Vec<ColoredString> = Vec::new();
        for (i, (property, value)) in props.properties.into_iter().enumerate() {
            let spaces: String = util::char_count(' ', max_len - property.len() + 2);
            result.push("    ".normal());
            result.push(property.color(color));
            result.push(":".color(color));
            result.push(spaces.normal());
            result.push(value.normal());
            if i + 1 < len {
                result.push("\n".normal());
            }
        }
        result
    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_len: usize = self
            .properties
            .iter()
            .map(|(p, _)| p.len())
            .max()
            .unwrap_or(0);
        let len: usize = self.properties.len();
        let color: Color = Info.color();
        for (i, (property, value)) in self.properties.iter().enumerate() {
            let spaces: String = util::char_count(' ', max_len - property.len() + 2);
            write!(
                f,
                "    {}{}{}{}",
                property.color(color),
                ":".color(color),
                spaces,
                value
            )?;
            if i + 1 < len {
                writeln!(f)?;
            }
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
