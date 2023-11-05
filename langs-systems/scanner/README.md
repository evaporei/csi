# scanner

To run you can either pass a file, eg: `example`:

```bash
$ go run main.go example
TERM hello %!s(<nil>)
AND AND %!s(<nil>)
TERM world %!s(<nil>)
OR OR %!s(<nil>)
TERM alice %!s(<nil>)
AND AND %!s(<nil>)
NOT NOT %!s(<nil>)
TERM bob %!s(<nil>)
EOF  %!s(<nil>)
```

Or use it like a REPL:

```bash
$ go run main.go
> hello AND world OR alice AND NOT bob
TERM hello %!s(<nil>)
AND AND %!s(<nil>)
TERM world %!s(<nil>)
OR OR %!s(<nil>)
TERM alice %!s(<nil>)
AND AND %!s(<nil>)
NOT NOT %!s(<nil>)
TERM bob %!s(<nil>)
EOF  %!s(<nil>)
>
```
