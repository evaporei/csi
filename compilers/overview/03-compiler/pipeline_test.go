package main

import (
	"testing"
)

type funcExample struct {
	x, y, expected byte
}

type testCase struct {
	name     string
	src      string
	examples []funcExample
}

var testCases = []testCase{
	{
		name: "Constant",
		src: `package f

func f(x, y byte) byte {
	return 5
}`,
		examples: []funcExample{
			{1, 1, 5},
			{2, 1, 5},
			{7, 19, 5},
		},
	},
	{
		name: "Parameters",
		src: `package f

func f(x, y byte) byte {
	return x + y
}`,
		examples: []funcExample{
			{1, 1, 2},
			{2, 3, 5},
			{5, 8, 13},
		},
	},
	{
		name: "More Complicated Expression",
		src: `package f

func f(x, y byte) byte {
	return 2 * (x + 3) * (y + 4)
}`,
		examples: []funcExample{
			{1, 1, 40},
			{2, 2, 60},
		},
	},
	{
		name: "Multiple Statements",
		src: `package f

func f(x, y byte) byte {
	x = x + 3
	y = y + 4
	return 2 * x * y
}`,
		examples: []funcExample{
			{1, 1, 40},
			{2, 2, 60},
		},
	},
	{
		name: "If/Else",
		src: `package f

func f(x, y byte) byte {
	if x == 0 {
		return y
	} else {
		return x
	}
}`,
		examples: []funcExample{
			{0, 2, 2},
			{1, 2, 1},
		},
	},
	{
		name: "Local Variables 1",
		src: `package f

func f(x, y byte) byte {
	var z byte
	z = x + y
	z = z * z
	return z
}`,
		examples: []funcExample{
			{1, 2, 9},
			{2, 3, 25},
		},
	},
	{
		name: "Local Variables 2",
		src: `package f

func f(x, y byte) byte {
	var z byte
	z = x
	x = y
	y = z
	return x
}`,
		examples: []funcExample{
			{1, 1, 1},
			{1, 2, 2},
			{2, 1, 1},
			{3, 15, 15},
		},
	},
	{
		name: "Local Variables 3",
		src: `package f

func f(x, y byte) byte {
	var a byte
	var b byte
	a = x * y
	b = x + y
	if a < b {
		return b - a
	} else {
		return a - b
	}
}`,
		examples: []funcExample{
			{1, 2, 1},
			{1, 7, 1},
			{2, 2, 0},
			{2, 3, 1},
			{4, 4, 8},
			{4, 5, 11},
		},
	},
	{
		name: "Fibonacci",
		src: `package f

func f(x, y byte) byte {
	var a byte
	var b byte
	var c byte
	a = 0
	b = 1
	for x > 0 {
		c = a + b
		a = b
		b = c
		x = x - 1
	}
	return b
}`,
		examples: []funcExample{
			{1, 0, 1},
			{2, 0, 2},
			{3, 0, 3},
			{4, 0, 5},
			{5, 0, 8},
			{6, 0, 13},
			{7, 0, 21},
			{8, 0, 34},
			{9, 0, 55},
		},
	},
}

func TestPipeline(t *testing.T) {
	for _, test := range testCases {
		t.Run(test.name, func(t *testing.T) { testPipeline(t, test) })
	}
}

// Given some Go code and test cases, run bytecode generation,
// then execute the resulting bytecode in the virtual machine
func testPipeline(t *testing.T, test testCase) {
	bytecode, err := generateBytecode(test.src)
	if err != nil {
		t.Fatal(err)
	}

	// For each case, set inputs and run VM
	for _, e := range test.examples {
		result, err := runVM(bytecode, e.x, e.y)
		if err != nil {
			t.Fatal(err)
		}
		if result != e.expected {
			t.Fatalf("Expected f(%d, %d) to be %d, got %d", e.x, e.y, e.expected, result)
		}
	}
}
