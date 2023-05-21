pub mod error;
pub mod instruction;
pub mod line;
pub mod token;

use self::{error::ParserError, instruction::Instruction, line::{Line, MemType}, token::Token};

use std::{iter::Iterator, str::FromStr};

/// Parses lines of asmr code into their corresponding representation as [`Line`]s.
/// 
/// # Examples
/// 
/// ```
/// # use assembl_really::parser::parse_lines;
/// let v = vec!["push 5", "call asmr::io::print"];
/// let lines = parse_lines(v.iter());
/// ```
pub fn parse_lines<L, S>(lines: L) -> Result<Vec<Line>, ParserError>
where
    L: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut parsed_lines = Vec::new();

    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        parsed_lines.push(parse_line(line.as_ref(), line_number)?);
    }

    Ok(parsed_lines)
}

/// Parses a line of asmr code into its corresponding representation as a [`Line`].
fn parse_line(line: &str, line_number: i32) -> Result<Line, ParserError>
{
    let mut line = line.trim();

    // Check if the line is blank
    let first_o = line.split_whitespace().next();
    if first_o == None {
        return Ok(Line::Blank);
    }

    // Remove inline comments
    let inline_comments = string_literal_aware_split(line, ';');
    if inline_comments.len() > 1 {
        line = inline_comments.first().unwrap().trim();
    }

    // Get the first token in the line
    let first = first_o.unwrap();

    // Ignore full line comments
    if first.chars().next() == Some(';') {
        return Ok(Line::Blank);
    }

    // Parse instructions
    if let Ok(instruction) = Instruction::from_str(first) {
        let remainder = line[first.len()..].trim();
        let mut params = Vec::new();
        
        // Convert the instruction arguments into their corresponding tokens
        if remainder.chars().last() != None {
            let tokens: Vec<_> = string_literal_aware_split(remainder, ',').iter()
                            .map(|s| Token::from_str(s.trim())).collect();
            
            // Get all the tokens or throw a ParserError
            for tok in tokens {
                match tok {
                    Ok(t) => params.push(t),
                    Err(cause) => return Err(ParserError { line_number, cause }),
                }
            }
        }

        return Ok(Line::Instruction { instruction, params });
    }

    // Parse labels
    else if line.chars().last() == Some(':') {
        let label = line[..line.len() - 1].trim();
        if !is_valid_identifier(label) {
            return Err(ParserError { line_number, cause: format!("Invalid label `{label}`. Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].") });
        }
        
        return Ok(Line::Label(label.to_string()));
    }

    // Parse variables
    else if line.contains("db") || line.contains("resb") {
        let mut identifier;
        let mut args;
        let mem_type;

        // Get the memory definition type
        if line.contains("db"){
            (identifier, args) = line.split_once("db").unwrap();
            mem_type = MemType::Db;
        }
        else {
            (identifier, args) = line.split_once("resb").unwrap();
            mem_type = MemType::Resb;
        }
            
        identifier = identifier.trim();
        args = args.trim();

        // Ensure proper syntax
        if identifier == "" || args == "" {
            return Err(ParserError { line_number, cause: "Invalid memory definition syntax.".to_string() });
        }

        // Ensure valid identifier
        if !is_valid_identifier(identifier) {
            return Err(ParserError { line_number, cause: format!("Invalid identifier `{identifier}`. Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].") });
        }
        
        // Convert the memory arguments into their corresponding tokens
        let mut params = Vec::new();
        if args.chars().last() != None {
            let tokens: Vec<_> = string_literal_aware_split(args, ',').iter()
                            .map(|s| Token::from_str(s.trim())).collect();
            
            // Get all the tokens or throw a ParserError
            for tok in tokens {
                match tok {
                    Ok(t) => params.push(t),
                    Err(cause) => return Err(ParserError { line_number, cause }),
                }
            }
        }

        return Ok(Line::Variable { identifier: identifier.to_string(), mem_type, params });
    }

    Err(ParserError { line_number, cause: "Could not parse the line. There is likely an uncaught syntax error.".to_string() })
}

/// Checks whether a given string is a valid asmr identifier.
fn is_valid_identifier(s: &str) -> bool {
    if let Some(first) = s.chars().next() {
        if !first.is_alphabetic() && first != '_' && first != '.' {
            return false;
        }
    }

    for c in s.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' && c != '.' && c != ':' {
            return false;
        }
    }

    true
}

