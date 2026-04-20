import * as vscode from 'vscode';
import { LanguageClient, TransportKind, State, type StateChangeEvent } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;
let outputChannel: vscode.OutputChannel | undefined;

/**
 * 激活扩展 - 当打开 .mp 文件时自动启动
 */
export function activate(context: vscode.ExtensionContext) {
    outputChannel = vscode.window.createOutputChannel('MP Language', 'mp-lang');
    outputChannel.appendLine('🚀 MP Language LSP 扩展已激活');
    
    // 注册命令
    context.subscriptions.push(
        vscode.commands.registerCommand('mp-lang.restart-server', restartServer)
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('mp-lang.show-status', showStatus)
    );
    
    // 启动 LSP 服务器
    startServer(context);
}

/**
 * 启动 LSP 服务器
 */
async function startServer(_context?: vscode.ExtensionContext) {
    try {
        if (!outputChannel) {
            outputChannel = vscode.window.createOutputChannel('MP Language', 'mp-lang');
        }
        outputChannel.appendLine('📡 正在启动 MP Language LSP 服务器...');
        
        const configServerPath = vscode.workspace.getConfiguration('mp-lang.lsp').get('server', '');
        let serverModule: string;
        
        if (configServerPath) {
            serverModule = configServerPath;
            outputChannel.appendLine(`使用自定义路径：${serverModule}`);
        } else {
            serverModule = 'mp-lang-lsp';
            outputChannel.appendLine(`使用环境变量中的 LSP：${serverModule}`);
        }
        
        // 服务器选项
        const serverOptions = {
            command: serverModule,
            transport: TransportKind.stdio,
            options: {
                cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
            }
        };
        
        // 客户端选项
        const clientOptions = {
            documentSelector: [
                { scheme: 'file', language: 'mp' }
            ],
            synchronize: {
                configurationSection: 'mp-lang',
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.mp')
            },
            outputChannel: outputChannel,
            traceOutputChannel: outputChannel
        };
        
        // 创建客户端
        client = new LanguageClient(
            'mpLangLsp',
            'MP Language Server',
            serverOptions,
            clientOptions
        );
        
        // 监听状态变化
        client.onDidChangeState((event: StateChangeEvent) => {
            const stateMap: Record<number, string> = {
                [State.Stopped]: '⏹️ 已停止',
                [State.Starting]: '🔄 正在启动',
                [State.Running]: '✅ 运行中'
            };
            outputChannel?.appendLine(`状态变化：${stateMap[event.newState] ?? '未知'}`);
        });
        
        // 启动客户端
        await client.start();
        outputChannel.appendLine('✨ MP Language LSP 服务器启动成功！');
        
        // 显示通知
        vscode.window.showInformationMessage('MP Language LSP 已启动', '查看状态').then(selection => {
            if (selection === '查看状态') {
                showStatus();
            }
        });
        
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : '未知错误';
        outputChannel?.appendLine(`❌ 启动失败：${errorMessage}`);
        vscode.window.showErrorMessage(`MP Language LSP 启动失败：${errorMessage}`, '查看输出').then(selection => {
            if (selection === '查看输出') {
                outputChannel?.show();
            }
        });
    }
}

/**
 * 重启 LSP 服务器
 */
async function restartServer() {
    if (!outputChannel) {
        outputChannel = vscode.window.createOutputChannel('MP Language', 'mp-lang');
    }
    outputChannel.appendLine('🔄 正在重启 LSP 服务器...');
    
    try {
        if (client) {
            try {
                await client.stop();
                outputChannel.appendLine('⏹️ 服务器已停止');
            } catch (stopError) {
                if (stopError instanceof Error && stopError.message !== 'Canceled') {
                    outputChannel.appendLine(`⚠️ 停止服务器时出错：${stopError.message}`);
                }
            }
        }
        
        await startServer();
        
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : '未知错误';
        outputChannel.appendLine(`❌ 重启失败：${errorMessage}`);
        vscode.window.showErrorMessage(`重启失败：${errorMessage}`);
    }
}

/**
 * 显示服务器状态
 */
async function showStatus() {
    if (!client) {
        vscode.window.showWarningMessage('LSP 服务器未启动');
        return;
    }
    
    const status = client.state;
    const statusMap: Record<number, string> = {
        [0]: '⏹️ Stopped',
        [1]: '🔄 Starting',
        [2]: '✅ Running'
    };
    
    const message = `MP Language LSP 状态\n\n` +
                   `状态：${statusMap[status] ?? '未知'}\n` +
                   `服务器：${client.name}`;
    
    vscode.window.showInformationMessage(message);
}

/**
 * 停用扩展
 */
export async function deactivate() {
    if (!outputChannel) {
        outputChannel = vscode.window.createOutputChannel('MP Language', 'mp-lang');
    }
    outputChannel.appendLine('👋 正在停用 MP Language LSP...');
    
    if (client) {
        try {
            await client.stop();
            outputChannel.appendLine('✅ LSP 服务器已停止');
        } catch (error) {
            // 忽略取消错误，这在停用时是正常的
            if (error instanceof Error && error.message !== 'Canceled') {
                outputChannel.appendLine(`⚠️ 停止服务器时出错：${error.message}`);
            }
        }
    }
    
    outputChannel.dispose();
}
