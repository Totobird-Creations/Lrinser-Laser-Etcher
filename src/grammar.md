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


- parse:
    - `((header | expression) EOL+)* EOF`


- header:
    - `HEADER HEADFUNC LPAREN arguments RPAREN`


- function:
    - `FUNCTION LPAREN arguments RPAREN RPAREN`


- expression:
    - `term EQUALS term`

- term:
    - `addition_term`

- addition_term:
    - `multiplication_term ((ADD|SUBTRACT) multiplication_term)*`

- multiplication_term:
    - `literal_multiplication ((MULTIPLY|DIVIDE) literal_multiplication)*`

- literal_multiplication:
    - `literal+`

- literal:
    - `LPAREN term LPAREN`
    - `NUMBER`
    - `VARIABLE`
    - `FUNCTION`
