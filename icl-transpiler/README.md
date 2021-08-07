The image generated with transpiler represents the same source code.
Image pixels encode program using tagged codes that have format
`<token> [arguments]`, where token encodes syntax element 
(brace, variable, number, operator etc). Only first byte (red channel) in token carries meaning,
other two bytes may be used freely and are randomly generated in this transpiler 
(so that code looks funnier).

###Token list
| Syntax element| code | argument format|
| ----| ---- | ----|
| left brace `(`| 0 | -|
| right brace `)` | 1| - |
| plus `+` | 2 | - |
| minus `-` | 3 | - |
| multiply `*` | 4 | - |
| divide `/` | 5 | - |
| variable | 6 | single pixel, (r,g,b), 24 bytes|
|print keyword| 7| -|
|number| 8| 4 pixels, (r,g,b), r then g, blue channel  is ignored|
|assign keyword | 9 | single pixel with target|
|define keyword| 10 | single pixel with target|
