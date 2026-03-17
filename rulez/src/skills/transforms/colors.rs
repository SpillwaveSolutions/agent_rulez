//! Named color to hex conversion for OpenCode compatibility.
//!
//! OpenCode accepts hex color codes but does not resolve CSS color names.
//! Gemini strips the `color:` field entirely (handled in frontmatter transform).

/// Convert a CSS color name to its hex equivalent.
/// Returns `None` if the name is not recognized (might already be hex).
pub fn color_name_to_hex(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "cyan" => Some("#00FFFF"),
        "red" => Some("#FF0000"),
        "green" => Some("#00FF00"),
        "blue" => Some("#0000FF"),
        "yellow" => Some("#FFFF00"),
        "magenta" => Some("#FF00FF"),
        "orange" => Some("#FFA500"),
        "purple" => Some("#800080"),
        "pink" => Some("#FFC0CB"),
        "white" => Some("#FFFFFF"),
        "black" => Some("#000000"),
        "gray" | "grey" => Some("#808080"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_colors() {
        assert_eq!(color_name_to_hex("cyan"), Some("#00FFFF"));
        assert_eq!(color_name_to_hex("Red"), Some("#FF0000"));
        assert_eq!(color_name_to_hex("GREY"), Some("#808080"));
        assert_eq!(color_name_to_hex("gray"), Some("#808080"));
    }

    #[test]
    fn test_unknown_returns_none() {
        assert_eq!(color_name_to_hex("#FF0000"), None);
        assert_eq!(color_name_to_hex("chartreuse"), None);
    }
}
