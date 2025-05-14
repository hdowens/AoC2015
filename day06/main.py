from pathlib import Path
from argparse import ArgumentParser
import re
import numpy as np
from functools import lru_cache

W, H = 999, 999


def main(file_path: Path) -> None:

    with open(file_path, "r") as f:
        instructions = f.read().splitlines()

    instr_re = re.compile(r"(turn on|turn off|toggle)\s+(\d+),(\d+)\s+through\s+(\d+),(\d+)")

    grid   = np.zeros((H+1, W+1), dtype=bool)
    lights = np.zeros((H+1, W+1), dtype=int)

    for instr in instructions:
        op, x0, y0, x1, y1 = instr_re.match(instr).groups()
        x0,y0,x1,y1 = map(int, (x0,y0,x1,y1))
        sl = np.s_[y0:y1+1, x0:x1+1]

        if op == "turn on":
            grid [sl] = True
            lights[sl] += 1

        elif op == "turn off":
            grid [sl] = False
            lights[sl]  = np.maximum(lights[sl] - 1, 0)

        else: 
            grid [sl] = ~grid[sl]
            lights[sl] += 2

    print("Part 1:", np.count_nonzero(grid))
    print("Part 2:", np.sum(lights))
    

if __name__ == "__main__":
    parser = ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "-t", "--test", action="store_true", help="Read from test_input.txt"
    )
    group.add_argument(
        "-p", "--puzzle", action="store_true", help="Read from puz_input.txt"
    )

    args = parser.parse_args()

    if args.test:
        file_path = Path("test.txt")
    elif args.puzzle:
        file_path = Path("input.txt")

    main(file_path)