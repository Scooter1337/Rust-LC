## Must
### General
- Read expression into character string
- Lexically analyze and parse the expression into AST
- Perform reductions until no longer possible
- Output character string corresponding to the final abstract syntax tree
- Detect syntax errors
- No external libraries
- Include makefile
- README

## Should
### General
- Report syntax error
- Least amount of STL
## May
### Reduction
- Terminate after x steps (1000) if there are still reductions possible.