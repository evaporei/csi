package main

import (
	"fmt"
	"go/ast"
	"go/token"
)

// Given an AST node corresponding to a function (guaranteed to be
// of the form `func f(x, y byte) byte`), compile it into assembly
// code.
//
// Recall from the README that the input parameters `x` and `y` should
// be read from memory addresses `1` and `2`, and the return value
// should be written to memory address `0`.
func compile(fn *ast.FuncDecl) (string, error) {
    out := ""
    for _, stmt := range fn.Body.List {
        if ret, ok := stmt.(*ast.ReturnStmt); ok {
            expr := ret.Results[0]
            if basicLit, ok := expr.(*ast.BasicLit); ok {
                out += "pushi " + basicLit.Value + "\n"
                out += "pop 0\n"
            }
            if binExpr, ok := expr.(*ast.BinaryExpr); ok {
                if ident, ok := binExpr.X.(*ast.Ident); ok {
                    if ident.Name == "x" {
                        out += "pop 1\n"
                    }
                    if ident.Name == "x" {
                        out += "pop 2\n"
                    }
                }

                if binExpr.Op == token.ADD {
                    out += "add\n"
                }

                if binExpr.Op == token.SUB {
                    out += "sub\n"
                }

                if binExpr.Op == token.MUL {
                    out += "mul\n"
                }

                if binExpr.Op == token.QUO {
                    out += "div\n"
                }

                if ident, ok := binExpr.Y.(*ast.Ident); ok {
                    if ident.Name == "x" {
                        out += "pop 1\n"
                    }
                    if ident.Name == "x" {
                        out += "pop 2\n"
                    }
                }

                out += "pop 1\n"
            }
        }
    }

    out += "halt\n"

    fmt.Println(out)

	return out, nil
}
