# [Advent of Code 2017](https://adventofcode.com/)

## C++ solutions

C++ solutions aim to use only `constexpr`, meaning that the goal is for the compiler to opimize the whole thing away, leaving only the solution.

All solutions require __C++14__ and some require __C++17__ features.

__/core/__ holds reusable `constexpr` methods.

My `constexpr` "containers" all use the same basic idea: they are only methods working on `std::array`-s.
Methods that mutate containers will return a new array, since constexpr functions cannot take references as parameters.

Common compiler flags used: `-O3 -std=c++1z -fconstexpr-steps=90000000`
