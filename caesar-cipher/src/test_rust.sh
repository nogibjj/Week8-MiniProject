#!/bin/bash

# Rust script runner

N=1000
total=0

# Ensure we're using the optimized build for fair comparison
cargo build --release

for (( i=0; i<N; i++ )); do
    # Capture time before execution
    start=$(python -c 'import time; print(time.time())')
    
    # Run your script. Replace with your actual Rust program's path if different.
    ./target/release/caesar-cipher "the quick brown fox jumps over the lazy dog" 3
    
    # Capture time after execution
    end=$(python -c 'import time; print(time.time())')
    
    # Calculate the difference in the captured times using Python
    runtime=$(python -c "print($end - $start)")
    
    # Accumulate the total time
    total=$(python -c "print($total + $runtime)")
done

echo "Total time taken for $N runs (Rust): $total seconds"
