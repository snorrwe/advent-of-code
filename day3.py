from file_input import GetLinesFromFile
import re

def IsTriangle(a, b, c):
	return a + b > c and b + c > a and a + c > b

def Solve(input):
	count = 0
	for line in input:
		sides = re.findall("[0-9]+", line)
		if(IsTriangle(int(sides[0]), int(sides[1]), int(sides[2]))):
			count += 1
	return count

if __name__ == '__main__':
	input = GetLinesFromFile("day3_input.txt")
	print Solve(input)