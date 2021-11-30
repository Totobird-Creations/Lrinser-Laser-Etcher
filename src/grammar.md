# Lrinser Grammar Format



## Lexer

Tokens:
- VARIABLE:
    - `[a-zA-Z]`
- HEADFUNC:
    - `[a-zA-Z]+`
- NUMBER:
    - `[0-9]+(\.[0-9]*)?`
- ADD:
    - `\+`
- SUBTRACT:
    - `-`
- MULTIPLY:
    - `\\*`
- DIVIDE:
    - `\\/`
- EQUALS:
    - `=`
- LPAREN:
    - `\(`
- RPAREN:
    - `\)`
- HEADER:
    - `#`
- COMMA:
    - `,`

## Parser

Layer Stack:
- parse:
    - `((header | expression) EOL+)* EOF`

- header:
    - `HEADER HEADFUNC LPAREN header_frame RPAREN`
- header_frame:
    - 

- expression:
    - `term EQUALS term`
- term:
    - `addition_term`
- addition_term:
    - `multiplication_term ((ADD|SUBTRACT) multiplication_term)*`
- multiplication_term:
    - `literal ((MULTIPLY|DIVIDE) literal)*`
- literal:
    - `((LPAREN term RPAREN) | NUMBER | VARIABLE) literal?`
