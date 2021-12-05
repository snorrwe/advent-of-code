def GetTextInputFromFile(fname):
	result = ""
	with open(fname, 'r') as input:
		result = input.read()
	return result

def GetLinesFromFile(fname):
	with open(fname, 'r') as input:
		return input.readlines()
