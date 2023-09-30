import * as vscode from 'vscode';
import { AsmrSemanticTokensProvider, asmrSemanticTokensLegend } from './semantic_highlighter';
import { AsmrCompletionItemProvider } from './completion_provider';
import { AsmrDocumentSymbolProvider } from './document_symbol_provider';

// Called upon activation of the extension
export function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(vscode.languages.registerDocumentRangeSemanticTokensProvider({ language: 'asmr' }, new AsmrSemanticTokensProvider(), asmrSemanticTokensLegend));
	context.subscriptions.push(vscode.languages.registerCompletionItemProvider({ language: 'asmr' }, new AsmrCompletionItemProvider()));
	context.subscriptions.push(vscode.languages.registerDocumentSymbolProvider({ language: 'asmr' }, new AsmrDocumentSymbolProvider()));
}

// Called upon deactivation of the extension
export function deactivate() {}
