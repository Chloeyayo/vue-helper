import { commands, window, workspace, Position, TextEditor, TextEditorEdit, Selection, Range } from 'vscode'
import ExplorerProvider from './explorer'
import * as native from './native'

export default class Assist {
  private explorer: ExplorerProvider

  constructor(explorer: ExplorerProvider) {
    this.explorer = explorer
  }

  public register() {
    this.explorer.context.subscriptions.push(commands.registerCommand('vue-helper.blockSelect', () => {
      this.blockSelect()
    }))
    this.explorer.context.subscriptions.push(commands.registerCommand('vue-helper.funcEnhance', () => {
      this.funcEnhance()
    }))
    this.explorer.context.subscriptions.push(commands.registerCommand('vue-helper.backspace', () => {
      this.backspace()
    }))
    this.explorer.context.subscriptions.push(commands.registerCommand('vue-helper.autoImport', () => {
      this.autoImport()
    }))
    this.explorer.context.subscriptions.push(commands.registerCommand('vue-helper.autoEnhance', () => {
      this.autoEnhance()
    }))
    this.explorer.setContext('vue-helper.backspace', true)
  }

  private blockSelect() {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('blockSelect') as boolean ?? true)) return

    const editor = window.activeTextEditor
    if (!editor) return

    const lines = editor.document.getText().split('\n')
    const selection = editor.selection
    const startLine = selection.start.line
    const startChar = selection.start.character

    const range = native.computeBlockSelect(lines, startLine, startChar)
    if (range) {
      editor.selection = new Selection(
        new Position(range.startLine, range.startChar),
        new Position(range.endLine, range.endChar)
      )
    }
  }

  private funcEnhance() {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('funcEnhance') as boolean ?? true)) return

    const editor = window.activeTextEditor
    if (!editor) return

    const lines = editor.document.getText().split('\n')
    const position = editor.selection.active
    const tabSize = this.explorer.tabSize

    const result = native.computeFuncEnhance(lines, position.line, position.character, tabSize)
    if (result) {
      if (result.actionType === 'snippet') {
        const line = editor.document.lineAt(position.line)
        editor.edit((editBuilder: TextEditorEdit) => {
          editBuilder.replace(line.range, result.insertText)
        }).then(() => {
          if (result.cursorLine && result.cursorChar) {
            editor.selection = new Selection(
              new Position(result.cursorLine, result.cursorChar),
              new Position(result.cursorLine, result.cursorChar)
            )
          }
        })
      } else {
        editor.edit((editBuilder: TextEditorEdit) => {
          editBuilder.insert(position, result.insertText)
        }).then(() => {
          editor.selection = new Selection(
            new Position(result.cursorLine, result.cursorChar),
            new Position(result.cursorLine, result.cursorChar)
          )
        })
      }
    }
  }

  private backspace() {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('smartBackspace') as boolean ?? true)) {
      commands.executeCommand('deleteLeft')
      return
    }

    const editor = window.activeTextEditor
    if (!editor) return

    const lines = editor.document.getText().split('\n')
    const position = editor.selection.active

    const edit = native.computeBackspace(lines, position.line, position.character)
    if (edit) {
      editor.edit((editBuilder: TextEditorEdit) => {
        const range = new Range(
          new Position(edit.startLine, edit.startChar),
          new Position(edit.endLine, edit.endChar)
        )
        editBuilder.replace(range, edit.text)
      })
    } else {
      commands.executeCommand('deleteLeft')
    }
  }

  private autoImport() {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('autoImport') as boolean ?? true)) return

    const editor = window.activeTextEditor
    if (!editor) return

    const document = editor.document
    const selection = editor.selection
    const word = document.getText(selection)

    if (!word) return

    const normalizedWord = word.toLowerCase().replace(/-/g, '')
    const foundFile = this.explorer.vueFilesByNormalizedName.get(normalizedWord)

    if (!foundFile) return

    const componentName = this.toPascalCase(foundFile.name)

    // Check for duplicate import
    if (document.getText().includes(`import ${componentName}`)) {
      return
    }

    const importPath = foundFile.path
    const importStatement = `import ${componentName} from '${importPath}'`

    this.insertImportAndRegister(editor, componentName, importStatement)
  }

  public autoEnhance() {
    const config = workspace.getConfiguration('vue-helper')
    if (!(config.get('autoImport') as boolean ?? true) && !(config.get('funcEnhance') as boolean ?? true)) return

    const editor = window.activeTextEditor
    if (!editor) return

    const txt = editor.document.lineAt(editor.selection.anchor.line).text
    // If current line is a component tag, trigger auto import
    if (/<.*>\s?<\/.*>/gi.test(txt.trim()) || /<.*\/>/gi.test(txt.trim())) {
      if (config.get('autoImport') as boolean ?? true) {
        this.autoImportComponent(txt, editor, editor.selection.anchor.line)
      }
      return
    }
    this.funcEnhance()
  }

  private autoImportComponent(txt: string, editor: TextEditor, line: number) {
    const tag = txt.trim().replace(/<([\w-]*)[\s>].*/gi, '$1')
    for (let i = 0; i < this.explorer.vueFiles.length; i++) {
      const vf: any = this.explorer.vueFiles[i]
      if (tag === vf.name) {
        const name = this.toPascalCase(vf.name)
        // Prevent duplicate imports
        if (editor.document.getText().includes(`import ${name}`)) {
          return
        }
        const countLine = editor.document.lineCount
        // Find <script> position
        let scriptLine = line
        while (!/^\s*<script.*>\s*$/.test(editor.document.lineAt(scriptLine).text)) {
          if (countLine > scriptLine) {
            scriptLine++
          } else {
            break
          }
        }
        const activeEditorPath = this.explorer.getActiveEditorDir(editor.document.uri.path)
        let importString = `import ${name} from '${this.explorer.getVueRelativePath(activeEditorPath, vf.path)}'\n`
        importString = importString.replace(/\\/gi, '/')

        if (editor.document.lineAt(scriptLine).text.includes('setup')) {
          // Composition API
          editor.edit((editBuilder) => {
            editBuilder.insert(new Position(scriptLine + 1, 0), importString)
          })
          return
        }

        // Find import insertion line
        let importLine = scriptLine + 1
        if (editor.document.lineAt(importLine).text.includes('export default')) {
          // no-op, insert before export default
        } else {
          while (importLine < countLine && /import /gi.test(editor.document.lineAt(importLine).text.trim())) {
            importLine++
          }
        }

        this.insertImportWithComponents(editor, name, importString, importLine, countLine)
        break
      }
    }
  }

  private insertImportWithComponents(editor: TextEditor, name: string, importString: string, importLine: number, countLine: number) {
    let line = importLine
    let priorityInsertLine = 0
    let secondInsertLine = 0
    let hasComponents = false
    const tabSize = this.explorer.tabSize

    while (line < countLine && !/\s*<\/script>\s*/gi.test(editor.document.lineAt(line).text.trim())) {
      const lineText = editor.document.lineAt(line).text
      const trimmed = lineText.trim()

      // components: { ... } on single line
      if (/\s*components\s*:\s*\{.*\}.*/gi.test(trimmed)) {
        const preText = lineText.replace(/\s*}.*$/, '')
        const insertPos = preText.length
        editor.edit((editBuilder) => {
          editBuilder.insert(new Position(importLine, 0), importString)
          editBuilder.insert(new Position(line, insertPos), ', ' + name)
        })
        return
      }
      // Closing brace of components block
      if (hasComponents && /\s*},?\s*$/gi.test(trimmed)) {
        const prevText = editor.document.lineAt(line - 1).text
        const insertPos = prevText.indexOf(prevText.trim())
        let empty = ''
        for (let i = 0; i < insertPos; i++) { empty += ' ' }
        editor.edit((editBuilder) => {
          editBuilder.insert(new Position(importLine, 0), importString)
          editBuilder.insert(new Position(line - 1, prevText.length), ',\n' + empty + name)
        })
        return
      }
      // Multi-line components block
      if (/\s*components\s*:\s*\{\s*$/gi.test(trimmed)) {
        hasComponents = true
      }
      if (/\s*export\s*default\s*\{\s*/gi.test(trimmed)) {
        secondInsertLine = line
      }
      if (/\s*data\s*\(\s*\)\s*\{\s*/gi.test(trimmed)) {
        priorityInsertLine = line
      }
      line++
    }

    // No existing components section found, create one
    if (priorityInsertLine > 0) {
      editor.edit((editBuilder) => {
        editBuilder.insert(new Position(importLine - 1, 0), importString)
        editBuilder.insert(new Position(priorityInsertLine - 1, editor.document.lineAt(priorityInsertLine - 1).text.length), `\n${tabSize}components: { ${name} },`)
      })
    } else if (secondInsertLine > 0) {
      editor.edit((editBuilder) => {
        editBuilder.insert(new Position(importLine, 0), importString)
        editBuilder.insert(new Position(secondInsertLine, editor.document.lineAt(secondInsertLine).text.length), `\n${tabSize}components: { ${name} },`)
      })
    } else {
      // Fallback: just insert import
      editor.edit((editBuilder) => {
        editBuilder.insert(new Position(importLine, 0), importString)
      })
    }
  }

  private insertImportAndRegister(editor: TextEditor, componentName: string, importStatement: string) {
    const document = editor.document
    const text = document.getText()
    const lines = text.split('\n')
    let insertLine = 0
    let inScript = false

    for (let i = 0; i < lines.length; i++) {
      if (/^\s*<script/.test(lines[i])) {
        inScript = true
        insertLine = i + 1
        continue
      }
      if (inScript && lines[i].trim().startsWith('import')) {
        insertLine = i + 1
      }
    }

    // Try to find and update components section
    let componentsLine = -1
    let componentsOneLine = false
    for (let i = 0; i < lines.length; i++) {
      if (/\s*components\s*:\s*\{.*\}/.test(lines[i])) {
        componentsLine = i
        componentsOneLine = true
        break
      }
      if (/\s*components\s*:\s*\{\s*$/.test(lines[i])) {
        // Find closing brace
        for (let j = i + 1; j < lines.length; j++) {
          if (/\s*},?\s*$/.test(lines[j].trim()) && !lines[j].includes(':')) {
            componentsLine = j
            break
          }
        }
        break
      }
    }

    editor.edit((editBuilder: TextEditorEdit) => {
      editBuilder.insert(new Position(insertLine, 0), importStatement + '\n')
      if (componentsLine >= 0 && componentsOneLine) {
        const lineText = lines[componentsLine]
        const preText = lineText.replace(/\s*}.*$/, '')
        editBuilder.insert(new Position(componentsLine, preText.length), ', ' + componentName)
      } else if (componentsLine >= 0) {
        const prevText = lines[componentsLine - 1]
        const indent = prevText.indexOf(prevText.trim())
        const empty = ' '.repeat(indent)
        editBuilder.insert(new Position(componentsLine - 1, prevText.length), ',\n' + empty + componentName)
      }
    })
  }

  private toPascalCase(str: string): string {
    return str.split(/[-_]/).map(part =>
      part.charAt(0).toUpperCase() + part.slice(1)
    ).join('')
  }
}
