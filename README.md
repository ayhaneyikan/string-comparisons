# String Comparisons

A small tool created to assist a peer with comparing a large list of strings stored in a csv.
The goal was to make a comparison between each string in the list using a handful of different metrics.
I created this repository in the interest of attempting to increase the efficiency of these comparisons.

No benchmarking was done as this was thrown together quickly to be used, however, it was a fun exercise 
in spawning threads and managing some shared state in Rust.
There are certainly many ways this could be optimized far better which I did not explore.

## Executing the Comparisons

The current state of the `main()` function is set up to run the threaded version of the comparison.
Each thread will perform comparisons on a subset of the string list and output results to a csv.
These are stored in `results/` and are labeled with the thread's index.
Note that even the threaded version takes a couple hours to run (depending on hardware).
