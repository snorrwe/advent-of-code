def nextDirection(currentDir, turn):
    return {
        "NL": "W",
        "NR": "E",
        "WL": "S",
        "WR": "N",
        "EL": "N",
        "ER": "S",
        "SL": "E",
        "SR": "W"
    }[currentDir + turn]

def findHq(input):
	hqX = 0
	hqY = 0
	currentDirection = "N"

	for instruction in input:
		currentDirection = nextDirection(currentDirection, instruction[0])
		steps = instruction[1:]
		if(currentDirection == "N"):
			hqY += int(steps)
		if(currentDirection == "S"):
			hqY -= int(steps)
		if(currentDirection == "E"):
			hqX += int(steps)
		if(currentDirection == "W"):
			hqX -= int(steps)

	return [hqX, hqY]


def howFarIsHq(input):
	hqCoords = findHq(input)
	distanceX = abs(hqCoords[0])
	distanceY = abs(hqCoords[1])
	return distanceX + distanceY

if __name__ == '__main__':
	input = ["L3", "R2", "L5", "R1", "L1", "L2", "L2", "R1", "R5", "R1", "L1", "L2", "R2", "R4", "L4", "L3", "L3", "R5", "L1", "R3", "L5", "L2", "R4", "L5", "R4", "R2", "L2", "L1", "R1", "L3", "L3", "R2", "R1", "L4", "L1", "L1", "R4", "R5", "R1", "L2", "L1", "R188", "R4", "L3", "R54", "L4", "R4", "R74", "R2", "L4", "R185", "R1", "R3", "R5", "L2", "L3", "R1", "L1", "L3", "R3", "R2", "L3", "L4", "R1", "L3", "L5", "L2", "R2", "L1", "R2", "R1", "L4", "R5", "R4", "L5", "L5", "L4", "R5", "R4", "L5", "L3", "R4", "R1", "L5", "L4", "L3", "R5", "L5", "L2", "L4", "R4", "R4", "R2", "L1", "L3", "L2", "R5", "R4", "L5", "R1", "R2", "R5", "L2", "R4", "R5", "L2", "L3", "R3", "L4", "R3", "L2", "R1", "R4", "L5", "R1", "L5", "L3", "R4", "L2", "L2", "L5", "L5", "R5", "R2", "L5", "R1", "L3", "L2", "L2", "R3", "L3", "L4", "R2", "R3", "L1", "R2", "L5", "L3", "R4", "L4", "R4", "R3", "L3", "R1", "L3", "R5", "L5", "R1", "R5", "R3", "L1"]
	print howFarIsHq(input)
