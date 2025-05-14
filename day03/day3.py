
dirs = {
    '^' : (0, 1),
    '>' : (1, 0),
    '<' : (-1, 0),
    'v' : (0, -1)
}

def part1(data):
    seen = set()
    pos = (0,0)
    seen.add(pos)
    for char in data:
        move = dirs[char]
        new_pos = (pos[0] + move[0], pos[1] + move[1])
        if new_pos not in seen:
            seen.add(new_pos)
        pos = new_pos

    print(f"Houses visted: {len(seen)}")


def main():
    with open("input.txt", 'r') as file:
        data = list(file.read())

    #data = ['^', 'v','^', 'v','^', 'v','^', 'v','^', 'v']

    seen = set()
    santa_pos = (0,0)
    robot_pos = (0,0)
    seen.add((0,0))

    for idx, char in enumerate(data):
        delta = dirs[char]
        if idx % 2 == 0 :
            santa_pos = (
                santa_pos[0] + delta[0], santa_pos[1] + delta[1]
            )
            seen.add(santa_pos)
        elif idx % 2 == 1:
            robot_pos = (
                robot_pos[0] + delta[0], robot_pos[1] + delta[1]
            )
            seen.add(robot_pos)


    print(len(seen))




if __name__ == "__main__":
    main()


    #for char in santa_instrs:
    #    move = dirs[char]
    #    new_pos = (pos[0] + move[0], pos[1] + move[1])
    #    seen.add(new_pos)
    #    pos = new_pos
#
    #pos = (0,0) #reset
    #for char in robot_instrs:
    #    move = dirs[char]
    #    new_pos = (pos[0] + move[0], pos[1] + move[1])
    #    seen.add(new_pos)
    #    pos = new_pos
