const vscode = require('vscode');
const { spawn } = require('child_process');

let lspProcess = null;

function activate(context) {
    console.log('Sorayunara Language Support extension activated.');

    const disposable = vscode.languages.registerDocumentFormattingEditProvider(['sorayunara', 'aoi', 'nami', 'aether'], {
        provideDocumentFormattingEdits(document) {
            return [];
        }
    });

    context.subscriptions.push(disposable);
}

function deactivate() {
    if (lspProcess) {
        lspProcess.kill();
    }
}

module.exports = {
    activate,
    deactivate
};
