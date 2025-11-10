use crate::report::utils::char_count;
use crate::Severity::Info;
use colored::{ColoredString, Colorize};

/// A properties entry.
///
/// # Display
///
/// ```text
///     first:  value
///     second: another value
///     third:  third value
/// ```
pub struct Properties {
    pub properties: Vec<(String, String)>,
}

impl Properties {
    //! Entry

    /// Constructs the report entry.
    pub fn entry(mut self) -> Vec<ColoredString> {
        let max_len: usize = self
            .properties
            .iter()
            .map(|(p, _)| p.len())
            .max()
            .unwrap_or(0);
        let mut entry: Vec<ColoredString> = Vec::with_capacity(self.properties.len() * 6);
        for (property, value) in self.properties.drain(..) {
            let len: usize = property.len();
            let spaces: usize = max_len - len + 2;

            entry.push("    ".normal());
            entry.push(property.color(Info.color()));
            entry.push(":".color(Info.color()));
            entry.push(char_count(' ', spaces).normal());
            entry.push(value.normal());
            entry.push("\n".normal());
        }
        entry
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Properties, Report};

    #[test]
    #[ignore]
    fn properties() {
        let properties: Properties = Properties {
            properties: vec![
                ("one".to_string(), "two".to_string()),
                ("three".to_string(), "four".to_string()),
                ("five".to_string(), "six".to_string()),
            ],
        };
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::new(code).with_entry(properties.entry());
        println!("{}", report)
    }
}
