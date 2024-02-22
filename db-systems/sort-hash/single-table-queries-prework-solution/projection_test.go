package solution

import "testing"

func TestProjectionNone(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male",
			"age", "29",
		),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female",
			"age", "31"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female",
			"age", "42"),
	}

	var (
		scanOp   = NewScanOperator(tuples)
		selectOp = NewProjectionOperator(nil, scanOp)
	)
	// None of the fields were selected so expect 3 empty tuples.
	for range tuples {
		assertEq(t, true, selectOp.Next())
		assertEq(t, 0, len(selectOp.Execute().Values))
	}
	assertEq(t, false, selectOp.Next())
}

func TestProjectionSome(t *testing.T) {
	tuples := []Tuple{
		newTuple(
			"id", "bradfieldstudent1",
			"gender", "male",
			"age", "29",
		),
		newTuple(
			"id", "bradfieldstudent2",
			"gender", "female",
			"age", "31"),
		newTuple(
			"id", "bradfieldstudent3",
			"gender", "female",
			"age", "42"),
	}

	var (
		scanOp   = NewScanOperator(tuples)
		selectOp = NewProjectionOperator(map[string]struct{}{
			"id":  struct{}{},
			"age": struct{}{},
		}, scanOp)
	)

	// Only the "id" and "age" fields were selected so expect 3 tuples with only those
	// fields ("age" field should not be present).
	assertEq(t, true, selectOp.Next())
	assertEq(t,
		newTuple(
			"id", "bradfieldstudent1",
			"age", "29"),
		selectOp.Execute())

	assertEq(t, true, selectOp.Next())
	assertEq(t,
		newTuple(
			"id", "bradfieldstudent2",
			"age", "31"),
		selectOp.Execute())

	assertEq(t, true, selectOp.Next())
	assertEq(t,
		newTuple(
			"id", "bradfieldstudent3",
			"age", "42"),
		selectOp.Execute())

	assertEq(t, false, selectOp.Next())
}
