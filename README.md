# Introduction of this tool
This tool helps to abstract text from multiple html source, e.g, the code below outputs the provinces list of Canada: 
```
dessert3 'The list of provinces of Canada: {{loop selector='div.field-item > ul > li > a' doc="https://www.statcan.gc.ca/en/reference/province" var="province"}}
- {{var province}}
{{end}}'
```
# Versus V2
This version is much more flexible and easier than the last version v2. It parses the output from the template that you give. It does care if you want to output json, yaml, or any other formation you want.
# How to use
By now, we support `loop`, and `css`.
`loop` supposes there are multiple node selected by the css selector, it iterates each of the selected value, renders the children tokens entil the `{{end}}`.
`css` takes the first selected node, then render the output. The default value will be rendered if there is no matched node by the selector.
String can appear between the above tokens.
The `loop` and `css` can take those parameters:
- 'selector'. The css selector.
- 'doc'. The url of the source.
- 'doc-var'. The variable that is taken from the parent token. It can combine with `base-doc` if the url does not start with "http://" or "https://".
# License
MIT
