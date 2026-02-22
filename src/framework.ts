import {
  CancellationToken, CompletionContext, CompletionItem, CompletionItemProvider, CompletionList,
  Position, ProviderResult, TextDocument, languages, workspace, Range, window,
  CompletionItemKind, SnippetString, HoverProvider, Hover, DefinitionProvider,
  Definition, Uri, Location
} from "vscode"
import ExplorerProvider from './explorer'
import * as native from './native'
import * as fs from 'fs'
import * as path from 'path'

export default class FrameworkProvider {
  public explorer: ExplorerProvider
  public frameworks: string[] = []

  constructor(explorer: ExplorerProvider) {
    this.explorer = explorer
    this.init()
    this.explorer.addInit(this)
  }

  init() {
    this.frameworks = this.explorer.frameworks
  }

  register() {
    const selector = [
      { scheme: 'file', language: 'vue' },
      { scheme: 'file', language: 'javascript' },
      { scheme: 'file', language: 'typescript' },
      { scheme: 'file', language: 'html' },
      { scheme: 'file', language: 'javascriptreact' },
      { scheme: 'file', language: 'typescriptreact' },
      { scheme: 'file', language: 'wxml' }
    ]

    const completionProvider = new FrameworkCompletionItemProvider(this)
    this.explorer.context.subscriptions.push(
      languages.registerCompletionItemProvider(selector, completionProvider, '', ':', '<', '"', "'", '/', '@', '(', '>', '{')
    )

    const hoverProvider = new FrameworkHoverProvider(this)
    this.explorer.context.subscriptions.push(
      languages.registerHoverProvider(selector, hoverProvider)
    )

    const definitionProvider = new VueHelperDefinitionProvider(this)
    this.explorer.context.subscriptions.push(
      languages.registerDefinitionProvider(selector, definitionProvider)
    )

    workspace.onDidChangeTextDocument((event: any) => {
      const editor = window.activeTextEditor
      if (!editor || event.document !== editor.document) return
      if (event.contentChanges.length === 0) return

      const lastChange = event.contentChanges[event.contentChanges.length - 1]
      const lastChar = lastChange.text

      if (lastChar === '>') {
        const config = workspace.getConfiguration('vue-helper')
        if (!(config.get('autoCloseTag') as boolean ?? true)) return

        const line = editor.document.lineAt(editor.selection.active.line)
        const lineText = line.text.substring(0, editor.selection.active.character)
        if (native.isCloseTag(lineText)) {
          const tagName = native.getCloseTagName(lineText)
          const closeTag = `</${tagName}>`
          editor.insertSnippet(new SnippetString(`$0${closeTag}`), editor.selection.active)
        }
      }
    })
  }
}

class FrameworkCompletionItemProvider implements CompletionItemProvider {
  private frameworkProvider: FrameworkProvider

  constructor(frameworkProvider: FrameworkProvider) {
    this.frameworkProvider = frameworkProvider
  }

