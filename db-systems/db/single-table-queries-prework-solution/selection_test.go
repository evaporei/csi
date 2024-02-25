package solution

import "testing"

func TestSelectionOperatorAllPass(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male"),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female"),
	}
	scanOp := NewScanOperator(tuples)
	filterOp := NewSelectionOperator(NewTrueExpression(), scanOp)

	// All tuples should be returned.
	for _, tuple := range tuples {
		assertEq(t, true, filterOp.Next())
		assertEq(t, tuple, filterOp.Execute())
	}
	assertEq(t, false, filterOp.Next())
}

func TestSelectionOperatorNonePass(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male"),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female"),
	}
	var (
		scanOp   = NewScanOperator(tuples)
		expr     = NewNotExpression(NewTrueExpression())
		filterOp = NewSelectionOperator(expr, scanOp)
	)
	// No tuples should be returned.
	assertEq(t, false, filterOp.Next())
}

func TestSelectionOperatorEQ(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male"),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female"),
	}

	var (
		scanOp   = NewScanOperator(tuples)
		expr     = NewEQExpression("gender", "female")
		filterOp = NewSelectionOperator(expr, scanOp)
	)

	// Only tuples with gender == female should be present.
	assertEq(t, true, filterOp.Next())
	assertEq(t, tuples[1], filterOp.Execute())

	assertEq(t, true, filterOp.Next())
	assertEq(t, tuples[2], filterOp.Execute())

	assertEq(t, false, filterOp.Next())
}

func TestSelectionOperatorAndEQ(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male"),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female"),
	}

	var (
		scanOp       = NewScanOperator(tuples)
		femaleExpr   = NewEQExpression("gender", "female")
		student1Expr = NewEQExpression("id", "bradfieldstudent2")
		andExpr      = NewAndExpression(femaleExpr, student1Expr)
		filterOp     = NewSelectionOperator(andExpr, scanOp)
	)

	// Only tuples with gender == female and id == bradfieldstudent2 should be present.
	assertEq(t, true, filterOp.Next())
	assertEq(t, tuples[1], filterOp.Execute())

	assertEq(t, false, filterOp.Next())
}
