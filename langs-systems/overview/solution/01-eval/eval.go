package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"log"
	"strconv"
)

func applyBinaryOp(op token.Token, x, y int) (int, error) {
	switch op {
	case token.ADD:
		return x + y, nil
	case token.SUB:
		return x - y, nil
	case token.MUL:
		return x * y, nil
	case token.QUO:
		return x / y, nil
	default:
		return 0, fmt.Errorf("unrecognized op %v", op)
	}
}

// Given an expression containing only int types, evaluate
// the expression and return the result.
func Evaluate(expr ast.Expr) (int, error) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		x, err := strconv.Atoi(e.Value)
		if err != nil {
			return 0, err
		}
		return x, nil

	case *ast.BinaryExpr:
		x, err := Evaluate(e.X)
		if err != nil {
			return 0, err
		}
		y, err := Evaluate(e.Y)
		if err != nil {
			return 0, err
		}
		return applyBinaryOp(e.Op, x, y)

	case *ast.ParenExpr:
		return Evaluate(e.X)

	default:
		return 0, fmt.Errorf("unrecognized type %T", expr)
	}
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
