package main

import (
	"bytes"
	// "fmt"
	"log"
	"os"
	"strings"
	"sort"

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

    fns := make([]dst.Decl, 0)
    idxs := make([]int, 0)

    for i, decl := range f.Decls {
        if _, ok := decl.(*dst.FuncDecl); ok {
            fns = append(fns, dst.Clone(decl).(dst.Decl))
            idxs = append(idxs, i)
        }
    }

    c := 0
    for _, idx := range idxs {
        // https://stackoverflow.com/questions/37334119/how-to-delete-an-element-from-a-slice-in-golang
        f.Decls = append(f.Decls[:idx - c], f.Decls[idx+1 - c:]...)
        c += 1
    }

    sort.SliceStable(fns, func(i, j int) bool {
        fn1 := fns[i].(*dst.FuncDecl)
        fn2 := fns[j].(*dst.FuncDecl)
        res := strings.Compare(fn1.Name.Name, fn2.Name.Name)
        if res == 1 {
            return false
        }
        if res == -1 {
            return true
        }
        return false
    })

    for _, fn := range fns {
        f.Decls = append(f.Decls, fn)
    }

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
