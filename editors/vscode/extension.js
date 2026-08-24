const vscode = require('vscode');
const { spawn, execFile } = require('child_process');

let lspProcess = null;
let outputChannel = null;

function getExecutablePath() {
    const config = vscode.workspace.getConfiguration('sorayunara');
    return config.get('executablePath') || 'sorayunara';
}

function startLspServer() {
    if (lspProcess) {
        lspProcess.kill();
        lspProcess = null;
    }

    const config = vscode.workspace.getConfiguration('sorayunara');
    if (!config.get('enableLsp', true)) {
        return;
    }

    const exe = getExecutablePath();
    try {
        lspProcess = spawn(exe, ['lsp'], { stdio: ['pipe', 'pipe', 'pipe'] });
        lspProcess.stderr.on('data', (data) => {
            if (outputChannel) {
                outputChannel.appendLine(`[LSP] ${data.toString().trim()}`);
            }
        });
        lspProcess.on('error', (err) => {
            if (outputChannel) {
                outputChannel.appendLine(`[LSP Error] Failed to start: ${err.message}`);
            }
        });
        if (outputChannel) {
            outputChannel.appendLine('[LSP] Sorayunara LSP daemon spawned successfully.');
        }
    } catch (err) {
        if (outputChannel) {
            outputChannel.appendLine(`[LSP Launch Error] ${err.message}`);
        }
    }
}

function runTerminalCommand(title, args) {
    const terminal = vscode.window.createTerminal(`Sorayunara: ${title}`);
    const exe = getExecutablePath();
    terminal.show();
    terminal.sendText(`${exe} ${args.join(' ')}`);
}

function activate(context) {
    outputChannel = vscode.window.createOutputChannel('Sorayunara');
    outputChannel.appendLine('🌌 Sorayunara Language Support Extension Active.');

    // LSP Daemon
    startLspServer();

    // Document Formatter Provider
    const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider('sorayunara', {
        provideDocumentFormattingEdits(document) {
            return new Promise((resolve) => {
                const exe = getExecutablePath();
                const filePath = document.uri.fsPath;
                execFile(exe, ['fmt', filePath], (error, stdout, stderr) => {
                    if (error) {
                        outputChannel.appendLine(`[Format Error] ${stderr || error.message}`);
                        resolve([]);
                        return;
                    }
                    // If file is formatted in place or stdout is provided
                    resolve([]);
                });
            });
        }
    });
    context.subscriptions.push(formattingProvider);

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('sorayunara.run', () => {
            const editor = vscode.window.activeTextEditor;
            const file = editor ? editor.document.fileName : 'main.sora';
            runTerminalCommand('Run', ['run', `"${file}"`]);
        }),
        vscode.commands.registerCommand('sorayunara.build', () => {
            const editor = vscode.window.activeTextEditor;
            const file = editor ? editor.document.fileName : 'main.sora';
            runTerminalCommand('Build', ['build', `"${file}"`]);
        }),
        vscode.commands.registerCommand('sorayunara.test', () => {
            runTerminalCommand('Test', ['test']);
        }),
        vscode.commands.registerCommand('sorayunara.check', () => {
            const editor = vscode.window.activeTextEditor;
            const file = editor ? editor.document.fileName : 'main.sora';
            runTerminalCommand('Check', ['check', `"${file}"`]);
        }),
        vscode.commands.registerCommand('sorayunara.format', () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const file = editor.document.fileName;
                runTerminalCommand('Format', ['fmt', `"${file}"`]);
            }
        }),
        vscode.commands.registerCommand('sorayunara.debug', () => {
            const editor = vscode.window.activeTextEditor;
            const file = editor ? editor.document.fileName : 'main.sora';
            runTerminalCommand('Debug', ['debug', `"${file}"`]);
        }),
        vscode.commands.registerCommand('sorayunara.restartLsp', () => {
            startLspServer();
            vscode.window.showInformationMessage('Sorayunara LSP server restarted.');
        })
    );
}

function deactivate() {
    if (lspProcess) {
        lspProcess.kill();
        lspProcess = null;
    }
    if (outputChannel) {
        outputChannel.dispose();
    }
}

module.exports = {
    activate,
    deactivate
};
