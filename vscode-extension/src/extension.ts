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
        vscode.commands.registerCommand('mp-lang.build-server', buildServer)
    );
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
        
        // 获取服务器路径
        const configServerPath = vscode.workspace.getConfiguration('mp-lang.lsp').get('server', '');
        let serverModule;
        
        if (configServerPath) {
            serverModule = configServerPath;
            outputChannel.appendLine(`使用自定义路径：${serverModule}`);
        } else {
            // 自动检测工作区中的 MP 项目
            const workspaceFolders = vscode.workspace.workspaceFolders;
            if (workspaceFolders && workspaceFolders.length > 0) {
                let workspaceRoot = workspaceFolders[0].uri.fsPath;
                
                // 查找包含 target 目录的实际项目根目录
                const path = require('path');
                const fs = require('fs');
                let projectRoot = workspaceRoot;
                let currentDir = workspaceRoot;
                let maxDepth = 5; // 最多向上查找 5 层
                let depth = 0;
                
                while (depth < maxDepth) {
                    const targetDir = path.join(currentDir, 'target');
                    if (fs.existsSync(targetDir)) {
                        projectRoot = currentDir;
                        break;
                    }
                    
                    const parentDir = path.dirname(currentDir);
                    if (parentDir === currentDir) {
                        break; // 已到达根目录
                    }
                    currentDir = parentDir;
                    depth++;
                }
                
                // 优先使用 release 版本
                const releasePath = `${projectRoot}\\target\\release\\mp-lang-lsp.exe`;
                const debugPath = `${projectRoot}\\target\\debug\\mp-lang-lsp.exe`;
                
                if (fs.existsSync(releasePath)) {
                    serverModule = releasePath;
                    outputChannel.appendLine(`✓ 找到 Release 版本：${releasePath}`);
                } else if (fs.existsSync(debugPath)) {
                    serverModule = debugPath;
                    outputChannel.appendLine(`✓ 找到 Debug 版本：${debugPath}`);
                } else {
                    // 尝试构建
                    outputChannel.appendLine('⚠️ 未找到 LSP 服务器，正在构建...');
                    workspaceRoot = projectRoot; // 使用项目根目录进行构建
                    await buildServer();
                    
                    // 重新检查
                    if (fs.existsSync(releasePath)) {
                        serverModule = releasePath;
                    } else if (fs.existsSync(debugPath)) {
                        serverModule = debugPath;
                    } else {
                        throw new Error('构建失败，未找到 LSP 服务器可执行文件');
                    }
                }
            } else {
                throw new Error('请先打开包含 MP 项目的工作区文件夹');
            }
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
        vscode.window.showErrorMessage(`MP Language LSP 启动失败：${errorMessage}`, '查看输出', '构建服务器').then(selection => {
            if (selection === '查看输出') {
                outputChannel?.show();
            } else if (selection === '构建服务器') {
                buildServer();
            }
        });
    }
}

/**
 * 构建 LSP 服务器
 */
async function buildServer() {
    if (!outputChannel) {
        outputChannel = vscode.window.createOutputChannel('MP Language', 'mp-lang');
    }
    outputChannel.appendLine('🔨 正在构建 MP Language LSP 服务器...');
    vscode.window.showInformationMessage('正在构建 LSP 服务器，请稍候...');
    
    try {
        const { exec } = require('child_process');
        const workspaceFolders = vscode.workspace.workspaceFolders;
        
        if (!workspaceFolders || workspaceFolders.length === 0) {
            throw new Error('请先打开工作区');
        }
        
        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        
        // 执行 cargo build --release
        await new Promise<void>((resolve, reject) => {
            const buildProcess = exec('cargo build --release', {
                cwd: workspaceRoot,
                env: process.env
            });
            
            buildProcess.stdout.on('data', (data: Buffer) => {
                outputChannel?.append(data.toString());
            });
            
            buildProcess.stderr.on('data', (data: Buffer) => {
                outputChannel?.append(data.toString());
            });
            
            buildProcess.on('close', (code: number | null) => {
                if (code === 0) {
                    resolve();
                } else {
                    reject(new Error(`构建失败，退出码：${code ?? -1}`));
                }
            });
        });
        
        outputChannel.appendLine('✅ 构建成功！');
        vscode.window.showInformationMessage('LSP 服务器构建成功！', '重启服务器').then(selection => {
            if (selection === '重启服务器') {
                restartServer();
            }
        });
        
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : '未知错误';
        outputChannel.appendLine(`❌ 构建失败：${errorMessage}`);
        vscode.window.showErrorMessage(`构建失败：${errorMessage}`, '查看输出');
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
