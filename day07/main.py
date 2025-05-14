from pathlib import Path
from argparse import ArgumentParser
from collections import defaultdict, deque

def simulate_circuit(data, part2 = False) -> int:
    
    instructions = deque(data)
    
    activated_gates = set()
    gates = defaultdict(int)

    if part2:
        instructions.appendleft("956 -> b")
        pass

    while instructions:
        i = instructions.popleft()
        instr, output = i.split("->")
        instr, output = instr.strip(), output.strip()

        if instr.strip().isnumeric():
            gates[output] = int(instr)
            activated_gates.add(output)

        elif "AND" in instr:
            w0, w1 = map(str.strip, instr.split("AND"))
            if (w0 in activated_gates or w0.isnumeric()) and (w1 in activated_gates or w1.isnumeric()):
                v0 = int(w0) if w0.isnumeric() else gates[w0]
                v1 = int(w1) if w1.isnumeric() else gates[w1]
                gates[output] = v0 & v1
                activated_gates.add(output)
            else:
                instructions.append(i)

        elif "OR" in instr:
            w0, w1 = map(str.strip, instr.split("OR"))
            if (w0 in activated_gates or w0.isnumeric()) and (w1 in activated_gates or w1.isnumeric()):
                v0 = int(w0) if w0.isnumeric() else gates[w0]
                v1 = int(w1) if w1.isnumeric() else gates[w1]
                gates[output] = v0 | v1
                activated_gates.add(output)
            else:
                instructions.append(i)

        elif "NOT" in instr:
            w = instr.split()[1].strip()
            if w in activated_gates:
                gates[output] = (1 << 16) - 1 - gates[w]
                activated_gates.add(output)
            else:
                instructions.append(i)

        elif "LSHIFT" in instr:
            w, shift = map(str.strip, instr.split("LSHIFT"))
            if w in activated_gates:
                gates[output] = gates[w] << int(shift)
                activated_gates.add(output)
            else:
                instructions.append(i)

        elif "RSHIFT" in instr:
            w, shift = map(str.strip, instr.split("RSHIFT"))
            if w in activated_gates:
                gates[output] = gates[w] >> int(shift)
                activated_gates.add(output)
            else:
                instructions.append(i)

        elif instr in activated_gates:
            gates[output] = gates[instr]
            activated_gates.add(output)

        else:
            instructions.append(i)  

    return gates['a']

def main(file_path: Path) -> None:

    with open(file_path, "r") as f:
        data = f.read().splitlines()


    print(f"Part 1: {simulate_circuit(data)}")
    
    print(f"part 2 a: {simulate_circuit(data, part2=True)}")

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