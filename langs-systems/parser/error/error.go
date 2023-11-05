package error

import (
	"fmt"
	"os"
)

var HadError bool = false

func Fail(line int, msg string) {
    report(line, "", msg)
}

func report(line int, where, msg string) {
    fmt.Fprintf(os.Stderr, "[line %d] Error %s: %s", line, where, msg)
    HadError = true
}
