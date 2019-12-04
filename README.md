# lis2

## Grammar
```
integer     : /-?[0-9]+/ ;                                  \
decimal     : /-?[0-9]+\\.[0-9]+/ ;                         \
number      : <decimal> | <integer> ;                       \
symbol      : /[a-zA-Z0-9_+\\-*\\/\\\\=<>!&]+/ ;            \
qexpr       : '{' <expr>* '}' ;                             \
sexpr       : '(' <expr>* ')' ;                             \
expr        : <number> | <symbol> | <sexpr> | <qexpr> ;     \
program     : /^/ <expr>* /$/ ;                             \
```
