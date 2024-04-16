## Must

### General

- read expression ✅
- lex expression ✅
- parse tokens ✅
- output string ✅
- detect syntax errors ✅
- no external dependencies ✅
- include README.md ✅
- compilable with `make` ✅
- cli interface ✅
- exit with '0' when all expressions are valid ✅
- exit with '1' when any expression is invalid ✅
- must terminate ✅
- must output if 0 ✅
- must error if 1 ✅

### Grammar

- Start with latin alphabet ✅
- Followed by alphanumeric characters ✅
- Seperated by spaces ✅
- Parentheses are supported ✅
- reparsing output yields the same result (reparse) ✅

### Examples

- each expression seperately ✅
- each expression in one go ✅

## Should

### General

- report error ✅
- minimize stdlib usage ✅
- terminate instantly on error ✅

### Grammar

- whitespace insensitive ✅
- should result in same output on second run ✅
- unambiguous output ✅
- multiple expressions on new lines ✅

## May

- Explain format in README ✅
- program explanation in README ✅
- positive and negative examples ✅/✅
- may accept multiple expressions, one per line ✅
- error message may be printed on error ✅
- Program may support unicode ✅
- Program may support dot notation ✅
- Use least amount of whitespace and parentheses ✅
