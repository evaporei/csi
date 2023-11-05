# Author: Bryan Lynch
CC=gcc
CFLAGS=-Wall
COMPILE_COMMAND=$(CC) $(CFLAGS) -o $@ $^

iterative: echoserveri.c csapp.c echo.c
	$(COMPILE_COMMAND)

process: echoserverp.c csapp.c echo.c
	$(COMPILE_COMMAND)

select: select.c csapp.c echo.c
	$(COMPILE_COMMAND)

concurrent_io: echoservers.c csapp.c echo.c
	$(COMPILE_COMMAND)

threads: echoservert.c csapp.c echo.c
	$(COMPILE_COMMAND)

prethreaded: echoservert_pre.c csapp.c echo_cnt.c sbuf.c
	$(COMPILE_COMMAND)

badcount: badcnt.c csapp.c
	$(COMPILE_COMMAND)

race: race.c csapp.c
	$(COMPILE_COMMAND)

.PHONY : clean
clean :
	rm -f iterative process select concurrent_io threads prethreaded badcount race
