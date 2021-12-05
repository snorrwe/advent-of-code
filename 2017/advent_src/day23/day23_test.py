from .day23 import part1, part2

INPUT = """set b 79
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23""".split('\n')


def test_part1():
    actual = part1(INPUT)
    assert actual == 5929


def test_part2():
    assert part2() == 907


if __name__ == '__main__':
    import pytest
    pytest.main()
