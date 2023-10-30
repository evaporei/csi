package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/evaporei/interpreter/scanner"
	e "github.com/evaporei/interpreter/error"
)

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

    if e.HadError {
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
