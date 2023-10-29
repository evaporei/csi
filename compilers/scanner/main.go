package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func run(src string) {
    fmt.Println(src)
}

func runFile(file string) {
    bytes, err := os.ReadFile(file)
    check(err)

    contents := string(bytes)
    run(contents)
}

func runPrompt() {
    reader := bufio.NewScanner(os.Stdin)
    fmt.Print("> ")
    for reader.Scan() {
        text := strings.TrimSpace(reader.Text())
        run(text)
        fmt.Print("> ")
    }
    fmt.Println()
}

func main() {
    if len(os.Args) > 2 {
        fmt.Println("Usage: scanner [file]")
        os.Exit(64)
    } else if len(os.Args) == 2 {
        runFile(os.Args[1])
    } else {
        runPrompt()
    }
}
