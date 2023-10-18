package main

import (
	"fmt"
	"os"
	"testing"
)

func TestSortFunctions(t *testing.T) {
	const (
		numCases  = 1
		inputFmt  = "test_cases/input-%02d.txt"
		outputFmt = "test_cases/output-%02d.txt"
	)

	for i := 1; i <= numCases; i++ {
		input, err := os.ReadFile(fmt.Sprintf(inputFmt, i))
		if err != nil {
			t.Fatal(err)
		}

		expected, err := os.ReadFile(fmt.Sprintf(outputFmt, i))
		if err != nil {
			t.Fatal(err)
		}

		actual, err := SortFunctions(string(input))
		if err != nil {
			t.Fatal(err)
		}

		if actual != string(expected) {
			t.Fatalf("Bad output for test case %d\n", i)
		}
	}
}
