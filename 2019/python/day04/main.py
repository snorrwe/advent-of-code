from collections import Counter


def validate(pw):
    pw = str(pw)
    return (list(pw) == sorted(pw) and len(pw) != len(set(pw)))


assert(validate(111111))
assert(not validate(223450))
assert(not validate(123789))

count = 0
for i in range(240920, 789857):
    if validate(i):
        count += 1

print("part1: ", count)


def validate_p2(pw):
    pw = str(pw)
    count = Counter(pw)
    pt2 = any(v == 2 for v in count.values())
    return (list(pw) == sorted(pw) and pt2)


assert(validate_p2(112233))
assert(not validate_p2(123444))
assert(validate_p2(111122))

count = 0
for i in range(240920, 789857):
    if validate_p2(i):
        count += 1

print("part2: ", count)
