import * as vscode from 'vscode';
import * as asmr from './asmr/asmr_lsp';

export class AsmrCompletionItemProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken, context: vscode.CompletionContext): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList<vscode.CompletionItem>> {
        const builder = new vscode.CompletionList<vscode.CompletionItem>();
        const tokens: asmr.CompletionItem[] = JSON.parse(asmr.getCompletionItems(document.getText()), (key, val) => key === "token_type" ? asmr.CompletionType[val] : val);

        tokens.forEach(token => {
            let completionItem = new vscode.CompletionItem(token.token_name);

            switch (token.token_type) {
                case asmr.CompletionType.Variable:
                    completionItem.kind = vscode.CompletionItemKind.Variable;
                    break;
                case asmr.CompletionType.Function:
                    completionItem.kind = vscode.CompletionItemKind.Function;
                    break;
                case asmr.CompletionType.Label:
                    completionItem.kind = vscode.CompletionItemKind.Reference;
                    break;

                default:
                    throw new Error(`Invalid CompletionItem type encountered.`);
            }

            builder.items.push(completionItem);
        });

        return builder;
    }
}
