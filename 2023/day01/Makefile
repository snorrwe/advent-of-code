part1: part1.o
	ld part1.o -o part1

part1.o: part1.S
	nasm -f elf64 -o part1.o part1.S

clean:
	rm -f part1 *.o
