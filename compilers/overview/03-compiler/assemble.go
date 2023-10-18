package main

import (
	"fmt"
	"strconv"
	"strings"
)

const (
	instructionStart byte = 0x08
	parameterStart   byte = 0x01
)

var instructionSize = map[string]byte{
	"push":  2,
	"pushi": 2,
	"pop":   2,

	"add": 1,
	"sub": 1,
	"mul": 1,
	"div": 1,

	"eq":  1,
	"lt":  1,
	"gt":  1,
	"neq": 1,
	"leq": 1,
	"geq": 1,

	"jump": 2,
	"jeqz": 2,

	"halt": 1,
}

func mem(s string) (b byte) {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return byte(i)
}

func imm(s string) (b byte) {
	// for now, immediate values and memory addresses are both just ints
	return mem(s)
}

// Assemble the given assembly code to bytecode
func assemble(asm string) ([]byte, error) {
	bc := []byte{}
	asm = strings.TrimSpace(asm)

	labels := make(map[string]byte)
	addr := instructionStart

	// First pass just collects all label addresses
	for _, line := range strings.Split(asm, "\n") {
		parts := strings.Split(strings.TrimSpace(line), " ")
		op := parts[0]
		if op == "label" {
			labels[parts[1]] = addr
		} else if size, ok := instructionSize[op]; ok {
			addr += size
		} else {
			return nil, fmt.Errorf("Invalid operation: %v", op)
		}
	}

	// Second pass actually generates bytecode
	for _, line := range strings.Split(asm, "\n") {
		parts := strings.Split(strings.TrimSpace(line), " ")
		switch parts[0] {
		case "push":
			bc = append(bc, []byte{0x01, mem(parts[1])}...)
		case "pushi":
			bc = append(bc, []byte{0x02, imm(parts[1])}...)
		case "pop":
			bc = append(bc, []byte{0x03, mem(parts[1])}...)

		case "add":
			bc = append(bc, byte(0x10))
		case "sub":
			bc = append(bc, byte(0x11))
		case "mul":
			bc = append(bc, byte(0x12))
		case "div":
			bc = append(bc, byte(0x13))

		case "eq":
			bc = append(bc, byte(0x14))
		case "lt":
			bc = append(bc, byte(0x15))
		case "gt":
			bc = append(bc, byte(0x16))
		case "neq":
			bc = append(bc, byte(0x17))
		case "leq":
			bc = append(bc, byte(0x18))
		case "geq":
			bc = append(bc, byte(0x19))

		case "jump":
			bc = append(bc, []byte{0x20, labels[parts[1]]}...)
		case "jeqz":
			bc = append(bc, []byte{0x21, labels[parts[1]]}...)

		case "halt":
			bc = append(bc, 0xff)

		case "label":
			// Do nothing

		default:
			return nil, fmt.Errorf("Invalid operation: %v", parts[0])
		}
	}
	return bc, nil
}
