# metrics

No change:

```bash
$ go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	    1002	   1180386 ns/op
BenchmarkMetrics/Average_payment-8     	      85	  13663136 ns/op
BenchmarkMetrics/Payment_stddev-8      	      43	  27659781 ns/op
PASS
ok  	command-line-arguments	4.887s
```

Just fetching a list of ages instead of the whole map (`map[UserId]*User` -> `[]int`).

```bash
BenchmarkMetrics/Average_age-8         	    2397	    500411 ns/op
```

Converting `UserMap` into `UserData` (`[]int` + `[]DollarAmount`).

```bash
$ go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	    2391	    500697 ns/op
BenchmarkMetrics/Average_payment-8     	     230	   5311983 ns/op
BenchmarkMetrics/Payment_stddev-8      	     189	   6327712 ns/op
PASS
ok  	command-line-arguments	5.583s
```

Changing `ages` to be `uint8` instead of `int`. No difference.

```bash
$ go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	    2384	    505523 ns/op
BenchmarkMetrics/Average_payment-8     	     234	   5168928 ns/op
BenchmarkMetrics/Payment_stddev-8      	     189	   6325219 ns/op
PASS
ok  	command-line-arguments	5.582s
```

Do only one division in average `age` instead of one per element.

```bash
BenchmarkMetrics/Average_age-8         	   12769	     94525 ns/op 
```
