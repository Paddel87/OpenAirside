//! Limit check of a JSON document before it is parsed (S-4 "Grenzen gegen
//! schädliche Pakete"): nesting depth, length of strings and length of lists.
//!
//! `serde_json` checks none of these limits (its recursion limit is a fixed
//! 128), so this linear scan runs first and reports the first violation with
//! line, column and JSON path. It does not check the syntax: on malformed
//! input it stops silently and leaves the error message to `serde_json`.

/// Limits of one JSON document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Most nested objects and arrays.
    pub depth: usize,
    /// Most characters of a string (keys included).
    pub string_chars: usize,
    /// Most elements of an array.
    pub list_len: usize,
}

/// The limit that was exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limit {
    /// Nesting deeper than [`Limits::depth`].
    Depth,
    /// A string longer than [`Limits::string_chars`].
    StringChars,
    /// An array longer than [`Limits::list_len`].
    ListLen,
}

/// First limit violation of a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// The exceeded limit.
    pub limit: Limit,
    /// One-based line of the offending value.
    pub line: u32,
    /// One-based column (in bytes) of the offending value.
    pub column: u32,
    /// JSON path in the notation of `serde_path_to_error` (`objects[3].id`,
    /// `.` for the root).
    pub path: String,
}

enum Frame {
    Object {
        key: Option<String>,
        expecting_key: bool,
    },
    Array {
        count: usize,
    },
}

struct Scanner<'a> {
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    line_start: usize,
    stack: Vec<Frame>,
    limits: Limits,
}

/// Checks `bytes` against `limits`.
///
/// # Errors
///
/// Returns the first violation in document order.
pub fn scan(bytes: &[u8], limits: Limits) -> Result<(), Violation> {
    Scanner {
        bytes,
        pos: 0,
        line: 1,
        line_start: 0,
        stack: Vec::new(),
        limits,
    }
    .run()
}

