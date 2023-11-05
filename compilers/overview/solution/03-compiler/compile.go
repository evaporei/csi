package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/token"
	"os"
)

var tokenToAsm = map[token.Token]string{
	token.ADD: "add",
	token.SUB: "sub",
	token.MUL: "mul",
	token.QUO: "div",

	token.EQL: "eq",
	token.LSS: "lt",
	token.GTR: "gt",
	token.NEQ: "neq",
	token.LEQ: "leq",
	token.GEQ: "geq",
}

type scope struct {
	idToLoc map[string]byte
	nextLoc byte
}

func (s *scope) compileExpr(expr ast.Expr, buf *bytes.Buffer) error {
	var err error
	switch expr := expr.(type) {
	case *ast.BasicLit:
		buf.WriteString("pushi " + expr.Value + "\n")
		return nil
	case *ast.BinaryExpr:
		err = s.compileExpr(expr.X, buf)
		if err != nil {
			return err
		}
		err = s.compileExpr(expr.Y, buf)
		if err != nil {
			return err
		}
		buf.WriteString(tokenToAsm[expr.Op] + "\n")
		return nil
	case *ast.Ident:
		buf.WriteString(fmt.Sprintf("push %d\n", s.idToLoc[expr.Name]))
		return nil
	case *ast.ParenExpr:
		return s.compileExpr(expr.X, buf)
	default:
		return fmt.Errorf("unrecognized type %T", expr)
	}
}

func (s *scope) compileStmt(stmt ast.Stmt, buf *bytes.Buffer) error {
	var err error
	switch stmt := stmt.(type) {
	case *ast.AssignStmt:
		// TODO: Handle more general assignment statements
		if len(stmt.Lhs) != 1 || len(stmt.Rhs) != 1 {
			return fmt.Errorf("expected exactly one expression on both sides of assignment")
		}
		id, ok := stmt.Lhs[0].(*ast.Ident)
		if !ok {
			return fmt.Errorf("expected lhs of assignment to be identifier; got %T", stmt.Lhs[0])
		}
		err = s.compileExpr(stmt.Rhs[0], buf)
		if err != nil {
			return err
		}
		buf.WriteString(fmt.Sprintf("pop %d\n", s.idToLoc[id.Name]))
		return nil
	case *ast.BlockStmt:
		for _, item := range stmt.List {
			err = s.compileStmt(item, buf)
			if err != nil {
				return err
			}
		}
		return nil
	case *ast.DeclStmt:
		// TODO:
		// - Enforce limit on number of variables
		// - Make sure the type is correct (byte)
		// - Handle multiple declarations, e.g. `var a, b, c byte`
		name := stmt.Decl.(*ast.GenDecl).Specs[0].(*ast.ValueSpec).Names[0].Name
		s.idToLoc[name] = s.nextLoc
		s.nextLoc++
		return nil
	case *ast.ForStmt:
		// TODO: Handle initialization and increment, not just loop condition
		labelBefore := fmt.Sprintf("for-%d-before", stmt.For)
		labelAfter := fmt.Sprintf("for-%d-after", stmt.For)
		buf.WriteString("label " + labelBefore + "\n")
		err = s.compileExpr(stmt.Cond, buf)
		if err != nil {
			return err
		}
		buf.WriteString("jeqz " + labelAfter + "\n")
		err = s.compileStmt(stmt.Body, buf)
		if err != nil {
			return err
		}
		buf.WriteString("jump " + labelBefore + "\n")
		buf.WriteString("label " + labelAfter + "\n")
		return nil
	case *ast.IfStmt:
		labelElse := fmt.Sprintf("if-%d-else", stmt.If)
		labelAfter := fmt.Sprintf("if-%d-after", stmt.If)
		err = s.compileExpr(stmt.Cond, buf)
		if err != nil {
			return err
		}
		if stmt.Else != nil {
			buf.WriteString("jeqz " + labelElse + "\n")
		} else {
			buf.WriteString("jeqz " + labelAfter + "\n")
		}
		err = s.compileStmt(stmt.Body, buf)
		if err != nil {
			return err
		}
		if stmt.Else != nil {
			buf.WriteString("jump " + labelAfter + "\n")
			buf.WriteString("label " + labelElse + "\n")
			err = s.compileStmt(stmt.Else, buf)
			if err != nil {
				return err
			}
		}
		buf.WriteString("label " + labelAfter + "\n")
		return nil
	case *ast.ReturnStmt:
		if len(stmt.Results) != 1 {
			return fmt.Errorf("expected exactly one return value; got %d", len(stmt.Results))
		}
		err = s.compileExpr(stmt.Results[0], buf)
		if err != nil {
			return err
		}
		buf.WriteString("pop 0\n")
		buf.WriteString("halt\n")
		return nil
	default:
		ast.Fprint(os.Stdout, nil, stmt, nil)
		return fmt.Errorf("unrecognized type %T", stmt)
	}
}

func compile(node *ast.FuncDecl) (string, error) {
	idToLoc := make(map[string]byte)
	nextLoc := byte(1)
	for _, field := range node.Type.Params.List {
		for _, name := range field.Names {
			idToLoc[name.Name] = nextLoc
			nextLoc++
		}
	}
	s := scope{
		idToLoc: idToLoc,
		nextLoc: nextLoc,
	}
	var buf bytes.Buffer
	err := s.compileStmt(node.Body, &buf)
	if err != nil {
		return "", err
	}
	return buf.String(), nil
}
