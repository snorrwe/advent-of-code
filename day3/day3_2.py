import sys
sys.path.append('../')
from file_input import GetLinesFromFile
import re

def IsTriangle(a, b, c):
	return a + b > c and b + c > a and a + c > b

def Solve(input):
	count = 0
	lines = []
	for line in input:
		sides = re.findall("[0-9]+", line)
		lines.append(sides)
		if(len(lines) == 3):
			triangle1 = []
			triangle2 = []
			triangle3 = []
			for line in lines:
				triangle1.append(line[0])
				triangle2.append(line[1])
				triangle3.append(line[2])
			if(IsTriangle(triangle1[0], triangle1[1], triangle1[2])):
				count += 1
			if(IsTriangle(triangle2[0], triangle2[1], triangle2[2])):
				count += 1
			if(IsTriangle(triangle3[0], triangle3[1], triangle3[2])):
				count += 1
			lines = []
	return count

if __name__ == '__main__':
	input = GetLinesFromFile("day3_input.txt")
	print Solve(input)