#!/bin/sh

nitrogen --restore &
picom -b &
dwmblocks &

while true; do
	penrose 2> ./.penrose.log
done
