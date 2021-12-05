#!/usr/bin/bash

c++ day5_part2.cpp -o d5.exe -std=c++1z -stdlib=libc++ && echo "D5 normal" && d5.exe
c++ day5_part2.cpp -o d5.exe -std=c++1z -stdlib=libc++ -fconstexpr-steps=500000000 && echo "D5 constexpr" && d5.exe
rm d5.exe
