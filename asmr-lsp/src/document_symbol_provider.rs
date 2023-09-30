use wasm_bindgen::prelude::*;
use serde_json as json;
use serde::Serialize;

use assembl_really as asmr;
use asmr::parser::line::Line;

/// Parses an asmr file into an array of document symbols for navigation.
/// Returns a JSON encoded `DocumentSymbol[]`.
#[wasm_bindgen(js_name = getDocumentSymbols)]
pub fn get_document_symbols(file_contents: &str) -> String {
    let file_lines = file_contents.split('\n').collect::<Vec<_>>();
    let lines = asmr::parse_lines(file_lines.iter()).unwrap_throw();
    let mut symbols: Vec<DocumentSymbol> = Vec::new();
    
    // Tracks the current label that symbols are under
    let mut current_label: Option<(String, usize)> = None;

    let mut push_symbol = |name, kind, start, i| symbols.push(DocumentSymbol {
        token_name: name,
        token_type: kind,
        range: Range {
            line_start: start,
            char_start: 0,
            line_end: i,
            char_end: file_lines.get(i).unwrap().len(),
        }
    });

    for i in 0..lines.len() {
        if let Some(Line::Label(label)) = lines.get(i) {
            // Push existing label if it exists
            if let Some(cur) = current_label {
                push_symbol(cur.0, SymbolType::Label, cur.1, i - 1);
            }

            // Set current label
            current_label = Some((label.clone(), i));
        }
        else if let Some(Line::Variable { identifier, .. }) = lines.get(i) {
            push_symbol(identifier.clone(), SymbolType::Variable, i, i);
        }
    }

    // Push remaining label if one exists
    if let Some(cur) = current_label {
        push_symbol(cur.0, SymbolType::Label, cur.1, lines.len() - 1);
    }

    json::to_string(&symbols).unwrap_throw()
}

/// Contains document symbol information for asmr symbols
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DocumentSymbol {
    #[wasm_bindgen(skip)]
    pub token_name: String,
    pub token_type: SymbolType,
    pub range: Range,
}

#[wasm_bindgen]
impl DocumentSymbol {
    #[wasm_bindgen(getter)]
    pub fn token_name(&self) -> String {
        self.token_name.clone()
    }
}

/// The types of asmr document symbols
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SymbolType {
    Label = 11,
    Variable = 12,
}

/// Contains position information about a document symbol
#[wasm_bindgen]
#[derive(Serialize)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Range {
    pub line_start: usize,
    pub char_start: usize,
    pub line_end: usize,
    pub char_end: usize,
}
