# champernowne100b
100 billion digits of champernowne's constant\
[WolfRam Definition](https://mathworld.wolfram.com/ChampernowneConstant.html)

### Option 1: champernowne.rs
initialize_champernowne(range: u32) makes a champernowne that goes into src/bin as an artifact.
read_champernowne(filename: &str, n:usize) reads champernowne from a file and prints the nth digit.

#### Note: not limited by 32 bit integer cap

### Option 2 - r2champernowne.rs
champ(mut i: u32) prints out the ith digit of champernowne. Quick and does not create permanent artefacts

#### Note: limited by the u32 power (so around i=~700m)

### Option 3 - r2champernowne linux binary
asks you for the nth digit and generates. Used for quick lookups. Little customization.

## NOTE: This string of bytes is not in decimal form (0.n), instead taking on the integral form of (n).
