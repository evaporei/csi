package solution

import "testing"

func TestLimitOperatorLessTuplesThanLimit(t *testing.T) {
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
	limitOp := NewLimitOperator(len(tuples)+1, scanOp)

	// All tuples should be returned.
	for _, tuple := range tuples {
		assertEq(t, true, limitOp.Next())
		assertEq(t, tuple, limitOp.Execute())
	}
	assertEq(t, false, limitOp.Next())
}

func TestLimitOperatorMoreTuplesThanLimit(t *testing.T) {
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
	limitOp := NewLimitOperator(len(tuples)-1, scanOp)

	// All but the last tuple should be returned.
	for _, tuple := range tuples[:len(tuples)-1] {
		assertEq(t, true, limitOp.Next())
		assertEq(t, tuple, limitOp.Execute())
	}
	assertEq(t, false, limitOp.Next())
}
