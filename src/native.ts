// Native Rust module binding for vue-helper
// This file loads the compiled Rust native addon and provides type-safe wrappers

let nativeModule: any
try {
  nativeModule = require('../vue-helper-core.node')
} catch (e: any) {
  throw new Error(`Failed to load vue-helper-core.node: ${e.message}`)
}

export interface VueFile {
  name: string
  path: string
}

export interface CompletionSuggestion {
  label: string
  sortText: string
  insertText: string
  kind: string
  detail: string
  documentation: string
}

export interface TagMatch {
  text: string
  offset: number
}

export interface WordResult {
  selectText: string
  startText: string
}

export interface SelectionRange {
  startLine: number
  startChar: number
  endLine: number
  endChar: number
}

export interface EditOperation {
  startLine: number
  startChar: number
  endLine: number
  endChar: number
  text: string
}

export interface EnhanceResult {
  insertText: string
  cursorLine: number
  cursorChar: number
  actionType: string
}

export interface DefinitionLocation {
  filePath: string
  line: number
  character: number
}

// Framework init
export function initFrameworks(packageJsonContent: string): string[] {
  return nativeModule.initFrameworks(packageJsonContent)
}

// File search
export function searchFiles(
  rootPath: string, poster: string, searchName: string,
  usePrefix: boolean, prefixAlias: string, prefixPath: string
): VueFile[] {
  return nativeModule.searchFiles(rootPath, poster, searchName, usePrefix, prefixAlias, prefixPath)
}

// Completions
export function getTagCompletions(frameworks: string[], tabSize: string, useVueSnippets: boolean): CompletionSuggestion[] {
  return nativeModule.getTagCompletions(frameworks, tabSize, useVueSnippets)
}

export function getJsTagCompletions(frameworks: string[], tabSize: string, useVueSnippets: boolean): CompletionSuggestion[] {
  return nativeModule.getJsTagCompletions(frameworks, tabSize, useVueSnippets)
}

export function getAttrCompletions(tag: string, frameworks: string[], tabSize: string, attrType: string): CompletionSuggestion[] {
  return nativeModule.getAttrCompletions(tag, frameworks, tabSize, attrType)
}

export function getAttrValueCompletions(tag: string, attr: string, frameworks: string[], tabSize: string): CompletionSuggestion[] {
  return nativeModule.getAttrValueCompletions(tag, attr, frameworks, tabSize)
}

export function getElementTagLabels(frameworks: string[], tabSize: string, extensionName: string): CompletionSuggestion[] {
  return nativeModule.getElementTagLabels(frameworks, tabSize, extensionName)
}

export function extractVueProps(fileContent: string): CompletionSuggestion[] {
  return nativeModule.extractVueProps(fileContent)
}

export function getImportSuggestions(searchText: string, vueFiles: VueFile[], documentPath: string, projectRoot: string): CompletionSuggestion[] {
  return nativeModule.getImportSuggestions(searchText, vueFiles, documentPath, projectRoot)
}

// Hover
export function provideHover(word: string, frameworks: string[], tabSize: string): string | null {
  return nativeModule.provideHover(word, frameworks, tabSize)
}

// Tag matching
export function isCloseTag(textBeforeCursor: string): boolean {
  return nativeModule.isCloseTag(textBeforeCursor)
}

export function getCloseTagName(lineText: string): string {
  return nativeModule.getCloseTagName(lineText)
}

export function matchPreTag(text: string): TagMatch | null {
  return nativeModule.matchPreTag(text)
}

export function matchPreAttr(text: string): string | null {
  return nativeModule.matchPreAttr(text)
}

// Text
export function getCurrentWordAt(text: string, character: number): string {
  return nativeModule.getCurrentWordAt(text, character)
}

export function isImportLine(text: string): boolean {
  return nativeModule.isImportLine(text)
}

export function checkNotInTemplate(lines: string[], currentLine: number): boolean {
  return nativeModule.checkNotInTemplate(lines, currentLine)
}

// Definition
export function getDefinitionPath(lineText: string): string | null {
  return nativeModule.getDefinitionPath(lineText)
}

export function resolveFilePath(basePath: string, filePath: string, projectRoot: string, isAbsolute: boolean): string | null {
  return nativeModule.resolveFilePath(basePath, filePath, projectRoot, isAbsolute)
}

export function findDefinitionInFile(lines: string[], selectText: string, startText: string): DefinitionLocation | null {
  return nativeModule.findDefinitionInFile(lines, selectText, startText)
}

// Assist
export function computeBlockSelect(lines: string[], startLine: number, startChar: number): SelectionRange | null {
  return nativeModule.computeBlockSelect(lines, startLine, startChar)
}

export function computeBackspace(lines: string[], cursorLine: number, cursorChar: number): EditOperation | null {
  return nativeModule.computeBackspace(lines, cursorLine, cursorChar)
}

export function computeFuncEnhance(lines: string[], cursorLine: number, cursorChar: number, tabSize: string): EnhanceResult | null {
  return nativeModule.computeFuncEnhance(lines, cursorLine, cursorChar, tabSize)
}

// Util
export function getWordAtPosition(lineText: string, character: number, delimiters: string[]): WordResult {
  return nativeModule.getWordAtPosition(lineText, character, delimiters)
}

// Cache invalidation
export function invalidateFrameworkCache(): void {
  return nativeModule.invalidateFrameworkCache()
}
