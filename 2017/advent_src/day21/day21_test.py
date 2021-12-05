from .day21 import run


TEST_RULES = """../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#""".split('\n')


def test_simple():
    actual = run(TEST_RULES, 2)
    assert actual == 12


if __name__ == '__main__':
    import pytest
    pytest.main()