  provideCompletionItems(
    document: TextDocument,
    position: Position,
    _token: CancellationToken,
    context: CompletionContext
  ): ProviderResult<CompletionItem[] | CompletionList> {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('completion') as boolean ?? true)) return []

    const lineText = document.lineAt(position.line).text
    const textBeforeCursor = lineText.substring(0, position.character)
    const explorer = this.frameworkProvider.explorer
    const frameworks = explorer.frameworks
    const tabSize = explorer.tabSize

    const range = new Range(new Position(0, 0), position)
    const fullTextBefore = document.getText(range)

    const lines = document.getText().split('\n')
    const notInTemplate = native.checkNotInTemplate(lines, position.line)

    if (native.isImportLine(textBeforeCursor)) {
      const suggestions = native.getImportSuggestions(
        textBeforeCursor,
        explorer.vueFiles,
        document.uri.fsPath,
        explorer.projectRootPath
      )
      return suggestions.map((s: any) => {
        const item = new CompletionItem(s.label, CompletionItemKind.Reference)
        item.sortText = s.sortText
        item.insertText = new SnippetString(s.insertText)
        item.detail = s.detail
        item.documentation = s.documentation
        return item
      })
    }

    if (!notInTemplate) {
      const preTag = native.matchPreTag(fullTextBefore)
      if (preTag) {
        const preAttr = native.matchPreAttr(fullTextBefore)
        if (preAttr) {
          const attrSuggestions = native.getAttrValueCompletions(preTag.text, preAttr, frameworks, tabSize)
          return attrSuggestions.map((s: any) => {
            const item = new CompletionItem(s.label, CompletionItemKind.Value)
            item.sortText = s.sortText
            item.insertText = s.insertText
            item.detail = s.detail
            return item
          })
        }

        const triggerChar = context.triggerCharacter || textBeforeCursor.slice(-1)
        if (triggerChar === ' ' || triggerChar === ':' || triggerChar === '@') {
          const attrSuggestions = native.getAttrCompletions(preTag.text, frameworks, tabSize, triggerChar)

          const tag = preTag.text
          const normalizedTag = tag.toLowerCase().replace(/-/g, '')
          const foundFile = explorer.vueFilesByNormalizedName.get(normalizedTag)
          if (foundFile) {
            try {
              let filePath = foundFile.path
              filePath = filePath.replace(explorer.prefix.alias, explorer.prefix.path)
              const fullPath = path.join(explorer.projectRootPath, filePath)
              if (fs.existsSync(fullPath)) {
                const content = fs.readFileSync(fullPath, 'utf8')
                const propSuggestions = native.extractVueProps(content)
                attrSuggestions.push(...propSuggestions)
              }
            } catch (_e) {
              // Ignore
            }
          }

          return attrSuggestions.map((s: any) => {
            const kind = s.kind === 'method' ? CompletionItemKind.Method : CompletionItemKind.Property
            const item = new CompletionItem(s.label, kind)
            item.sortText = s.sortText
            item.insertText = s.insertText
            item.detail = s.detail
            item.documentation = s.documentation
            return item
          })
        }
      }

      const tagSuggestions = native.getTagCompletions(frameworks, tabSize, true)
      const elementLabels = native.getElementTagLabels(frameworks, tabSize, explorer.name)
      return [...tagSuggestions, ...elementLabels].map((s: any) => {
        const item = new CompletionItem(s.label, CompletionItemKind.Snippet)
        item.sortText = s.sortText
        item.insertText = new SnippetString(s.insertText)
        item.detail = s.detail
        item.documentation = s.documentation
        return item
      })
    }

    const jsSuggestions = native.getJsTagCompletions(frameworks, tabSize, true)
    return jsSuggestions.map((s: any) => {
      const item = new CompletionItem(s.label, CompletionItemKind.Snippet)
      item.sortText = s.sortText
      item.insertText = new SnippetString(s.insertText)
      item.detail = s.detail
      item.documentation = s.documentation
      return item
    })
  }
}

class FrameworkHoverProvider implements HoverProvider {
  private frameworkProvider: FrameworkProvider

  constructor(frameworkProvider: FrameworkProvider) {
    this.frameworkProvider = frameworkProvider
  }

  provideHover(document: TextDocument, position: Position, _token: CancellationToken): ProviderResult<Hover> {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('hover') as boolean ?? true)) return null

    const explorer = this.frameworkProvider.explorer
    const line = document.lineAt(position.line)
    // Use bidirectional word extraction with proper delimiters (matching original getWord behavior)
    const wordResult = native.getWordAtPosition(line.text, position.character, [' ', '<', '>', '"', "'", '.', '\\', '=', ':'])
    const word = wordResult.selectText

    if (!word) return null

    const hoverText = native.provideHover(word, explorer.frameworks, explorer.tabSize)
    if (!hoverText) return null

    return new Hover(hoverText)
  }
}

class VueHelperDefinitionProvider implements DefinitionProvider {
  private frameworkProvider: FrameworkProvider

  constructor(frameworkProvider: FrameworkProvider) {
    this.frameworkProvider = frameworkProvider
  }

