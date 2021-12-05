package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
)

func fuel(mod int) int {
	return mod/3 - 2
}

func part1(input io.Reader) (int, error) {
	sum := 0
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		mod, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return sum, err
		}
		sum += fuel(mod)
	}
	return sum, nil
}

func part2(input io.Reader) (int, error) {
	sum := 0
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		mod, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return sum, err
		}
		mod = fuel(mod)
		for mod > 0 {
			sum += mod
			mod = fuel(mod)
		}
	}
	return sum, nil
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	res, err := part1(file)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d\n", res)

	file, err = os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	res, err = part2(file)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d\n", res)
}
