import * as vscode from 'vscode';
import * as asmr from './asmr/asmr_lsp';

export class AsmrDocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(document: vscode.TextDocument, token: vscode.CancellationToken): vscode.ProviderResult<vscode.SymbolInformation[] | vscode.DocumentSymbol[]> {
        const documentSymbols: vscode.DocumentSymbol[] = [];
        let symbols: asmr.DocumentSymbol[] = JSON.parse(asmr.getDocumentSymbols(document.getText()), (key, val) => key === "token_type" ? asmr.SymbolType[val] : val);
        
        symbols.forEach(symbol => {
            const range = new vscode.Range(symbol.range.line_start, symbol.range.char_start, symbol.range.line_end, symbol.range.char_end);
            documentSymbols.push(new vscode.DocumentSymbol(symbol.token_name, "", symbol.token_type, range, range));
        });

        return documentSymbols;
    }
}