  provideDefinition(document: TextDocument, position: Position, _token: CancellationToken): ProviderResult<Definition> {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('definition') as boolean ?? true)) return null

    const explorer = this.frameworkProvider.explorer
    const line = document.lineAt(position.line)
    const lineText = line.text

    // Import/require path jump
    const defPath = native.getDefinitionPath(lineText)
    if (defPath) {
      const isAbsolute = defPath.includes(explorer.prefix.alias)
      const resolvedFilePath = defPath.replace(explorer.prefix.alias, explorer.prefix.path)
      const resolved = native.resolveFilePath(
        document.uri.fsPath,
        resolvedFilePath,
        explorer.projectRootPath,
        isAbsolute
      )
      if (resolved) {
        return new Location(Uri.file(resolved), new Position(0, 0))
      }

      try {
        const nmPath = path.join(explorer.projectRootPath, 'node_modules', defPath)
        const pkgPath = path.join(nmPath, 'package.json')
        if (fs.existsSync(pkgPath)) {
          const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'))
          const mainFile = path.join(nmPath, pkg.main || 'index.js')
          if (fs.existsSync(mainFile)) {
            return new Location(Uri.file(mainFile), new Position(0, 0))
          }
        }
      } catch (_e) {
        // Ignore
      }
    }

    // For TypeScript projects, defer to VS Code's built-in TS language service
    const docText = document.getText()
    if (docText.includes('lang="ts"') || explorer.isTs) {
      return null
    }

    // In-file definition (Vue2 Options API)
    const wordResult = native.getWordAtPosition(lineText, position.character, [' ', '<', '>', '"', "'", '`', '(', ')', '.', ',', '{', '}', '[', ']', ':', ';', '=', '+', '/', '!', '?', '&', '|', '@'])
    if (wordResult.selectText) {
      const lines = document.getText().split('\n')
      const defLoc = native.findDefinitionInFile(lines, wordResult.selectText, wordResult.startText)
      if (defLoc) {
        if (defLoc.filePath) {
          const resolved = native.resolveFilePath(
            document.uri.fsPath,
            defLoc.filePath.replace(explorer.prefix.alias, explorer.prefix.path),
            explorer.projectRootPath,
            defLoc.filePath.includes(explorer.prefix.alias)
          )
          if (resolved) {
            return new Location(Uri.file(resolved), new Position(0, 0))
          }
        } else {
          return new Location(document.uri, new Position(defLoc.line, defLoc.character))
        }
      }

      // Match project vue files
      const tag = wordResult.selectText
      const normalizedTag = tag.toLowerCase().replace(/-/g, '')
      const foundFile = explorer.vueFilesByNormalizedName.get(normalizedTag)
      if (foundFile) {
        let filePath = foundFile.path
        filePath = filePath.replace(explorer.prefix.alias, explorer.prefix.path)
        const fullPath = path.join(explorer.projectRootPath, filePath)
        if (fs.existsSync(fullPath)) {
          return new Location(Uri.file(fullPath), new Position(0, 0))
        }
      }

      // Fallback: resolve element-ui / element-plus components in node_modules
      if (wordResult.startText === '<' && tag.includes('-')) {
        const loc = this.resolveFrameworkComponent(tag, explorer)
        if (loc) return loc
      }

      // Fallback: search vueFiles in memory (no glob)
      const fallbackFile = explorer.vueFilesByNormalizedName.get(normalizedTag)
      if (fallbackFile) {
        let filePath = fallbackFile.path.replace(explorer.prefix.alias, explorer.prefix.path)
        const fullPath = path.join(explorer.projectRootPath, filePath)
        if (fs.existsSync(fullPath)) {
          return new Location(Uri.file(fullPath), new Position(0, 0))
        }
      }
    }

    return null
  }

  private resolveFrameworkComponent(tag: string, explorer: ExplorerProvider): Location | null {
    const root = explorer.projectRootPath
    const componentName = tag.replace(/^el-/, '')

    const candidates: string[] = []
    if (explorer.frameworks.includes('element-ui')) {
      candidates.push(
        path.join(root, 'node_modules', 'element-ui', 'packages', componentName, 'src', 'main.vue'),
        path.join(root, 'node_modules', 'element-ui', 'packages', componentName, 'index.js'),
        path.join(root, 'node_modules', 'element-ui', 'lib', `${componentName}.js`),
      )
    }
    if (explorer.frameworks.includes('element-plus')) {
      candidates.push(
        path.join(root, 'node_modules', 'element-plus', 'es', 'components', componentName, 'index.mjs'),
        path.join(root, 'node_modules', 'element-plus', 'lib', 'components', componentName, 'index.js'),
      )
    }

    for (const candidate of candidates) {
      if (fs.existsSync(candidate)) {
        return new Location(Uri.file(candidate), new Position(0, 0))
      }
    }
    return null
  }
}
