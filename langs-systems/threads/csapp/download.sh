#!/bin/bash
# Author: Bryan Lynch
BASEURL="http://csapp.cs.cmu.edu/3e/ics3/code/"
URLS="include/csapp.h src/csapp.c netp/echo.c netp/echoserveri.c conc/echoserverp.c conc/select.c conc/echoservers.c conc/echoservert.c conc/echoservert_pre.c conc/sbuf.c conc/sbuf.h conc/echo_cnt.c conc/badcnt.c conc/race.c"
for url in $URLS; 
do
	filename=$(echo "$url" | cut -d / -f2)

	if [ ! -f "$filename" ]; then
		curl -o "$filename" "$BASEURL/$url"
	fi
done
