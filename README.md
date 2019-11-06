# Instant Compiler

Instant Compiler is a JVM & LLVM compiler of a simple language consisting of a assignments and arithmetic expressions. 
It's written in Rust and uses [LARPOP](https://github.com/lalrpop/lalrpop) to generate a parser.

## Grammar

Instant grammar in LALRPOP format is available [here](parser/src/grammar.lalrpop).

BNFC grammar below for visual purposes only:

```
Program.                Program ::= [Statement] ;
AssignmentStatement.    Statement ::= Ident "=" Expression ;
ExpressionStatement.    Statement ::= Expression ;
separator Statement ";" ;

Addition.               Expression1   ::= Expression2 "+"  Expression1 ;
Subtraction.            Expression2   ::= Expression2 "-"  Expression3 ;
Multiplication.         Expression3   ::= Expression3 "*"  Expression4 ;
Division.               Expression3   ::= Expression3 "/"  Expression4 ;
Literal.                Expression4   ::= Integer ;
Variable.               Expression4   ::= Ident ;
coercions Expression 4;
```

## Project setup
```
cargo build
```
Downloads all dependencies and generates executables: `target/debug/jvm` and `target/debug/llvm`

### Print AST
```
cargo run < <source_file_path>
```

### Compile for JVM
```
./insc_jvm <source_file_path>
```

#### Example:
```
./insc_jvm foo/bar/file.ins
```
Generates `foo/bar/file.j` (text bytecode representation) and `foo/bar/file.class` (bytecode executable)


### Compile for LLVM
```
./insc_llvm <source_file_path>
```

#### Example:
```
./insc_jvm foo/bar/file.ins
```
Generates `foo/bar/file.ll` (text bytecode representation) and `foo/bar/file.bc` (bitcode executable)

## Project structure
```
├── compiler    # crate for jvm & llvm compiler
├── lib         # jasmin.jar & 
├── parser      # crate for grammar and ast
├── src         
│   └── bin     # jvm & llvm executable sources
├── target
│   └── debug   # jvm & llvm binaries
└── tests       # test files
```
