# Technique language support for the Zed Editor

This repository contains machinery for syntax highlighting Technique files in
the Zed Editor, which is in the form of a language extension which contains
the extension's configuration, a Tree Sitter grammar, and the appropriate
metadata to detect the file type.

The Tree Sitter grammar here is referenced by Git URL and commit hash from
**technique-lang/tree-sitter-technique**. It is _not_ a complete parser to a
full abstract syntax tree; rather it's only requirement is to produce tokens
for the purpose of syntax highlighting a Technique file in the the Zed Editor.

In the future this repository will also be the place to specify the
integration into the Technique language server, but we'll have to write that
first. `:)`