/// Splits a string by a pattern provided the pattern is not inside an asmr string literal.
/// Quotation marks are kept, the matched pattern is not.
/// 
/// # Panics
/// This will panic if `pat == '"'` since splitting is performed based on quotation marks in `s`.
fn string_literal_aware_split(s: &str, pat: char) -> Vec<String>
{
    if pat == '"' {
        panic!("Split pattern cannot be a quotation mark.")
    }

    let mut split = Vec::new();
    let mut is_quoted = false;
    let mut segment = String::new();

    for c in s.chars() {
        if c == pat && !is_quoted {
            split.push(segment);
            segment = String::new();
            continue;
        }
        
        if c == '"' {
            is_quoted = !is_quoted;
        }
        segment.push(c);
    }
    split.push(segment);

    split
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::register::RegisterName;
    type ParserResult = Result<(), ParserError>;

    #[test]
    fn parse_line_parses_blank_line() -> ParserResult {
        let s1 = parse_line("", 0)?;
        let s2 = parse_line(" ", 0)?;

        assert_eq!(s1, Line::Blank);
        assert_eq!(s2, Line::Blank);

        Ok(())
    }

    #[test]
    fn parse_line_removes_inline_comments() -> ParserResult {
        let s1 = parse_line("nop ; inline comment", 0)?;
        let s2 = parse_line("push \"semicolon; in; string\" ; inline comment", 0)?;

        assert_eq!(s1, Line::Instruction { instruction: Instruction::Nop, params: vec![] });
        assert_eq!(s2, Line::Instruction { instruction: Instruction::Push, params: vec![
            Token::String("semicolon; in; string".to_string()),
        ] });

        Ok(())
    }

    #[test]
    fn parse_line_treats_line_comments_as_blank() -> ParserResult {
        let s1 = parse_line("; this is a line comment", 0)?;
        let s2 = parse_line("  ; line comment with whitespace", 0)?;

        assert_eq!(s1, Line::Blank);
        assert_eq!(s2, Line::Blank);

        Ok(())
    }

    #[test]
    fn parse_line_parses_instructions() -> ParserResult {
        let s1 = parse_line("mov eax, ecx", 0)?;
        let s2 = parse_line("add edx, 5", 0)?;
        let s3 = parse_line("call asmr::io::println", 0)?;

        assert_eq!(s1, Line::Instruction { instruction: Instruction::Mov, params: vec![
            Token::Register(RegisterName::Eax),
            Token::Register(RegisterName::Ecx),
        ] });

        assert_eq!(s2, Line::Instruction { instruction: Instruction::Add, params: vec![
            Token::Register(RegisterName::Edx),
            Token::Numeric(5),
        ] });

        assert_eq!(s3, Line::Instruction { instruction: Instruction::Call, params: vec![
            Token::Identifier("asmr::io::println".to_string()),
        ] });

        Ok(())
    }

    #[test]
    fn parse_line_parses_instructions_err_on_invalid_param() {
        let s1 = parse_line("mov edx, 0xq", 0);
        let s2 = parse_line("call #2", 0);
        
        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Could not parse `0xq` as a hexadecimal value.".to_string(),
        }));

        assert!(s2.err() == Some(ParserError {
            line_number: 0,
            cause: "Could not parse token `#2`. Was this meant to be an identifier? Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string(),
        }));
    }

    #[test]
    fn parse_line_parses_labels() -> ParserResult {
        let s1 = parse_line(".loop:", 0)?;
        let s2 = parse_line(" main: ", 0)?;

        assert_eq!(s1, Line::Label(".loop".to_string()));
        assert_eq!(s2, Line::Label("main".to_string()));

        Ok(())
    }

    #[test]
    fn parse_line_parses_labels_err_on_invalid_label() {
        let s1 = parse_line("#2:", 0);
        let s2 = parse_line("another label:", 0);

        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Invalid label `#2`. Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string(),
        }));

        assert!(s2.err() == Some(ParserError {
            line_number: 0,
            cause: "Invalid label `another label`. Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string(),
        }));
    }

    #[test]
    fn parse_line_parses_variables() -> ParserResult {
        let s1 = parse_line("msg db \"my message\"", 0)?;
        let s2 = parse_line("buffer resb 50", 0)?;
        let s3 = parse_line(" cmplx_ex  db \"this db'ed string, ends with a newline (0xa)\", 0xa", 0)?;

        assert_eq!(s1, Line::Variable { 
            identifier: "msg".to_string(),
            mem_type: MemType::Db,
            params: vec![
                Token::String("my message".to_string()),
            ],
        });

        assert_eq!(s2, Line::Variable { 
            identifier: "buffer".to_string(),
            mem_type: MemType::Resb,
            params: vec![
                Token::Numeric(50),
            ],
        });

        assert_eq!(s3, Line::Variable { 
            identifier: "cmplx_ex".to_string(),
            mem_type: MemType::Db,
            params: vec![
                Token::String("this db'ed string, ends with a newline (0xa)".to_string()),
                Token::Numeric(0xa),
            ],
        });

        Ok(())
    }

    #[test]
    fn parse_line_parses_variables_err_on_invalid_syntax() {
        let s1 = parse_line("msg db  ", 0);
        let s2 = parse_line("db 35", 0);

        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Invalid memory definition syntax.".to_string(),
        }));

        assert!(s2.err() == Some(ParserError {
            line_number: 0,
            cause: "Invalid memory definition syntax.".to_string(),
        }));
    }

    #[test]
    fn parse_line_parses_variables_err_on_invalid_identifier() {
        let s1 = parse_line("$5 db \"five dollars\"", 0);

        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Invalid identifier `$5`. Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string(),
        }));
    }

    #[test]
    fn parse_line_parses_variables_err_on_invalid_param() {
        let s1 = parse_line("foo db 0b12", 0);
        
        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Could not parse `0b12` as a binary value.".to_string(),
        }));
    }

    #[test]
    fn parse_line_err_on_unparsable_line() {
        let s1 = parse_line("#this should never_compile", 0);
        let s2 = parse_line("12 xor 3 ; invalid instruction order", 0);

        assert!(s1.err() == Some(ParserError {
            line_number: 0,
            cause: "Could not parse the line. There is likely an uncaught syntax error.".to_string(),
        }));

        assert!(s2.err() == Some(ParserError {
            line_number: 0,
            cause: "Could not parse the line. There is likely an uncaught syntax error.".to_string(),
        }));
    }

    #[test]
    fn is_valid_identifier_true_when_valid() {
        let s1 = "prompt_msg_2";
        let s2 = "asmr::io::println";
        let s3 = ".loop:";

        assert!(is_valid_identifier(s1));
        assert!(is_valid_identifier(s2));
        assert!(is_valid_identifier(s3));
    }
    
    #[test]
    fn is_valid_identifier_false_when_invalid() {
        let s1 = "prompt 7";
        let s2 = "phone#";
        let s3 = "2nd_id";
    
        assert!(!is_valid_identifier(s1));
        assert!(!is_valid_identifier(s2));
        assert!(!is_valid_identifier(s3));
    }

    #[test]
    fn string_literal_aware_split_parses_with_no_strings() {
        let s1 = string_literal_aware_split("regular,list,of,words", ',');
        let s2 = string_literal_aware_split("semicolons;work;too", ';');

        assert_eq!(s1, vec!["regular", "list", "of", "words"]);
        assert_eq!(s2, vec!["semicolons", "work", "too"]);
    }
    
    #[test]
    fn string_literal_aware_split_parses_with_strings() {
        let s1 = string_literal_aware_split("\"Hello, World!\"", ',');
        let s2 = string_literal_aware_split("mov eax, \"Welcome, friend.\"", ',');
        let s3 = string_literal_aware_split("push \"programming; computers\" ; this is a comment", ';');

        assert_eq!(s1, vec!["\"Hello, World!\""]);
        assert_eq!(s2, vec!["mov eax", " \"Welcome, friend.\""]);
        assert_eq!(s3, vec!["push \"programming; computers\" ", " this is a comment"]);
    }

    #[test]
    #[should_panic(expected = "Split pattern cannot be a quotation mark.")]
    fn string_literal_aware_split_panics_on_quote_pat() {
        string_literal_aware_split("some arbitrary string", '"');
    }
}
