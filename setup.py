import os
from pathlib import Path


YEAR = 2022

root = Path(str(YEAR))
os.mkdir(root)

for i in range(1, 26):
    day = root / f"day{i:02d}"
    os.mkdir(day)
    g = day / ".gitkeep"
    g.touch()
