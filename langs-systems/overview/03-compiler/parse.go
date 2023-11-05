package main

import (
	"go/ast"
	"go/parser"
	"go/token"
)

// Given a complete Go source file, parses it and returns the AST
// node corresponding to the body of the function with the given name.
func parse(src, name string) (*ast.FuncDecl, error) {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "", src, 0)
	if err != nil {
		return nil, err
	}
	for _, decl := range f.Decls {
		if fd, ok := decl.(*ast.FuncDecl); ok {
			if fd.Name.Name == name {
				return fd, nil
			}
		}
	}
	return nil, nil
}
