package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/evaporei/interpreter/scanner"
)

var hadError bool = false

func fail(line int, msg string) {
    report(line, "", msg)
}

func report(line int, where, msg string) {
    fmt.Fprintf(os.Stderr, "[line %d] Error %s: %s", line, where, msg)
    hadError = true
}

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func run(src string) {
    s := scanner.New(src)
    tokens := s.ScanTokens()

    for _, token := range tokens {
        fmt.Println(token)
    }
}

func runFile(file string) {
    bytes, err := os.ReadFile(file)
    check(err)

    contents := string(bytes)
    run(contents)

    if hadError {
        os.Exit(65)
    }
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
