## Overview

The goal of this exercise is to convert a simple Go function into bytecode, and then execute the bytecode in a virtual machine.

## Simplifying Assumptions

Solving the problem in full generality (i.e. supporting an arbitrary Go function) is well outside the scope of this exercise. Instead, we recommend making the following simplifying assumptions:

- The input will always be a file with a package declaration followed by a single function
- The single function will always have the prototype `func f(x, y byte) byte`
- Every value will have the type `byte`
- There are no function calls

## Pipeline

The source code will pass through the following steps:

- `parse` uses standard library functions to convert raw source code into an abstract syntax tree
- `compile`, **which you are responsible for implementing**, converts the abstract syntax tree into assembly code
- `assemble` converts assembly code into bytecode
- `runVM` executes the bytecode with the given "input data" and returns the result

## VM Details

There are a few key differences between this VM and the one you implemented for the first session of _Introduction to Computer Systems_.

**Input / Output**

You can assume that as before, the input parameters will always be placed at memory addresses `1` and `2`. In other words, the function `func f(x, y byte) byte` should read `x` and `y` from memory addresses `1` and `2`.

Once the function is done, the return value should be stored in memory address `0`.

**Stack Machine**

The biggest difference is that this one is a "stack machine" rather than a "register machine". In a register machine, arithmetic instructions might look like `add r1 r2`. In a stack machine, arithmetic instructions don't take any arguments; instead, they get their arguments from the stack.

For example, if the stack is `[1 5 2]`, then the `sub` instruction will:

- Pop two values `5` and `2` from the stack
- Perform the subtraction to get `3` (note the ordering)
- Push the result back onto the stack, which will then be `[1 3]`

In addition to arithmetic instructions not taking any arguments, there are also three new instructions involving the stack:

- `push <addr>` will push the contents at memory location `addr` onto the stack
- `pushi <val>` will push the literal value `val` onto the stack
- `pop <addr>` will pop a value from the stack and save it at memory location `addr`

**Comparison Instructions**

There are several new comparison instructions such as `eq`, `lt`. These instructions will pop two values from the stack, perform a comparison, and then push either `0` or `1` back onto the stack, depending on the result.

**Jumps / Labels**

The new `jeqz <addr>` instruction (jump if equal to zero) will:
- Pop a value from the stack (the value will be removed from the stack)
- Check if the value is equal to `0`
- If so, the VM will jump to `addr`; if not, the VM will continue executing the next instruction in order

At the bytecode level, both `Jump` and `Jeqz` absolute (rather than relative) offsets.

At the assembly level, `jump` and `jeqz` take labels (rather than raw addresses), which should make it easier to generate assembly code with jumps. For example:

```
label loop_start
...
jump loop_start
```

## Suggestions

The full version of this problem is VERY difficult. We recommend that you start with a highly restricted version of the problem, and only gradually relax the restrictions. For example, you might want to start by handling a trivial function such as:

```go
func f(x, y byte) byte {
    return 5
}
```

The corresponding assembly might look something like this:

```go
pushi 5
pop 0
halt
```

The first instruction pushes the literal value `5` onto the stack, and the second instruction pops the `5` from the top of the stack and stores it at memory location `0` (the expected location for the return value, after the program halts).

Once you have something working, you can start introducing more functionality (e.g. loading parameters from memory locations `1` and `2`, more complicated expressions, multiple statements, if / else, local variables, for loops) to pass more of the test cases.

## Stretch Goals

Here are a few stretch goals to consider:

**[Constant Folding](https://en.wikipedia.org/wiki/Constant_folding)**

Consider a function such as:

```go
func f(x, y byte) byte {
    return 2 + 3 * 4
}
```

Can you add an optimization step that does the precomputation of subexpressions involving only constants, e.g. `2 + 3 * 4`, so that the output of `compile` looks something like this?

```
pushi 14
pop 0
halt
```

**Recursive Function Calls**

Can you add support for recursive function calls? For example, consider the following version of a Fibonacci function (that ignores the `y` input parameter):

```go
func f(x, y byte) byte {
    if x < 2 {
        return x
    }
    return f(x - 1, 0) + f(x - 2, 0)
}
```

This stretch goal is very difficult! Among other changes, you will likely need to:

- Change the memory layout of the VM so that you have enough space for "stack frames"
- Add more instructions to the VM to support calling and returning (or at least indirect jumps)

**[Tail Call Optimization](https://en.wikipedia.org/wiki/Tail_call)**

Consider the following version of a factorial function, in which we calculate the factorial of `x` by calling `f(x, 1)`:

```go
func f(x, y byte) byte {
    if x == 0 {
        return y
    } else {
        return f(x - 1, y * x)
    }
}
```

There's a recursive function call in the `else` branch, but we don't need to do anything else with the result of the recursive function call. We can just pass it along, unchanged, as the top level return value.

In this situation we can avoid the overhead of a function call by transforming the recursive function call into a loop: instead of creating a new stack frame for the recursive call, we can reuse the current stack frame (since we don't need it anymore) by updating the values of `x` and `y` and then jumping back to the start of the function.

For example, if we express this transformation in Go, we might end up with something like this:

```go
func f(x, y byte) byte {
    for x != 0 {
        x, y = x - 1, y * x
    }
    return y
}
```

Can you update your compiler so that it performs this "tail call optimization" whenever it encounters a `return` statement with just a single recursive call?