import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: 'aether',
    args: ['lsp'],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'aether' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.ae'),
    },
  };

  client = new LanguageClient(
    'aetherLsp',
    'Aether Language Server',
    serverOptions,
    clientOptions
  );

  client.start();

  context.subscriptions.push(
    vscode.commands.registerCommand('aether.restartServer', async () => {
      if (client) {
        await client.stop();
        client.start();
        vscode.window.showInformationMessage('Aether Language Server restarted.');
      }
    })
  );
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
