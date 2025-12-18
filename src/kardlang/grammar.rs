pub const GRAMMAR: &str = r#"
Kardlang v0 grammar (EBNF-ish)

program   := (call (sep call)*)? ;
sep       := ';' | whitespace ;

call      := ident '(' (expr (',' expr)*)? ')' ;

expr      := add ;
add       := mul ( '+' mul )* ;
mul       := primary ( '*' primary )* ;

primary   := number
          | ident
          | '(' expr ')' ;

number    := unary | digit ;
unary     := '1'+ ;
digit     := '0' | '2'..'9' ;

ident     := [A-Za-z_][A-Za-z0-9_]* ;

Notes
* All values are integers.
* Digit shorthand is intentionally single-digit (no multi-digit literals).
* Cost model (enforced at the card level): each character costs 1, except digits cost their numeric value (0 costs 1).
"#;
