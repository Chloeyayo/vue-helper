import { ExtensionContext, window, OutputChannel } from 'vscode';
import ExplorerProvider from './explorer';
import FrameworkProvider from './framework';
import Assist from './assist';
import MonitorProvider from './monitor'

export let outputChannel: OutputChannel

export function activate(context: ExtensionContext) {
  outputChannel = window.createOutputChannel('vue-helper')
  context.subscriptions.push(outputChannel)
  outputChannel.appendLine('vue-helper activating...')
  try {
    init(context)
    outputChannel.appendLine('vue-helper activated successfully')
  } catch (e: any) {
    outputChannel.appendLine('vue-helper activation failed: ' + e.message)
    outputChannel.appendLine(e.stack || '')
    outputChannel.show()
    window.showErrorMessage('vue-helper activation failed: ' + e.message)
  }
}

function init(context: ExtensionContext) {
  const explorer = new ExplorerProvider(context)

  const framework = new FrameworkProvider(explorer)
  framework.register()

  const assist = new Assist(explorer)
  assist.register()

  new MonitorProvider(explorer)
}
