package solution

import (
	"testing"
)

func TestScanNoTuples(t *testing.T) {
	op := NewScanOperator(nil)
	assertEq(t, false, op.Next())
}

func TestScanWithTuples(t *testing.T) {
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
	op := NewScanOperator(tuples)

	for _, tuple := range tuples {
		assertEq(t, true, op.Next())
		assertEq(t, tuple, op.Execute())
	}
	assertEq(t, false, op.Next())
}
