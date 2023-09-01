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

Do the same for `average` (there's still a division by 100 for the `cents`):

```bash
BenchmarkMetrics/Average_payment-8     	    1242	    959322 ns/op
BenchmarkMetrics/Payment_stddev-8      	     528	   2271891 ns/op
```

Remove divisions by `100` in `cents`. No change in performance probably because the compiler was already optimizing this.

```bash
BenchmarkMetrics/Average_payment-8     	    1242	    964086 ns/op
BenchmarkMetrics/Payment_stddev-8      	     535	   2240803 ns/op
```

Removing most divisions and optimizing standard deviation. This gets to the provided solution performance (look into solution folder).

```bash
$ go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	   38312	     31443 ns/op
BenchmarkMetrics/Average_payment-8     	    3556	    342336 ns/op
BenchmarkMetrics/Payment_stddev-8      	     956	   1252118 ns/op
PASS
ok  	command-line-arguments	4.883s
```

Loop unrolling, barely any difference:

```bash
go test -bench=. *.go
goos: darwin
goarch: arm64
BenchmarkMetrics/Average_age-8         	   38217	     31376 ns/op
BenchmarkMetrics/Average_payment-8     	    3805	    313599 ns/op
BenchmarkMetrics/Payment_stddev-8      	     957	   1254484 ns/op
PASS
ok  	command-line-arguments	4.800s
```
