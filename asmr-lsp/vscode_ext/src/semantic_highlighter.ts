import * as vscode from 'vscode';
import * as asmr from './asmr/asmr_lsp';

export const asmrSemanticTokensLegend = new vscode.SemanticTokensLegend(
    ['variable', 'function'],
    ['declaration', 'definition', 'defaultLibrary'],
);

export class AsmrSemanticTokensProvider implements vscode.DocumentRangeSemanticTokensProvider {
    async provideDocumentRangeSemanticTokens(document: vscode.TextDocument, range: vscode.Range, token: vscode.CancellationToken): Promise<vscode.SemanticTokens> {
        const builder = new vscode.SemanticTokensBuilder(asmrSemanticTokensLegend);
        const tokens: asmr.SemanticToken[] = JSON.parse(asmr.parseFileTokens(document.getText()));

        tokens.forEach(token =>
            builder.push(token.delta_line, token.delta_start, token.length, token.token_type));

        return builder.build();
    }
}
