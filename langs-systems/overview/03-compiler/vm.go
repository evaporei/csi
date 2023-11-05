package main

import (
	"fmt"
)

const (
	Push  = 0x01
	Pushi = 0x02
	Pop   = 0x03

	Add = 0x10
	Sub = 0x11
	Mul = 0x12
	Div = 0x13

	Eq  = 0x14
	Lt  = 0x15
	Gt  = 0x16
	Neq = 0x17
	Leq = 0x18
	Geq = 0x19

	Jump = 0x20
	Jeqz = 0x21

	Halt = 0xff
)

// Given a 256 byte array of "memory", run the stored program
// to completion, modifying the data in place to reflect the result
//
// The memory format is:
//
// 00 01 02 03 04 05 06 07 08 09 0a 0b 0c ... df e0 e1 ... ff
// __ __ __ __ __ __ __ __ __ __ __ __ __ ... __ __ __ ... __
// ^==DATA===============^ ^==INSTRUCTIONS=====^ ^==STACK===^
//
func execute(memory []byte) error {
	var (
		pc byte = 0x08 // program counter
		sp byte = 0xff // stack pointer (one byte past end)

		// Temporary registers for arithmetic operations
		r1 byte = 0x00
		r2 byte = 0x00
	)

	applyOp := func(op byte) (byte, error) {
		switch op {
		case Add:
			return r1 + r2, nil
		case Sub:
			return r1 - r2, nil
		case Mul:
			return r1 * r2, nil
		case Div:
			return r1 / r2, nil

		case Eq:
			if r1 == r2 {
				return 1, nil
			} else {
				return 0, nil
			}
		case Lt:
			if r1 < r2 {
				return 1, nil
			} else {
				return 0, nil
			}
		case Gt:
			if r1 > r2 {
				return 1, nil
			} else {
				return 0, nil
			}
		case Neq:
			if r1 != r2 {
				return 1, nil
			} else {
				return 0, nil
			}
		case Leq:
			if r1 <= r2 {
				return 1, nil
			} else {
				return 0, nil
			}
		case Geq:
			if r1 >= r2 {
				return 1, nil
			} else {
				return 0, nil
			}

		default:
			return 0, fmt.Errorf("Invalid binary operation: 02%02x", op)
		}
	}

	for {
		op := memory[pc]

		if op == Halt {
			return nil
		}

		// fetch oparg (at most 1)
		arg := memory[pc+1]

		// execute
		switch op {
		case Push:
			// Push the value at memory location arg onto the stack
			memory[sp] = memory[arg]
			sp--
			pc += 2
		case Pushi:
			// Push the immediate value arg onto the stack
			memory[sp] = arg
			sp--
			pc += 2
		case Pop:
			// Pop a value from the stack to the memory location arg
			sp++
			memory[arg] = memory[sp]
			pc += 2

		// Pop two values from the stack, apply the binary operation, and push
		// the result back onto the stack
		case Add:
			fallthrough
		case Sub:
			fallthrough
		case Mul:
			fallthrough
		case Div:
			fallthrough
		case Eq:
			fallthrough
		case Lt:
			fallthrough
		case Gt:
			fallthrough
		case Neq:
			fallthrough
		case Leq:
			fallthrough
		case Geq:
			sp++
			r2 = memory[sp]
			sp++
			r1 = memory[sp]
			var err error
			memory[sp], err = applyOp(op)
			if err != nil {
				return err
			}
			sp--
			pc += 1

		case Jump:
			pc = arg
		case Jeqz:
			sp++
			if memory[sp] == 0 {
				pc = arg
			} else {
				pc += 2
			}

		default:
			return fmt.Errorf("Invalid opcode: 0x%02x", op)
		}
	}
}
