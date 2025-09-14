# Technique language support for the Zed Editor

This repository contains machinery for syntax highlighting Technique files in
the Zed Editor, which is in the form of a language extension which contains
the extension's configuration, a Tree Sitter grammar, and the appropriate
metadata to detect the file type.

The Tree Sitter grammar here is referenced by Git URL and commit hash from
**technique-lang/tree-sitter-technique**. It is _not_ a complete parser to a
full abstract syntax tree; rather it's only requirement is to produce tokens
for the purpose of syntax highlighting a Technique file in the the Zed Editor.

This repository also contains the machinery to load and run the Technique
language server which provides locations of parser errors back to the editor.

## Enabling

You can install this extension by opening the Extensions tab in the Zed Editor
and selecting "Install Dev Extension".

You will also want to set the colours used when syntax highlighting. This isn't
directly supported by Zed, but by adding entries under the
`"experimental.theme_overrides"` key in your _settings.json_ you can have Zed
use the correct colours for Technique files. This works as all the mappings in
_languages/technique/highlights.scm_ are marked as being `function.technique`
or `punctuation.bracket.technique` giving us a unique string to match on in the
user settings so that we don't corrupt ordinary theme driven highlighting for
other file types.

A suitable `"experimental.theme_overrides"` value is in
_config/settings.json-sample_ and can be copied from there into your user
settings.

You then need to configure the location of your technique binary into your
_settings.json_ file:

```json
  "lsp": {
    "technique": {
      "binary": {
        "path": "/home/andrew/.local/bin/technique",
        "arguments": ["language"],
        "env": {}
      }
    },
```

perhaps. This will be necessary until we have a stable version of Technique
with the language server protocol features released on GitHub, at which point
the extension will be able to download it itself.