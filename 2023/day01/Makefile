all: part1 part2

part1: part1.o
	ld part1.o -o part1

part1.o: part1.S
	nasm -f elf64 -o part1.o part1.S

part2: part2.o
	ld part2.o -o part2

part2.o: part2.S
	nasm -f elf64 -o part2.o part2.S

clean:
	rm -f part1 part2 *.o
