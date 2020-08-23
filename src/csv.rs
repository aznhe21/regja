use std::iter::Peekable;
use std::str::Chars;

pub struct CsvFieldParser<'a> {
    chars: Peekable<Chars<'a>>,
    has_next: bool,
}

impl<'a> CsvFieldParser<'a> {
    pub fn new(s: &'a str) -> CsvFieldParser<'a> {
        CsvFieldParser {
            chars: s.chars().peekable(),
            has_next: true,
        }
    }

    fn parse_quote(&mut self) -> Result<String, CsvError> {
        assert_eq!(self.chars.next(), Some('"'));

        let mut buffer = String::new();
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') => return Err(CsvError),
                Some('"') => match self.chars.peek() {
                    Some('"') => {
                        self.chars.next();
                        buffer.push('"');
                    }
                    Some(',') => {
                        self.chars.next();
                        return Ok(buffer);
                    }
                    None => {
                        self.has_next = false;
                        return Ok(buffer);
                    }
                    Some(_) => return Err(CsvError),
                },
                Some('\\') => match self.chars.next() {
                    Some('n') => buffer.push('\n'),
                    Some('r') => buffer.push('\r'),
                    Some('t') => buffer.push('\t'),
                    _ => return Err(CsvError),
                },
                Some(c) => buffer.push(c),
            }
        }
    }

    fn parse_raw(&mut self) -> Result<String, CsvError> {
        let mut buffer = String::new();
        loop {
            match self.chars.next() {
                None => {
                    self.has_next = false;
                    return Ok(buffer);
                }
                Some(',') => return Ok(buffer),
                Some('\r') | Some('\n') => return Err(CsvError),
                Some(c) => {
                    buffer.push(c);
                }
            }
        }
    }
}

impl<'a> Iterator for CsvFieldParser<'a> {
    type Item = Result<String, CsvError>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.has_next, self.chars.peek()) {
            (false, None) => None,
            (false, Some(_)) => Some(Err(CsvError)),

            (true, None) => {
                self.has_next = false;
                Some(Ok(String::new()))
            }
            (true, Some('"')) => Some(self.parse_quote()),
            (true, Some(_)) => Some(self.parse_raw()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw() {
        let mut parser = CsvFieldParser::new("hoge,fuga,piyo");
        assert_eq!(parser.next(), Some(Ok("hoge".to_string())));
        assert_eq!(parser.next(), Some(Ok("fuga".to_string())));
        assert_eq!(parser.next(), Some(Ok("piyo".to_string())));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parse_quoted() {
        let mut parser = CsvFieldParser::new(r#""hoge","fu,ga","pi\nyo""#);
        assert_eq!(parser.next(), Some(Ok("hoge".to_string())));
        assert_eq!(parser.next(), Some(Ok("fu,ga".to_string())));
        assert_eq!(parser.next(), Some(Ok("pi\nyo".to_string())));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_trailing_comma() {
        let mut parser = CsvFieldParser::new("hoge,");
        assert_eq!(parser.next(), Some(Ok("hoge".to_string())));
        assert_eq!(parser.next(), Some(Ok("".to_string())));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_invalid_quote() {
        let mut parser = CsvFieldParser::new(r#""ho"ge"#);
        assert_eq!(parser.next(), Some(Err(CsvError)));
    }
}
