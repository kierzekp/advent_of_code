import re

def get_multiplication_result(memory_match_str: str) -> int:
    split = memory_match_str.split(',')
    (left, right) = (int(split[0].replace("mul(", "")), int(split[1].replace(")", "")))
    return left * right

def analyse_memory(memory_lines: list[str], use_control_instructions: bool = False) -> int:
    pattern = r"(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))"
    rolling_sum = 0
    ignore_muls = False

    for line in memory_lines:
        for match in re.findall(pattern, line):
            if not use_control_instructions:
                if match[0] != '':
                    rolling_sum += get_multiplication_result(str(match[0]))
            else:
                if match[1] != '':
                    ignore_muls = False
                elif match[2] != '':
                    ignore_muls = True
                if match[0] != '' and not ignore_muls:
                    rolling_sum += get_multiplication_result(str(match[0]))

    return rolling_sum
            

with open("03_input.txt", "r") as corrupted_memory:
    memory_lines = corrupted_memory.readlines()
    print("Part one -> Result of all multiplications: {}".format(analyse_memory(memory_lines, False)))
    print("Part two -> Result of all multiplications: {}".format(analyse_memory(memory_lines, True)))