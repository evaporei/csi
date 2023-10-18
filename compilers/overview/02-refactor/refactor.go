package main

import (
	"bytes"
	// "fmt"
	"log"
	"os"
    // "strings"

	"github.com/dave/dst"
	"github.com/dave/dst/decorator"
	// "github.com/dave/dst/dstutil"
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

// Moves all top-level functions to the end, sorted in alphabetical order.
// The "source file" is given as a string (rather than e.g. a filename).
func SortFunctions(src string) (string, error) {
    f, err := decorator.Parse(src)
    if err != nil {
        return "", err
    }

    fns := make([]*dst.Decl, 0)
    idxs := make([]int, 0)

    for i, decl := range f.Decls {
        if _, ok := decl.(*dst.FuncDecl); ok {
            fns = append(fns, &decl)
            idxs = append(idxs, i)
        }
    }

    for _, idx := range idxs {
        // https://stackoverflow.com/questions/37334119/how-to-delete-an-element-from-a-slice-in-golang
        f.Decls = append(f.Decls[:idx], f.Decls[idx+1:]...)
    }

    // for _, fn := range fns {
    //     f.Decls = append(f.Decls, fn)
    // }

    out := bytes.NewBuffer(nil)
    err = decorator.Fprint(out, f)
    if err != nil {
        return "", err
    }

	return out.String(), nil
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