impl Scanner<'_> {
    fn run(&mut self) -> Result<(), Violation> {
        while let Some(&byte) = self.bytes.get(self.pos) {
            match byte {
                b'\n' => {
                    self.pos += 1;
                    self.line = self.line.saturating_add(1);
                    self.line_start = self.pos;
                }
                b'{' | b'[' => {
                    self.value_starts()?;
                    if self.stack.len() >= self.limits.depth {
                        return Err(self.violation(Limit::Depth, self.pos));
                    }
                    self.stack.push(if byte == b'{' {
                        Frame::Object {
                            key: None,
                            expecting_key: true,
                        }
                    } else {
                        Frame::Array { count: 0 }
                    });
                    self.pos += 1;
                }
                b'}' | b']' => {
                    if self.stack.pop().is_none() {
                        return Ok(());
                    }
                    self.pos += 1;
                }
                b',' => {
                    if let Some(Frame::Object { expecting_key, .. }) = self.stack.last_mut() {
                        *expecting_key = true;
                    }
                    self.pos += 1;
                }
                b'"' => {
                    let start = self.pos;
                    let is_key = matches!(
                        self.stack.last(),
                        Some(Frame::Object {
                            expecting_key: true,
                            ..
                        })
                    );
                    if !is_key {
                        self.value_starts()?;
                    }
                    let Some((chars, end)) = self.string_at(start) else {
                        return Ok(());
                    };
                    if chars > self.limits.string_chars {
                        return Err(self.violation(Limit::StringChars, start));
                    }
                    if is_key {
                        let key = String::from_utf8_lossy(&self.bytes[start + 1..end - 1]);
                        if let Some(Frame::Object {
                            key: k,
                            expecting_key,
                        }) = self.stack.last_mut()
                        {
                            *k = Some(key.into_owned());
                            *expecting_key = false;
                        }
                    }
                    self.pos = end;
                }
                b' ' | b'\t' | b'\r' | b':' => self.pos += 1,
                _ => {
                    self.value_starts()?;
                    // Literal (number, true, false, null): skip to its end.
                    while let Some(&b) = self.bytes.get(self.pos) {
                        if matches!(
                            b,
                            b',' | b'}' | b']' | b' ' | b'\t' | b'\r' | b'\n' | b'"' | b'{' | b'['
                        ) {
                            break;
                        }
                        self.pos += 1;
                    }
                }
            }
        }
        Ok(())
    }

    /// Counts a value that starts at `self.pos` in the enclosing array.
    fn value_starts(&mut self) -> Result<(), Violation> {
        let limit = self.limits.list_len;
        if let Some(Frame::Array { count }) = self.stack.last_mut() {
            *count += 1;
            if *count > limit {
                return Err(self.violation(Limit::ListLen, self.pos));
            }
        }
        Ok(())
    }

    /// Characters of the string starting with the quote at `start` and the
    /// position after its closing quote; `None` if it is not terminated.
    fn string_at(&self, start: usize) -> Option<(usize, usize)> {
        let mut i = start + 1;
        let mut chars = 0usize;
        loop {
            let byte = *self.bytes.get(i)?;
            match byte {
                b'"' => return Some((chars, i + 1)),
                b'\\' => {
                    // \uXXXX is one character, every other escape too.
                    i += if self.bytes.get(i + 1) == Some(&b'u') {
                        6
                    } else {
                        2
                    };
                    chars += 1;
                }
                _ => {
                    // Count code points: skip UTF-8 continuation bytes.
                    if byte & 0xC0 != 0x80 {
                        chars += 1;
                    }
                    i += 1;
                }
            }
        }
    }

    fn violation(&self, limit: Limit, at: usize) -> Violation {
        Violation {
            limit,
            line: self.line,
            column: u32::try_from(at - self.line_start + 1).unwrap_or(u32::MAX),
            path: self.path(),
        }
    }

    /// Path of the value at the current position.
    fn path(&self) -> String {
        let mut out = String::new();
        for frame in &self.stack {
            match frame {
                Frame::Object { key: Some(key), .. } => {
                    if !out.is_empty() {
                        out.push('.');
                    }
                    out.push_str(key);
                }
                Frame::Object { key: None, .. } => {}
                Frame::Array { count } => {
                    out.push('[');
                    out.push_str(&count.saturating_sub(1).to_string());
                    out.push(']');
                }
            }
        }
        if out.is_empty() {
            out.push('.');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMITS: Limits = Limits {
        depth: 4,
        string_chars: 8,
        list_len: 3,
    };

    fn check(json: &str) -> Result<(), Violation> {
        scan(json.as_bytes(), LIMITS)
    }

    #[test]
    fn accepts_documents_within_the_limits() {
        assert_eq!(
            check(r#"{"a": [1, 2, 3], "b": {"c": [{"d": "12345678"}]}}"#),
            Ok(())
        );
        assert_eq!(check("[]"), Ok(()));
        assert_eq!(check(""), Ok(()));
    }

    #[test]
    fn reports_depth_with_path() {
        let err = check(r#"{"a": {"b": [{"c": [1]}]}}"#).unwrap_err();
        assert_eq!(err.limit, Limit::Depth);
        assert_eq!(err.path, "a.b[0].c");
        assert_eq!((err.line, err.column), (1, 20));
    }

    #[test]
    fn reports_long_strings_and_keys() {
        let err = check("{\n  \"id\": \"123456789\"\n}").unwrap_err();
        assert_eq!(err.limit, Limit::StringChars);
        assert_eq!(err.path, "id");
        assert_eq!((err.line, err.column), (2, 9));
        let err = check(r#"{"123456789": 1}"#).unwrap_err();
        assert_eq!(err.limit, Limit::StringChars);
    }

    #[test]
    fn counts_characters_not_bytes_and_escapes_as_one() {
        // Eight umlauts are 16 bytes but 8 characters.
        assert_eq!(check(r#"["ääääääää"]"#), Ok(()));
        assert_eq!(check(r#"["ä\n\"\\abcd"]"#), Ok(()));
        assert!(check(r#"["ä\n\"\\abcde"]"#).is_err());
    }

    #[test]
    fn reports_long_lists_at_the_first_surplus_element() {
        let err = check(r#"{"objects": [1, 2, 3, {"x": 1}]}"#).unwrap_err();
        assert_eq!(err.limit, Limit::ListLen);
        assert_eq!(err.path, "objects[3]");
        assert_eq!(err.column, 23);
    }

    #[test]
    fn root_path_is_a_dot() {
        let err = check("[1, 2, 3, 4]").unwrap_err();
        assert_eq!(err.path, "[3]");
        let err = scan(
            br#""123456789""#,
            Limits {
                depth: 1,
                string_chars: 3,
                list_len: 1,
            },
        )
        .unwrap_err();
        assert_eq!(err.path, ".");
    }

    #[test]
    fn stops_silently_on_malformed_input() {
        assert_eq!(check(r#"{"a": "unterminated"#), Ok(()));
        assert_eq!(check("]]]"), Ok(()));
        assert_eq!(check(r#"["\"#), Ok(()));
    }
}
