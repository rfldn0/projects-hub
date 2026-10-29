//! Escaping for text and attributes pulled from projects.json.

pub fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// An external link safe to put in `href`: only http(s) URLs pass.
pub fn external_url(s: &str) -> Option<String> {
    let s = s.trim();
    let lower = s.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://")).then(|| esc(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_markup() {
        assert_eq!(esc(r#"<a href="x">'&'</a>"#), "&lt;a href=&quot;x&quot;&gt;&#39;&amp;&#39;&lt;/a&gt;");
    }

    #[test]
    fn only_http_links() {
        assert_eq!(external_url("https://github.com/a?b=1&c").as_deref(), Some("https://github.com/a?b=1&amp;c"));
        assert_eq!(external_url("javascript:alert(1)"), None);
        assert_eq!(external_url(""), None);
        assert_eq!(external_url("#"), None);
    }
}
