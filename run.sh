#!/bin/bash

rustc bbp.rs
if test $? -ne 0
then
	exit 1
fi

#default data
SAMPLE_DATA1="(10,200) (15,100) (35,500) (50,30)"
#data with multiple sell events per day
SAMPLE_DATA2="(10,200) (10,190) (15,100) (15,99) (35,500) (35,400) (35,300) (50,30)"
#case with eating stale bread for 9 days
SAMPLE_DATA3="(19,250) (40,200) (50,40)"
#case with no input
SAMPLE_DATA4=""
#case with single provider with initial stock available
SAMPLE_DATA5="(5,500)"
#case with single provider after initial stock depleted
SAMPLE_DATA6="(15,200)"
#case with two providers
SAMPLE_DATA7="(1,500) (40,600)"
#invalid data
SAMPLE_DATA8="(0,0)"

SAMPLE_DATA=$SAMPLE_DATA1
NUM_DAYS=60

echo Running experiment with:
echo $NUM_DAYS days
echo $SAMPLE_DATA
echo -e "\n\nResults:"

RUST_BACKTRACE=full ./bbp $NUM_DAYS "$SAMPLE_DATA"
