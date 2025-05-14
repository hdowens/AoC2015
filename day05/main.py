import re

def part1(data) -> int:
    bad_strs = ['ab', 'cd', 'pq', 'xy']
    vowels   = ['a', 'e', 'i', 'o', 'u']

    count = sum(1 for sample in data
          if sum([sample.count(v) for v in vowels]) >= 3
          and not any([b in sample for b in bad_strs])
          and any(
            [sample[idx] == sample[idx+1] for idx in range(len(sample)-1)] 
        )
     )

    return count

def part2(data) -> int:
    nice_strs = set()
    for sample in data:
        repeats = set()
        for idx in range(len(sample)-1):
            substr = sample[idx] + sample[idx+1]
            if sample.count(substr) > 1:
               repeats.add(substr)
               
    
        #look for 3 letter substrs in nice pattern
        if len(repeats) > 0:
            if any(
                sample[i] == sample[i+2] for i in range(0, len(sample)-2)
            ):
                nice_strs.add(sample)
    return len(nice_strs)

def main() -> None:

    with open("input.txt", "r") as file:
        data = file.read().splitlines()

    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")




    
if __name__ == "__main__":
    main()