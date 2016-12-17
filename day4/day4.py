import sys
sys.path.append('../')
from file_input import GetLinesFromFile
import re
import operator

def SortAlphabetically(letters):
	return ''.join(sorted(letters))

def CountLetters(name):
	result = dict()
	for letter in name:
		if letter in result:
			result[letter] += 1
		else:
			result[letter] = 1
	return result
	
def ParseLineToDoorInfo(line):
	name = re.sub(r"-[0-9]+\[.*\]", "", line).replace("-", "").replace("\n", "")
	sectorId = re.sub(r"^([a-z]+-)+", "", line)
	sectorId = re.sub(r"\[.*\]", "", sectorId).replace("\n", "")
	checkSum = re.sub(r"^.*\[", "", line).replace("]", "").replace("\n", "")

	return {"name": name, "sectorId": sectorId, "checkSum": checkSum}

def SortTies(sortedLetters):
	sortedLetters.append(('extra', -1))
	result = ""
	lastCnt = ""
	todo = []
	for pair in sortedLetters:
		if lastCnt != pair[1]:
			result += SortAlphabetically(todo)
			todo = []
		todo.append(pair[0])
		lastCnt = pair[1]
	return result

def Solve(input):
	sum = 0
	for line in input:
		doorInfo = ParseLineToDoorInfo(line)
		letterCount = CountLetters(doorInfo["name"])
		sortedLetters = list(reversed(sorted(letterCount.items(), key=lambda x: x[1])))
		sortedLetters = SortTies(sortedLetters)[:5]
		if(sortedLetters == doorInfo["checkSum"]):
			sum += int(doorInfo["sectorId"])
	return sum

if __name__ == '__main__':
	#input = GetLinesFromFile("day4_input2.txt")
	input = GetLinesFromFile("day4_input.txt")
	print Solve(input)
