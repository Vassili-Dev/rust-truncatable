//! # truncatable
//!
//! Truncatable is a simple crate for Strings that should append
//! a follower when truncated (eg, ellipsis)
//!
//! ```
//! use truncatable::Trucatable;
//! let to_truncate = Truncatable::from("Hello World!").truncator("~~".into());
//! assert_eq!(to_trucate.truncate(5), String::from("Hello~~"));
//!```

/// Main struct, serves as a String wrapper
///
/// At its core, this struct is a wrapper for a string
/// but contains other crucial values such as the direction
/// of the text (right-to-left vs left-to-right). As well
/// as the token that should be appended to the truncated
/// text.
#[derive(Debug, Clone)]
pub struct Truncatable {
    value: String,
    truncator: String,
    rtl: bool,
}

impl From<&str> for Truncatable {
    fn from(value: &str) -> Self {
        Truncatable::new(String::from(value))
    }
}

impl From<String> for Truncatable {
    fn from(value: String) -> Self {
        Truncatable::new(value)
    }
}

impl Truncatable {
    /// Create a new truncator from a String
    pub fn new(value: String) -> Self {
        Self {
            value,
            rtl: false,
            truncator: String::from("..."),
        }
    }
    /// Change the truncator token (default = "...")
    pub fn truncator(mut self, truncator: String) -> Self {
        self.truncator = truncator;
        self
    }

    /// Set the truncation direction to be Left-To-Right (default)
    pub fn ltr(mut self) -> Self {
        self.rtl = false;
        self
    }

    /// Set the truncation direction to be Right-To-Left
    pub fn rtl(mut self) -> Self {
        self.rtl = true;
        self
    }

    /// Return a truncated string, syntactic sugar for ```format!("{value:.length$}", value=Truncator::from("test"), length=2)```
    pub fn truncate(&self, length: usize) -> String {
        return format!("{1:.*}", length, self);
    }
}
impl std::fmt::Display for Truncatable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            if precision < self.value.len() {
                if !self.rtl {
                    let res = f.pad(self.value.as_str());
                    write!(f, "{truncator}", truncator = self.truncator)
                        .expect("Failed to add truncator");
                    return res;
                } else {
                    write!(f, "{truncator}", truncator = self.truncator)
                        .expect("Failed to add truncator");
                    let res = f.pad(self.value.as_str());
                    return res;
                }
            }
        }
        return f.pad(self.value.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::Truncatable;

    #[test]
    fn build_from_str() {
        let trunc = Truncatable::from("Test_Truncatable");
        assert_eq!(format!("{}", trunc), String::from("Test_Truncatable"));
    }

    #[test]
    fn default_truncator() {
        let trunc = Truncatable::from("Test_Truncatable");
        assert_eq!(trunc.truncate(4), String::from("Test..."));
    }

    #[test]
    fn custom_truncator() {
        let trunc = Truncatable::from("Test_Truncatable").truncator("-".into());
        assert_eq!(trunc.truncate(4), String::from("Test-"));
    }

    #[test]
    fn right_to_left() {
        let trunc = Truncatable::from("Test_Truncatable").rtl();
        assert_eq!(trunc.truncate(4), "...Test");
        let trunc = trunc.ltr();
        assert_eq!(trunc.truncate(4), "Test...");
    }

    #[test]
    fn too_long_to_truncate() {
        let trunc = Truncatable::from("Test_Truncatable");
        assert_eq!(trunc.truncate(20), trunc.value);
        assert_eq!(format!("{:.20}", trunc), "Test_Truncatable");
    }

    #[test]
    fn works_with_other_formatting_flags() {
        let trunc = Truncatable::from("Test_Truncatable");
        let control = String::from("Test_Truncatable");

        let f_trunc = format!("{:]<50}", trunc);
        let f_control = format!("{:]<50}", control);
        assert_eq!(f_trunc, f_control);
    }
}
