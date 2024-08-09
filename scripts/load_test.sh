#!/bin/bash

# Function to display usage
display_usage() {
    echo "Usage: load_test.sh <actor_1> <actor_2> <iterations>"
    echo "  actor_1     The first actor ID"
    echo "  actor_2     The second actor ID"
    echo "  iterations  The number of iterations to run"
}

# Check if --help is passed in
if [[ "$1" == "--help" ]]; then
    display_usage
    exit 0
fi

# Check if all required arguments are provided
if [ $# -ne 3 ]; then
    echo "Usage: load_test.sh <actor_1> <actor_2> <iterations>"
    exit 1
fi

actor_1=$1
actor_2=$2
iterations=$3

# Perform load testing
for ((j=1; j<=$iterations; j++))
do
    curl -X POST -d "actor_1=$actor_1&actor_2=$actor_2" https://abroy77.co.uk/shortest_path
done

# Wait for all background processes to finish
wait