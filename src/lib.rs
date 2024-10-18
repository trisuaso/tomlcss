//! # tomlcss
//!
//! Convert TOML into CSS
use std::collections::BTreeMap;

pub type CSS = BTreeMap<String, CSSMap>;
pub type CSSMap = BTreeMap<String, String>;

pub fn convert(input: &str) -> String {
    let mut out = String::new();
    let css: CSS = toml::from_str(&input).unwrap_or(BTreeMap::new());

    for map in css {
        out.push_str(&format!("{} {{\n", map.0));

        for property in map.1 {
            out.push_str(&format!("    {}: {};\n", property.0, property.1));
        }

        out.push_str("}\n");
    }

    // return
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_background() {
        let result = convert("[body]\nbackground = \"#000000\"");
        assert_eq!(result, "body {\n    background: #000000;\n}\n")
    }

    #[test]
    fn body_background_color() {
        let result = convert("[body]\nbackground = \"#000000\"\ncolor = \"#ff0000\"");
        assert_eq!(
            result,
            "body {\n    background: #000000;\n    color: #ff0000;\n}\n"
        )
    }
}
