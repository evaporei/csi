package main

import (
	"bytes"
	"log"
	"os"
	"sort"

	"github.com/dave/dst"
	"github.com/dave/dst/decorator"
)

const src string = `package foo

import (
	"fmt"
	"time"
)

func baz() {
	fmt.Println("Hello, world!")
}

type A int

const b = "testing"

func bar() {
	fmt.Println(time.Now())
}`

type ByName []*dst.FuncDecl

func (a ByName) Len() int           { return len(a) }
func (a ByName) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByName) Less(i, j int) bool { return a[i].Name.Name < a[j].Name.Name }

func sortFunctionsInternal(f *dst.File) {
	var funcs []*dst.FuncDecl
	var nonFuncs []dst.Decl
	for _, decl := range f.Decls {
		if fd, ok := decl.(*dst.FuncDecl); ok {
			funcs = append(funcs, fd)
		} else {
			nonFuncs = append(nonFuncs, decl)
		}
	}
	sort.Sort(ByName(funcs))
	var decls []dst.Decl
	decls = append(decls, nonFuncs...)
	for _, fd := range funcs {
		decls = append(decls, fd)
	}
	f.Decls = decls
}

// Moves all top-level functions to the end, sorted in alphabetical order.
// The "source file" is given as a string (rather than e.g. a filename).
func SortFunctions(src string) (string, error) {
	f, err := decorator.Parse(src)
	if err != nil {
		return "", err
	}

	sortFunctionsInternal(f)

	// Convert AST back to source
	var buf bytes.Buffer
	err = decorator.Fprint(&buf, f)
	if err != nil {
		return "", err
	}

	return buf.String(), nil
}

func main() {
	f, err := decorator.Parse(src)
	if err != nil {
		log.Fatal(err)
	}

	// Print AST
	err = dst.Fprint(os.Stdout, f, nil)
	if err != nil {
		log.Fatal(err)
	}

	// Convert AST back to source
	err = decorator.Print(f)
	if err != nil {
		log.Fatal(err)
	}
}
