## Must

### General

- Read expression into character string ✅
- Lexically analyze and parse the expression into AST ✅
- Perform reductions until no longer possible ✅
- Output character string corresponding to the final abstract syntax tree ✅
- Detect syntax errors ✅
- No external libraries ✅
- Include makefile ✅
- README ✅
- Compilable ✅
- Work for files with ascii characters ✅
- README must document what reduction strategies are implemented ✅
- must exit with exit status 0 when an expression cannot be reduced any further by a beta-reduction ✅
- must exit with 1 when there is a syntax error or cli error ✅
- must be interruptable by the OS ✅
- If status 0 the program must output the reduced AST ✅

## Should

### General

- Report syntax error ✅
- Least amount of STL possible ✅
- Support only one expression in input file ✅

### Reductions

- alpha conversions should only be performed if a beta-reduction would otherwise lead to a captured variable. ✅

## May

### General

- Explanation of how the program works ✅
- positive and negative examples ✅
- Non printable ascii characters support ✅

### Reductions

- Terminate after x steps (1000) if there are still reductions possible. ✅
- Reduction strategy may be configured ❌ (WILL NOT IMPLEMENT)
- program may exit with status 2 when a limit on the number of steps has been reached ✅
- program may print error message on error 1 or 2 ✅
- may print understandable error messages ✅
