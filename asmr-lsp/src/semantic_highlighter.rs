use wasm_bindgen::prelude::*;
use serde_json as json;
use serde::Serialize;

use assembl_really as asmr;
use asmr::parser::{line::Line, token::Token};

/// Parses an asmr file into an array of semantic tokens for semantic highlighting.
/// Returns a JSON encoded `SemanticToken[]`.
#[wasm_bindgen(js_name = parseFileTokens)]
pub fn parse_file_tokens(file_contents: &str) -> String {
    let file_lines = file_contents.split('\n').collect::<Vec<_>>();
    let lines = asmr::parse_lines(file_lines.iter()).unwrap_throw();
    let mut tokens: Vec<SemanticToken> = Vec::new();
    
    let mut identifiers: Vec<&String> = Vec::new();
    for i in 0..lines.len() {
        if let Some(Line::Label(label)) = lines.get(i) {
            identifiers.push(label);
        }
    }
    
    for i in 0..lines.len() {
        let line = file_lines.get(i).unwrap_throw();

        match lines.get(i) {
            Some(Line::Instruction { params, .. }) => {
                tokens.append(&mut parse_params(params, line, i, &identifiers));
            },
            Some(Line::Label(label)) => {
                tokens.push(SemanticToken {
                    token_name: label.clone(),
                    delta_line: i as u32,
                    delta_start: line.match_indices(label).collect::<Vec<_>>().first().unwrap().0 as u32,
                    length: label.len() as u32,
                    token_type: SemanticTokenType::Variable as u32,
                });
                identifiers.push(label);
            },
            Some(Line::Variable { identifier, params, .. }) => {
                tokens.push(SemanticToken {
                    token_name: identifier.clone(),
                    delta_line: i as u32,
                    delta_start: line.match_indices(identifier).collect::<Vec<_>>().first().unwrap().0 as u32,
                    length: identifier.len() as u32,
                    token_type: SemanticTokenType::Variable as u32,
                });
                identifiers.push(identifier);

                tokens.append(&mut parse_params(params, line, i, &identifiers));
            },
            _ => {}, // No semantic information
        };
    }

    json::to_string(&tokens).unwrap_throw()
}

fn parse_params(params: &Vec<Token>, line: &str, line_idx: usize, identifiers: &Vec<&String>) -> Vec<SemanticToken> {
    let mut tokens: Vec<SemanticToken> = Vec::new();

    for token in params {
        match token {
            Token::Identifier(identifier) => {
                if ["asmr::io::print", "asmr::io::readln"].contains(&identifier.as_str()) {
                    tokens.push(SemanticToken {
                        token_name: identifier.clone(),
                        delta_line: line_idx as u32,
                        delta_start: line.match_indices(identifier).collect::<Vec<_>>().first().unwrap().0 as u32,
                        length: identifier.len() as u32,
                        token_type: SemanticTokenType::Function as u32,
                    });
                    continue;
                }

                if identifiers.contains(&identifier) {
                    tokens.push(SemanticToken {
                        token_name: identifier.clone(),
                        delta_line: line_idx as u32,
                        delta_start: line.match_indices(identifier).collect::<Vec<_>>().first().unwrap().0 as u32,
                        length: identifier.len() as u32,
                        token_type: SemanticTokenType::Variable as u32,
                    });
                }
            },
            _ => {}, // No semantic information
        };
    }

    tokens
}

/// Contains semantic highlighting information for an asmr token
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SemanticToken {
    #[wasm_bindgen(skip)]
    pub token_name: String,
    pub delta_line: u32,
    pub delta_start: u32,
    pub length: u32,
    pub token_type: u32,
}

#[wasm_bindgen]
impl SemanticToken {
    #[wasm_bindgen(getter)]
    pub fn token_name(&self) -> String {
        self.token_name.clone()
    }
}

pub enum SemanticTokenType {
    Variable,
    Function,
}
