; Tree-sitter highlighting for Technique language
; Matching the color scheme from `technique format -R`

; Metadata/Headers - purple (pragma-like directives)
(metadata) @preproc

; Procedure declarations - blue for names, brown for types
(procedure_name) @constructor.technique
(declaration_marker) @punctuation.delimiter.technique
(signature_marker) @punctuation.delimiter.technique
(genus) @punctuation.bracket.technique

; Forma (types) - brown
(forma) @type.technique

; Parameters and variables - light blue
(variable) @variable.technique
(parameters_start_marker) @punctuation.bracket.technique
(parameters_end_marker) @punctuation.bracket.technique
(parameters_separator) @punctuation.delimiter.technique

; Titles
(title_marker) @punctuation.special.technique
(title_text) @title.technique

; Regular descriptive text - NO highlighting (default text color)
; Note: We intentionally don't highlight these so they appear as normal text
; (description)
; (inline_text)
; (step_content)

; Step markers - should be bright/bold
(step_marker) @punctuation.list_marker.technique

; Code blocks
(code_start_marker) @punctuation.bracket.technique
(code_end_marker) @punctuation.bracket.technique

; Numeric values - pink/purple
(numeric_literal) @number.technique

; String literals - green
(string_marker) @punctuation.delimiter.technique
(string_text) @string.technique

; Multiline strings - green content
(multiline_content) @string.technique
(multiline_marker) @punctuation.delimiter.technique
(multiline_language) @property.technique

; Functions - blue
(function_name) @function.technique

; Invocations - dark blue for target
(invocation_target) @function.technique
(invocation_start_marker) @punctuation.bracket.technique
(invocation_end_marker) @punctuation.bracket.technique

; Keywords - purple
(repeat_keyword) @keyword.technique
(foreach_keyword) @keyword.technique
(in_keyword) @keyword.technique

; Tablets (tables)
(tablet_start_marker) @punctuation.bracket.technique
(tablet_end_marker) @punctuation.bracket.technique
(label_marker) @punctuation.delimiter.technique
(label_text) @string.special.technique
(tablet_equals_marker) @operator.technique

; Response options - orange
(response_marker) @punctuation.delimiter.technique
(response_separator) @punctuation.delimiter.technique
(response_value) @variant.technique

; Attributes/Roles
(attribute_joiner) @operator.technique
(role_marker) @attribute.technique
(role_name) @attribute.technique

; Binding expressions
(binding_marker) @operator.technique
