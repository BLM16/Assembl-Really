use std::str::FromStr;

use crate::core::register::RegisterName;

use super::is_valid_identifier;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Numeric(i32),
    Register(RegisterName),
    String(String),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Token, Self::Err> {
        // Parse registers
        if let Ok(register) = RegisterName::from_str(s) {
            return Ok(Token::Register(register));
        }

        // Parse string values
        else if s.contains('"') {
            if s.matches('"').count() == 2 && s.chars().next() == Some('"') && s.chars().last() == Some('"') {
                return Ok(Token::String(s[1..s.len() - 1].to_string()));
            }
            return Err(format!("Could not parse `{s}` as a string value."));
        }

        // Parse hexadecimal values
        else if s.starts_with("0x") {
            let int = i32::from_str_radix(s.trim_start_matches("0x"), 16);
            match int {
                Ok(i) => return Ok(Token::Numeric(i)),
                Err(_) => return Err(format!("Could not parse `{s}` as a hexadecimal value.")),
            }
        }

        // Parse binary values
        else if s.starts_with("0b") {
            let int = i32::from_str_radix(s.trim_start_matches("0b"), 2);
            match int {
                Ok(i) => return Ok(Token::Numeric(i)),
                Err(_) => return Err(format!("Could not parse `{s}` as a binary value.")),
            }
        }

        // Parse decimal values
        else if s.starts_with(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
            let int = i32::from_str(s);
            match int {
                Ok(i) => return Ok(Token::Numeric(i)),
                Err(_) => return Err(format!("Could not parse `{s}` as a decimal value.")),
            }
        }

        // Parse identifiers
        else if is_valid_identifier(s) {
            return Ok(Token::Identifier(s.to_owned()));
        }
        
        Err(format!("Could not parse token `{s}`. Was this meant to be an identifier? Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .]."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_from_str_parses_register_name() -> Result<(), String> {
        let t1 = Token::from_str("eax")?;
        let t2 = Token::from_str("bp")?;
        
        assert_eq!(t1, Token::Register(RegisterName::Eax));
        assert_eq!(t2, Token::Register(RegisterName::Bp));

        Ok(())
    }

    #[test]
    fn token_from_str_parses_string_value() -> Result<(), String> {
        let t1 = Token::from_str("\"Hello World\"")?;
        let t2 = Token::from_str("\"Welcome\"")?;
        let t3 = Token::from_str("\"123\"")?;

        assert_eq!(t1, Token::String("Hello World".to_string()));
        assert_eq!(t2, Token::String("Welcome".to_string()));
        assert_eq!(t3, Token::String("123".to_string()));

        Ok(())
    }

    #[test]
    fn token_from_str_err_on_invalid_string_value() {
        let t1 = Token::from_str("D\"og");
        let t2 = Token::from_str("\"Do\"g");

        assert!(t1.err() == Some("Could not parse `D\"og` as a string value.".to_string()));
        assert!(t2.err() == Some("Could not parse `\"Do\"g` as a string value.".to_string()));
    }

    #[test]
    fn token_from_str_parses_hexadecimal_value() -> Result<(), String> {
        let t1 = Token::from_str("0x0")?;
        let t2 = Token::from_str("0xa")?;
        let t3 = Token::from_str("0x1a35e")?;
        
        assert_eq!(t1, Token::Numeric(0x0));
        assert_eq!(t2, Token::Numeric(0xa));
        assert_eq!(t3, Token::Numeric(0x1a35e));

        Ok(())
    }

    #[test]
    fn token_from_str_err_on_invalid_hexadecimal_value() {
        let t1 = Token::from_str("0x");
        let t2 = Token::from_str("0xq4");

        assert!(t1.err() == Some("Could not parse `0x` as a hexadecimal value.".to_string()));
        assert!(t2.err() == Some("Could not parse `0xq4` as a hexadecimal value.".to_string()));
    }

    #[test]
    fn token_from_str_parses_binary_value() -> Result<(), String> {
        let t1 = Token::from_str("0b0")?;
        let t2 = Token::from_str("0b11010110")?;
        let t3 = Token::from_str("0b1010111010001001")?;

        assert_eq!(t1, Token::Numeric(0b0));
        assert_eq!(t2, Token::Numeric(0b11010110));
        assert_eq!(t3, Token::Numeric(0b1010111010001001));

        Ok(())
    }

    #[test]
    fn token_from_str_err_on_invalid_binary_value() {
        let t1 = Token::from_str("0b");
        let t2 = Token::from_str("0b21");

        assert!(t1.err() == Some("Could not parse `0b` as a binary value.".to_string()));
        assert!(t2.err() == Some("Could not parse `0b21` as a binary value.".to_string()));
    }

    #[test]
    fn token_from_str_parses_decimal_value() -> Result<(), String> {
        let t1 = Token::from_str("0")?;
        let t2 = Token::from_str("10")?;
        let t3 = Token::from_str("219384")?;

        assert_eq!(t1, Token::Numeric(0));
        assert_eq!(t2, Token::Numeric(10));
        assert_eq!(t3, Token::Numeric(219_384));

        Ok(())
    }

    #[test]
    fn token_from_str_err_on_invalid_decimal_value() {
        let t1 = Token::from_str("1_000");
        let t2 = Token::from_str("11.4");
        let t3 = Token::from_str("6r");

        assert!(t1.err() == Some("Could not parse `1_000` as a decimal value.".to_string()));
        assert!(t2.err() == Some("Could not parse `11.4` as a decimal value.".to_string()));
        assert!(t3.err() == Some("Could not parse `6r` as a decimal value.".to_string()));
    }

    #[test]
    fn token_from_str_parses_identifier_name() -> Result<(), String> {
        let t1 = Token::from_str("asmr::io::println")?;
        let t2 = Token::from_str("msg1")?;
        let t3 = Token::from_str(".loop")?;

        assert_eq!(t1, Token::Identifier("asmr::io::println".to_string()));
        assert_eq!(t2, Token::Identifier("msg1".to_string()));
        assert_eq!(t3, Token::Identifier(".loop".to_string()));

        Ok(())
    }

    #[test]
    fn token_from_str_err_on_invalid_identifier_name() {
        let t1 = Token::from_str("question mark");
        let t2 = Token::from_str("#seven");

        assert!(t1.err() == Some("Could not parse token `question mark`. Was this meant to be an identifier? Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string()));
        assert!(t2.err() == Some("Could not parse token `#seven`. Was this meant to be an identifier? Identifiers must be strictly [a-z, A-Z, 0-9, _, ., :] and must start with [a-z, A-Z. _, .].".to_string()));
    }
}
