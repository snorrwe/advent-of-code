import sys
sys.path.append('../')
from file_input import GetLinesFromFile
import re

def SortAlphabetically(letters):
	return ''.join(sorted(letters))

def CountLetters(name):
	result = dict()
	for letter in name:
		if letter != "-":
			if letter in result:
				result[letter] += 1
			else:
				result[letter] = 1
	return result
	
def ParseLineToDoorInfo(line):
	name = re.sub(r"-[0-9]+\[.*\]", "", line).replace("\n", "")
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
	for line in input:
		doorInfo = ParseLineToDoorInfo(line)
		letterCount = CountLetters(doorInfo["name"])
		sortedLetters = list(reversed(sorted(letterCount.items(), key=lambda x: x[1])))
		sortedLetters = SortTies(sortedLetters)[:5]
		if(sortedLetters == doorInfo["checkSum"]):
			decryptName = ""
			for letter in doorInfo["name"]:
				if letter != "-":
					rotatedLetter = chr(ord('a') + ((ord(letter) - ord('a') + int(doorInfo["sectorId"])) % 26))
					decryptName += rotatedLetter
				else:
					decryptName += letter
			if "north" in decryptName:
				print decryptName + ": " + doorInfo["sectorId"]

if __name__ == '__main__':
	input = GetLinesFromFile("day4_input.txt")
	Solve(input)
