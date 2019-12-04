from collections import defaultdict


def validate(pw):
    pw = str(pw)
    return (len(pw) == 6 and list(pw) == sorted(pw)
            and len(pw) != len(set(pw)))


print(validate(111111))
print(validate(223450))
print(validate(123789))

count = 0
for i in range(240920, 789857):
    if validate(i):
        count += 1

print("part1: ", count)


def validate_p2(pw):
    pw = str(pw)
    count = defaultdict(lambda: 0)
    for letter in pw:
        count[letter] += 1
    pt2 = any(v == 2 for v in count.values())
    return (len(pw) == 6 and list(pw) == sorted(pw)
            and len(pw) != len(set(pw)) and pt2)

print(validate_p2(112233))
print(validate_p2(123444))
print(validate_p2(111122))

count = 0
for i in range(240920, 789857):
    if validate_p2(i):
        count += 1

print("part2: ", count)
