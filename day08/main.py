from pathlib import Path
from argparse import ArgumentParser

import ast

def main(file_path: Path) -> None:

    with open(file_path, "r") as f:
        data = f.read().splitlines()
    
    encoded_strs = ['"' + s.replace("\\", r"\\").replace('"', r'\"') + '"' for s in data]

    print("Part 1: ", sum(len(s) for s in data) - sum(len(ast.literal_eval(s)) for s in data))
    print("Part 2: ", sum(len(s) for s in encoded_strs) - sum(len(ast.literal_eval(s)) for s in encoded_strs))

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