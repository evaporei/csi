## pagecount

### Opt 0 & Division (/)

```
$ gcc -O0 pagecount.c && ./a.out
0.01s to run 10000000 tests (0.64ns per test)
```

```
$ perf stat ./a.out
Performance counter stats for './a.out':

             56.63 msec task-clock:u                     #    0.983 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
                61      page-faults:u                    #    1.077 K/sec                     
         215457754      cpu_core/cycles/u                #    3.805 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
        1000134700      cpu_core/instructions/u          #   17.662 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
          40029452      cpu_core/branches/u              #  706.901 M/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
              2466      cpu_core/branch-misses/u         #   43.548 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
        1292746524      cpu_core/slots:u/                #   22.829 G/sec                     
        1069684378      cpu_core/topdown-retiring/u      #     82.7% Retiring                 
            241871      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
         217992551      cpu_core/topdown-fe-bound/u      #     16.9% Frontend Bound           
           5069594      cpu_core/topdown-be-bound/u      #      0.4% Backend Bound            
          70974318      cpu_core/topdown-heavy-ops/u     #      5.5% Heavy Operations          #     77.2% Light Operations         
            241871      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
          50695942      cpu_core/topdown-fetch-lat/u     #      3.9% Fetch Latency             #     12.9% Fetch Bandwidth          
             80623      cpu_core/topdown-mem-bound/u     #      0.0% Memory Bound              #      0.4% Core Bound               

       0.057608809 seconds time elapsed

       0.054232000 seconds user
       0.003379000 seconds sys

```

### Opt 0 & Bit shift (<<)

```
$ gcc -O0 pagecount.c ; and ./a.out
0.01s to run 10000000 tests (0.73ns per test)
```

```
$ perf stat ./a.out
 Performance counter stats for './a.out':

             48.79 msec task-clock:u                     #    0.973 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
                61      page-faults:u                    #    1.250 K/sec                     
         189409587      cpu_core/cycles/u                #    3.882 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
        1020134556      cpu_core/instructions/u          #   20.907 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
          40029417      cpu_core/branches/u              #  820.384 M/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
              2482      cpu_core/branch-misses/u         #   50.867 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
        1136457522      cpu_core/slots:u/                #   23.291 G/sec                     
        1060693687      cpu_core/topdown-retiring/u      #     93.3% Retiring                 
            256805      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
          66850442      cpu_core/topdown-fe-bound/u      #      5.9% Frontend Bound           
           8913392      cpu_core/topdown-be-bound/u      #      0.8% Backend Bound            
          40110265      cpu_core/topdown-heavy-ops/u     #      3.5% Heavy Operations          #     89.8% Light Operations         
            256805      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
           8913392      cpu_core/topdown-fetch-lat/u     #      0.8% Fetch Latency             #      5.1% Fetch Bandwidth          
                 0      cpu_core/topdown-mem-bound/u     #      0.0% Memory Bound              #      0.8% Core Bound               

       0.050146025 seconds time elapsed

       0.046714000 seconds user
       0.003306000 seconds sys
```

### Opt 2 & Division (/)

```
$ gcc -O2 pagecount.c ; and ./a.out
0.02s to run 10000000 tests (1.64ns per test)
```

```
$ perf stat ./a.out
 Performance counter stats for './a.out':

             32.84 msec task-clock:u                     #    0.949 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
                61      page-faults:u                    #    1.857 K/sec                     
         127796253      cpu_core/cycles/u                #    3.891 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
         320134368      cpu_core/instructions/u          #    9.747 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
          20029370      cpu_core/branches/u              #  609.824 M/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
              2388      cpu_core/branch-misses/u         #   72.706 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
         766777518      cpu_core/slots:u/                #   23.346 G/sec                     
         339787684      cpu_core/topdown-retiring/u      #     44.5% Retiring                 
              9555      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
          51118501      cpu_core/topdown-fe-bound/u      #      6.7% Frontend Bound           
         372864361      cpu_core/topdown-be-bound/u      #     48.8% Backend Bound            
          39090618      cpu_core/topdown-heavy-ops/u     #      5.1% Heavy Operations          #     39.4% Light Operations         
              8600      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
             20306      cpu_core/topdown-fetch-lat/u     #      0.0% Fetch Latency             #      6.7% Fetch Bandwidth          
          87202149      cpu_core/topdown-mem-bound/u     #     11.4% Memory Bound              #     37.4% Core Bound               

       0.034608481 seconds time elapsed

       0.031066000 seconds user
       0.003415000 seconds sys
```

### Opt 2 & Bit shift (<<)

```
$ gcc -O2 pagecount.c ; and ./a.out
0.00s to run 10000000 tests (0.06ns per test)
```

