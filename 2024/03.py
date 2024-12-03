import re

with open("03_input.txt", "r") as corrupted_memory:
    memory_lines = corrupted_memory.readlines()
    pattern = r"mul\([0-9]+,[0-9]+\)"

    rolling_sum = 0

    for line in memory_lines:
        for match in re.findall(pattern, line):
            split_str = str(match).split(',')
            (left, right) = (int(split_str[0].replace("mul(", "")), int(split_str[1].replace(")", "")))
            rolling_sum += left * right

    print("Result of all multiplications: {}".format(rolling_sum))