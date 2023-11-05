package main

import (
    "go/ast"
    "go/parser"
    "go/token"
    "log"
    "strconv"
)

// example:
//
//  0  *ast.BinaryExpr {
//  1  .  X: *ast.BasicLit {
//  2  .  .  ValuePos: -
//  3  .  .  Kind: INT
//  4  .  .  Value: "4"
//  5  .  }
//  6  .  OpPos: -
//  7  .  Op: *
//  8  .  Y: *ast.ParenExpr {
//  9  .  .  Lparen: -
// 10  .  .  X: *ast.BinaryExpr {
// 11  .  .  .  X: *ast.BasicLit {
// 12  .  .  .  .  ValuePos: -
// 13  .  .  .  .  Kind: INT
// 14  .  .  .  .  Value: "5"
// 15  .  .  .  }
// 16  .  .  .  OpPos: -
// 17  .  .  .  Op: +
// 18  .  .  .  Y: *ast.BasicLit {
// 19  .  .  .  .  ValuePos: -
// 20  .  .  .  .  Kind: INT
// 21  .  .  .  .  Value: "6"
// 22  .  .  .  }
// 23  .  .  }
// 24  .  .  Rparen: -
// 25  .  }
// 26  }

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

    if parenExpr, ok := expr.(*ast.ParenExpr); ok {
        return Evaluate(parenExpr.X)
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
	// expr, err := parser.ParseExpr("(3 - 4 * (5 + 6) + 7)")
	expr, err := parser.ParseExpr("4 * (5 + 6)")
	if err != nil {
		log.Fatal(err)
	}
	fset := token.NewFileSet()
	err = ast.Print(fset, expr)
	if err != nil {
		log.Fatal(err)
	}
}