```
$ perf stat ./a.out
 Performance counter stats for './a.out':

             17.92 msec task-clock:u                     #    0.972 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
                61      page-faults:u                    #    3.404 K/sec                     
          61513373      cpu_core/cycles/u                #    3.432 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
         310134668      cpu_core/instructions/u          #   17.305 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
          20029441      cpu_core/branches/u              #    1.118 G/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
              2444      cpu_core/branch-misses/u         #  136.371 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
         369052428      cpu_core/slots:u/                #   20.592 G/sec                     
         299583735      cpu_core/topdown-retiring/u      #     81.2% Retiring                 
                 0      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
           4341793      cpu_core/topdown-fe-bound/u      #      1.2% Frontend Bound           
          65126899      cpu_core/topdown-be-bound/u      #     17.6% Backend Bound            
          10130850      cpu_core/topdown-heavy-ops/u     #      2.7% Heavy Operations          #     78.4% Light Operations         
                 0      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
           1447264      cpu_core/topdown-fetch-lat/u     #      0.4% Fetch Latency             #      0.8% Fetch Bandwidth          
                 0      cpu_core/topdown-mem-bound/u     #      0.0% Memory Bound              #     17.6% Core Bound               

       0.018436405 seconds time elapsed

       0.018435000 seconds user
       0.000000000 seconds sys
```

## dotproduct

The measuraments using the `time.h` API show performance improvements with the optimizations, but using `perf stat` gave worse data compared to the ones registered in `solution/dotproduct.c` comments, the fully optimized version had worse `Retiring`, `Backend Bound` and `Memory Bound` than the least optimized version. Removing the `get_vec_element` call was one of the best optimizations. I've created a `bench.c` to remove the testing framework and only run dotproduct stuff.

Using the least optimized dotproduct implementation:
```
$ gcc -O2 bench.c vec.c dotproduct.c
$ perf stat ./a.out
0.16s to take product of length 100000000 vectors (1.55 ns per element)

 Performance counter stats for './a.out':

            441.38 msec task-clock:u                     #    0.998 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
              1817      page-faults:u                    #    4.117 K/sec                     
        1236123531      cpu_core/cycles/u                #    2.801 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
        6800140661      cpu_core/instructions/u          #   15.407 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
        2000032178      cpu_core/branches/u              #    4.531 G/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
             10791      cpu_core/branch-misses/u         #   24.448 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
        7414936686      cpu_core/slots:u/                #   16.799 G/sec                     
        6280887545      cpu_core/topdown-retiring/u      #     85.0% Retiring                 
                 0      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
                 0      cpu_core/topdown-fe-bound/u      #      0.0% Frontend Bound           
        1104970957      cpu_core/topdown-be-bound/u      #     15.0% Backend Bound            
         494329112      cpu_core/topdown-heavy-ops/u     #      6.7% Heavy Operations          #     78.3% Light Operations         
                 0      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
                 0      cpu_core/topdown-fetch-lat/u     #      0.0% Fetch Latency             #      0.0% Fetch Bandwidth          
        1046814590      cpu_core/topdown-mem-bound/u     #     14.2% Memory Bound              #      0.8% Core Bound               

       0.442298907 seconds time elapsed

       0.262436000 seconds user
       0.179373000 seconds sys

```

Using the most optimized dotproduct implementation:
```
$ gcc -O2 bench.c vec.c dotproduct.c
$ perf stat ./a.out
0.06s to take product of length 100000000 vectors (0.56 ns per element)

 Performance counter stats for './a.out':

            403.53 msec task-clock:u                     #    0.997 CPUs utilized             
                 0      context-switches:u               #    0.000 /sec                      
                 0      cpu-migrations:u                 #    0.000 /sec                      
              1816      page-faults:u                    #    4.500 K/sec                     
        1100559201      cpu_core/cycles/u                #    2.727 G/sec                     
     <not counted>      cpu_atom/cycles/u                                                       (0.00%)
        3400140631      cpu_core/instructions/u          #    8.426 G/sec                     
     <not counted>      cpu_atom/instructions/u                                                 (0.00%)
         925032129      cpu_core/branches/u              #    2.292 G/sec                     
     <not counted>      cpu_atom/branches/u                                                     (0.00%)
              9070      cpu_core/branch-misses/u         #   22.476 K/sec                     
     <not counted>      cpu_atom/branch-misses/u                                                (0.00%)
        6601863078      cpu_core/slots:u/                #   16.360 G/sec                     
        3080869436      cpu_core/topdown-retiring/u      #     46.7% Retiring                 
           1052841      cpu_core/topdown-bad-spec/u      #      0.0% Bad Speculation          
          25889659      cpu_core/topdown-fe-bound/u      #      0.4% Frontend Bound           
        3495103982      cpu_core/topdown-be-bound/u      #     52.9% Backend Bound            
         207117273      cpu_core/topdown-heavy-ops/u     #      3.1% Heavy Operations          #     43.5% Light Operations         
            526420      cpu_core/topdown-br-mispredict/u #      0.0% Branch Mispredict         #      0.0% Machine Clears           
           1579262      cpu_core/topdown-fetch-lat/u     #      0.0% Fetch Latency             #      0.4% Fetch Bandwidth          
        2977310799      cpu_core/topdown-mem-bound/u     #     45.1% Memory Bound              #      7.8% Core Bound               

       0.404560238 seconds time elapsed

       0.257149000 seconds user
       0.146887000 seconds sys
```
