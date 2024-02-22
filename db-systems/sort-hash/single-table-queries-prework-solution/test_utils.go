package solution

import (
	"fmt"
	"reflect"
	"testing"
)

func assertEq(t *testing.T, a, b interface{}) {
	if !reflect.DeepEqual(a, b) {
		t.Fatalf("expected: %v to equal: %v", a, b)
	}
}

func newTuple(inputs ...interface{}) Tuple {
	if len(inputs)%2 != 0 {
		panic(fmt.Sprintf("num inputs must be even, but was: %d", len(inputs)))
	}

	tuple := Tuple{
		Values: make([]Value, 0, len(inputs)/2),
	}

	for i := 0; i < len(inputs); i += 2 {
		tuple.Values = append(tuple.Values, Value{
			Name:        inputs[i].(string),
			StringValue: inputs[i+1].(string),
		})
	}

	return tuple
}
