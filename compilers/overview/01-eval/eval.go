package main

import (
    "go/ast"
    "go/parser"
    "go/token"
    "log"
    "strconv"
)

// - integer literals
// - basic arithmetic (+ - * /)
// - parenthesis (grouping)

// Given an expression containing only int types, evaluate
// the expression and return the result.
func Evaluate(expr ast.Expr) (int, error) {
    if basicLit, ok := expr.(*ast.BasicLit); ok {
        // assume it's an integer (basicLit.Kind == INT)
        return strconv.Atoi(basicLit.Value)
    }

    if binExpr, ok := expr.(*ast.BinaryExpr); ok {
        x, err := Evaluate(binExpr.X)
        if err != nil {
            return 0, err
        }

        y, err := Evaluate(binExpr.Y)
        if err != nil {
            return 0, err
        }

        if binExpr.Op == token.ADD {
            return x + y, nil
        }

        if binExpr.Op == token.SUB {
            return x - y, nil
        }

        if binExpr.Op == token.MUL {
            return x * y, nil
        }

        if binExpr.Op == token.QUO {
            return x / y, nil
        }
    }

	return 0, nil
}

func main() {
	expr, err := parser.ParseExpr("1 + 2 - 3 * 4")
	if err != nil {
		log.Fatal(err)
	}
	fset := token.NewFileSet()
	err = ast.Print(fset, expr)
	if err != nil {
		log.Fatal(err)
	}
}
