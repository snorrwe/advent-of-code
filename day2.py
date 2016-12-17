from file_input import GetLinesFromFile

def NextNumber(currentNumber, step):
	if(step == "U" and currentNumber > 3):
			return currentNumber - 3
	if(step == "D" and currentNumber < 7):
			return currentNumber + 3
	if(step == "L" and currentNumber % 3 != 1):
			return currentNumber - 1
	if(step == "R" and currentNumber % 3 != 0):
			return currentNumber + 1
	return currentNumber

def NextNumberByInstructions(startNumber, instruction):
	result = startNumber
	for step in instruction:
		result = NextNumber(result, step)
	return result

def Solve(instructions):
	currentNumber = 5
	result = []
	for instruction in instructions:
		currentNumber = NextNumberByInstructions(currentNumber, instruction)
		result.append(currentNumber)
	return result

if __name__ == '__main__':
	input = GetLinesFromFile("day2_input.txt")
	numbers = Solve(input)
	solution = ""
	for num in numbers:
		solution += str(num)
	print solution
