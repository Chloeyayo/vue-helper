import { ExtensionContext, workspace, commands, window, StatusBarAlignment, TextDocument } from 'vscode'
import * as native from './native'
import * as path from 'path'
import * as fs from 'fs'
import { outputChannel } from './client'

export interface Prefix {
  alias: string
  path: string
}

function debounce<T extends (...args: any[]) => void>(fn: T, delay: number): T {
  let timer: ReturnType<typeof setTimeout> | undefined
  return ((...args: any[]) => {
    if (timer !== undefined) { clearTimeout(timer) }
    timer = setTimeout(() => { fn(...args) }, delay)
  }) as unknown as T
}

export default class ExplorerProvider {
  public name: string = 'vue-helper'
  public context: ExtensionContext
  public projectRootPath: string = ''
  public projectRootPathReg: RegExp
  public prefix: Prefix = { alias: '@', path: 'src' }
  public config: any
  public tabSize: string = '  '
  public vueFiles: native.VueFile[] = []
  public vueFilesByNormalizedName: Map<string, native.VueFile> = new Map()
  public frameworks: string[] = []
  public isTs: boolean = false
  public inits: { init: () => void }[] = []

  private debouncedSearchVueFiles = debounce(() => this.searchVueFiles(), 300)
  private debouncedOpenDocument = debounce((e: TextDocument) => this._openDocumentCore(e), 300)

  public setContext(name: string, value: boolean) {
    commands.executeCommand('setContext', name, value)
  }

  public addInit(obj: { init: () => void }) {
    this.inits.push(obj)
  }

  public resetInit() {
    this.inits.forEach(initObj => {
      initObj.init()
    })
  }

  public getActiveEditorDir(activePath: string) {
    return activePath.replace(this.projectRootPathReg, '').replace(/[\/\\]\w*\.\w*$/gi, '')
  }

  public getVueRelativePath(activeEditorPath: string, vuePath: string) {
    let vfPath = path.relative(activeEditorPath, vuePath)
    vfPath = './' + vfPath
    return vfPath.replace(/\\/gi, '/')
  }

  constructor(context: ExtensionContext) {
    this.context = context
    this.projectRootPath = this.getWorkspaceRoot('')
    this.projectRootPathReg = new RegExp('^' + this.projectRootPath.replace(/[\\/]/g, '[\\\\/]'))

    this.config = workspace.getConfiguration('vue-helper')
    this.tabSize = this.getTabSize()

    const aliasConfig = this.config.get('alias')
    this.prefix.alias = typeof aliasConfig === 'string' ? aliasConfig : '@'
    const prefixConfig = this.config.get('componentPrefix')
    this.prefix.path = typeof prefixConfig === 'string' ? prefixConfig : 'src'

    const tsconfigPath = path.join(this.projectRootPath, 'tsconfig.json')
    this.isTs = fs.existsSync(tsconfigPath)

    this.detectFrameworks()
    this.searchVueFiles() // Initial call is synchronous (not debounced)
    this.watchFiles()

    const statusBar = window.createStatusBarItem(StatusBarAlignment.Right, -99999)
    statusBar.text = '$(extensions-view-icon) helper'
    statusBar.show()
    context.subscriptions.push(statusBar)

    workspace.onDidOpenTextDocument((e: TextDocument) => {
      this.openDocument(e)
    })
  }

  public detectFrameworks() {
    try {
      const packageJsonPath = path.join(this.projectRootPath, 'package.json')
      outputChannel.appendLine('projectRootPath: ' + this.projectRootPath)
      if (fs.existsSync(packageJsonPath)) {
        const content = fs.readFileSync(packageJsonPath, 'utf8')
        this.frameworks = native.initFrameworks(content)
        outputChannel.appendLine('detected frameworks: ' + JSON.stringify(this.frameworks))
      }
    } catch (_e: any) {
      outputChannel.appendLine('detectFrameworks error: ' + _e.message)
      this.frameworks = []
    }

    const configFrameworks = this.config.get('frameworks') as string[] | undefined
    if (configFrameworks && configFrameworks.length > 0) {
      this.frameworks = configFrameworks
    }
    outputChannel.appendLine('final frameworks: ' + JSON.stringify(this.frameworks))
  }

  public searchVueFiles() {
    const poster = (this.config.get('componentPoster') as string) || '.vue'
    this.vueFiles = native.searchFiles(
      this.projectRootPath,
      poster,
      '',
      true,
      this.prefix.alias,
      this.prefix.path
    )
    this.rebuildVueFileIndex()
  }

  private rebuildVueFileIndex() {
    this.vueFilesByNormalizedName = new Map()
    for (const vf of this.vueFiles) {
      const normalized = vf.name.toLowerCase().replace(/-/g, '')
      this.vueFilesByNormalizedName.set(normalized, vf)
    }
  }

  private watchFiles() {
    const watcher = workspace.createFileSystemWatcher('**/*.vue')
    watcher.onDidCreate(() => this.debouncedSearchVueFiles())
    watcher.onDidDelete(() => this.debouncedSearchVueFiles())
    this.context.subscriptions.push(watcher)
  }

  private openDocument(e: TextDocument) {
    let docPath = e.uri.path.replace(/.*:\//gi, '/')
    let rootNorm = this.projectRootPath.replace(/.*:\//gi, '/')
    if (!this.projectRootPath || !docPath.includes(rootNorm)) {
      this.debouncedOpenDocument(e)
    }
  }

  private _openDocumentCore(e: TextDocument) {
    this.projectRootPath = this.getWorkspaceRoot(e.uri.path)
    this.projectRootPathReg = new RegExp('^' + this.projectRootPath.replace(/[\\/]/g, '[\\\\/]'))
    this.isTs = fs.existsSync(path.join(this.projectRootPath, 'tsconfig.json'))
    native.invalidateFrameworkCache()
    this.detectFrameworks()
    this.searchVueFiles()
    this.resetInit()
  }

  private getWorkspaceRoot(documentUrl: string): string {
    let url = ''
    if (workspace.workspaceFolders?.length === 1) {
      url = workspace.workspaceFolders[0].uri.fsPath
    } else if (workspace.workspaceFolders && workspace.workspaceFolders.length > 1) {
      workspace.workspaceFolders.forEach(folder => {
        if (documentUrl.includes(folder.uri.path.replace(/.*:\//gi, '/'))) {
          url = folder.uri.fsPath
        }
      })
      if (!url) {
        const activeEditor = window.activeTextEditor
        if (activeEditor) {
          const folder = workspace.getWorkspaceFolder(activeEditor.document.uri)
          if (folder) {
            url = folder.uri.fsPath
          }
        }
      }
      if (!url) {
        url = workspace.workspaceFolders[0].uri.fsPath
      }
    }
    return url
  }

  private getTabSize(): string {
    const tabSize = workspace.getConfiguration('editor').get('tabSize') as number || 2
    return ' '.repeat(tabSize)
  }
}
