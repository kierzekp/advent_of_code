# Advent of Code
My solutions for [Advent of Code](https://adventofcode.com/) puzzles, sorted by year. 
As per the project's FAQ, no inputs nor any other content from the adventofcode.com website is contained within this repository.

## 2015
Solutions for puzzles from [Advent of Code 2015](https://adventofcode.com/2015) are presented in [Rust](https://www.rust-lang.org/).
#### Notes:
- Solution for 04 does not include the implementation of the actual md5 hashing function. Faced with that particular task, I had basically three possible options to consider: writing my own implementation of MD5 algorithm based on the reference, importing an [existing crate which implements MD5 hashing](https://crates.io/crates/md5), or using existing code from another source. I considered writing my own implementation, however to be honest, I do not consider my level of Rust knowledge to be anywhere near allowing me to write complex bitwise operations. Importing a crate I ended up not doing since I want to keep the implementations as light as possible and do not want to use Cargo. Therefore I ended up using the existing implementation from Rosetta Stone for that particular task. Due to licensing issues, the module including that function is not included in this repository. The function verbatim as used can be found [here](https://rosettacode.org/wiki/MD5/Implementation#Rust).
