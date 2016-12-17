import sys
sys.path.append('../')
from file_input import GetLinesFromFile

class KeyPadSolver(object):
	def __init__(self, keyPad):
		self.keyPad = keyPad

	def NextCoordinates(self, currentCoordinates, step):
		x = currentCoordinates[1]
		y = currentCoordinates[0]
		if(step == "U" and y > 0):
			if(len(self.keyPad[y]) < len(self.keyPad[y-1])):
				y -= 1
				x += 1
			elif(x > 0 and x < len(self.keyPad[y])- 1):
				y -= 1
				x -= 1
		if(step == "D" and y < len(self.keyPad) - 1):
			if(len(self.keyPad[y]) < len(self.keyPad[y+1])):
				y += 1
				x += 1
			elif(x > 0 and x < len(self.keyPad[y])- 1):
				y += 1
				x -= 1
		if(step == "L"):
			if(x > 0):
				x -= 1
		if(step == "R"):
			if(x < len(self.keyPad[y]) - 1):
				x += 1
		return [y, x]

	def NextCoordinatesByInstructions(self, currentCoordinates, instruction):
		for step in instruction:
			currentCoordinates = self.NextCoordinates(currentCoordinates, step)
		return currentCoordinates

	def Solve(self, startPoint, instructions):
		currentCoordinates = startPoint
		result = ""
		for instruction in instructions:
			currentCoordinates = self.NextCoordinatesByInstructions(currentCoordinates, instruction)
			result += self.keyPad[currentCoordinates[0]][currentCoordinates[1]]
		return result
		
if __name__ == '__main__':
	input = GetLinesFromFile("day2_input.txt")
	startPoint = [2,0]
	keyPad = [["1"],["2","3","4"],["5", "6", "7", "8", "9"], ["A", "B", "C"], ["D"]]
	solver = KeyPadSolver(keyPad)
	print solver.Solve(startPoint, input)
