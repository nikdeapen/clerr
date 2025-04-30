use crate::{Report, Severity};
use colored::{ColoredString, Colorize};

impl Report {
    //! Properties

    /// Generates the `properties` entry.
    ///
    /// # Display
    /// first:   value
    /// second:  another value
    /// third:   third value
    pub fn properties(mut properties: Vec<(String, String)>) -> Vec<ColoredString> {
        let max_property_len: usize = properties.iter().map(|(p, _)| p.len()).max().unwrap_or(0);
        let mut entry: Vec<ColoredString> = Vec::with_capacity(properties.len() * 5);
        for (property, value) in properties.drain(..) {
            let property_len: usize = property.len();
            let spaces: usize = max_property_len - property_len + 2;

            entry.push(property.color(Severity::Info.color()));
            entry.push(":".color(Severity::Info.color()));
            entry.push(Self::char_count(' ', spaces).normal());
            entry.push(value.normal());
            entry.push("\n".normal());
        }
        entry
    }

    /// Adds the `properties` entry.
    pub fn add_properties(&mut self, properties: Vec<(String, String)>) {
        self.add_entry(Self::properties(properties));
    }

    /// Adds the `properties` entry.
    pub fn with_properties(mut self, properties: Vec<(String, String)>) -> Self {
        self.add_properties(properties);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Report;
    use colored::ColoredString;

    #[test]
    #[ignore]
    fn properties() {
        let info: Vec<ColoredString> = Report::properties(vec![
            ("one".to_string(), "two".to_string()),
            ("three".to_string(), "four".to_string()),
            ("five".to_string(), "six".to_string()),
        ]);

        for s in info {
            print!("{}", s);
        }
    }
}
