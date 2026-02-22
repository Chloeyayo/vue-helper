import { workspace } from 'vscode'

export function getTabSize(): string {
  const editor = workspace.getConfiguration('editor')
  const tabSize = editor.get('tabSize') as number || 2
  return ' '.repeat(tabSize)
}

export function getWorkspaceRoot(): string {
  let url = ''
  if (workspace.workspaceFolders?.length === 1) {
    url = workspace.workspaceFolders[0].uri.fsPath
  } else if (workspace.workspaceFolders && workspace.workspaceFolders.length > 1) {
    url = workspace.workspaceFolders[0].uri.fsPath
  }
  return url
}