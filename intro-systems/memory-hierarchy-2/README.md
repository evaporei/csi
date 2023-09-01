# metrics

No change:

```
$ go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	    1002	   1180386 ns/op
BenchmarkMetrics/Average_payment-8     	      85	  13663136 ns/op
BenchmarkMetrics/Payment_stddev-8      	      43	  27659781 ns/op
PASS
ok  	command-line-arguments	4.887s
```
