use wasm_bindgen::prelude::*;
use serde_json as json;
use serde::Serialize;

use strum::IntoEnumIterator;

use assembl_really as asmr;
use asmr::core::register::RegisterName;
use asmr::parser::{line::Line, instruction::Instruction};

/// Parses an asmr file into an array of completion items for intellisense.
/// Returns a JSON encoded `CompletionItem[]`.
#[wasm_bindgen(js_name = getCompletionItems)]
pub fn get_completion_items(file_contents: &str) -> String {
    let lines = asmr::parse_lines(file_contents.split('\n')).unwrap_throw();
    let mut completion_items: Vec<CompletionItem> = Vec::new();

    for i in 0..lines.len() {
        match lines.get(i) {
            Some(Line::Label(label)) => {
                completion_items.push(CompletionItem {
                    token_name: label.clone(),
                    token_type: CompletionType::Label,
                });
            },
            Some(Line::Variable { identifier, .. }) => {
                completion_items.push(CompletionItem {
                    token_name: identifier.clone(),
                    token_type: CompletionType::Variable,
                });
            },
            _ => {}, // No completion information
        };
    }

    // Register builtin functions
    completion_items.push(CompletionItem {
        token_name: "asmr::io::print".to_string(),
        token_type: CompletionType::Function,
    });
    completion_items.push(CompletionItem {
        token_name: "asmr::io::readln".to_string(),
        token_type: CompletionType::Function,
    });

    // Register registers
    RegisterName::iter().for_each(|r| {
        completion_items.push(CompletionItem {
            token_name: r.to_string(),
            token_type: CompletionType::Register,
        });
    });

    // Register instructions
    Instruction::iter().for_each(|r| {
        completion_items.push(CompletionItem {
            token_name: r.to_string(),
            token_type: CompletionType::Instruction,
        });
    });

    json::to_string(&completion_items).unwrap_throw()
}

/// Contains completion item information for an asmr token
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CompletionItem {
    #[wasm_bindgen(skip)]
    pub token_name: String,
    pub token_type: CompletionType,
}

#[wasm_bindgen]
impl CompletionItem {
    #[wasm_bindgen(getter)]
    pub fn token_name(&self) -> String {
        self.token_name.clone()
    }
}

/// The types of asmr completion items
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CompletionType {
    Variable,
    Function,
    Label,
    Register,
    Instruction,
}
