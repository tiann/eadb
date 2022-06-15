#!/bin/bash

kvers=$(uname -r)

MAJOR=$(echo $kvers | awk -F. '{ print $1 }')
MINOR=$(echo $kvers | awk -F. '{ print $2 }')
SUBVR=$(echo $kvers | awk -F. '{ print $3 }' | awk -F- '{ print $1 }' | sed 's/[^0-9]*//g')

maj_num=$(($MAJOR * 65536))
min_num=$(($MINOR * 256))

echo $(($maj_num + $min_num + $SUBVR))
