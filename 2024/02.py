def get_differences_between_levels(levels: list[int]) -> list[int]:
    diffs = []
    for index in range(1, len(levels)):
        diffs.append(levels[index] - levels[index-1])
    return diffs

def check_if_levels_are_safe(levels: list[int]) -> bool:
    differences = get_differences_between_levels(levels)
    positive_count = 0
    negative_count = 0

    for i in range(0, len(differences)):
        diff = differences[i]
        if diff == 0:
            return False
        if diff > 0:
            if negative_count != 0 or diff > 3:
                return False
            positive_count += 1
        if diff < 0:
            if positive_count != 0 or diff < -3:
                return False
            negative_count += 1
        
    return True

def check_if_report_is_safe(report: str, problem_dampener: bool = False) -> bool:
    levels = list(map(int, report.split()))
    safe = check_if_levels_are_safe(levels)
    if problem_dampener and not safe:
        for i in range(0, len(levels)):
            levels_copy = levels.copy()
            levels_copy.pop(i)
            safe_after_removing = check_if_levels_are_safe(levels_copy)
            if safe_after_removing:
                return True
        return False
    return safe

with open("02_input.txt", "r") as data:
    reports = data.readlines()

    safe_report_count = 0
    safe_report_count_with_problem_dampener = 0

    for report in reports:
        if check_if_report_is_safe(report):
            safe_report_count += 1
        if check_if_report_is_safe(report, True):
            safe_report_count_with_problem_dampener += 1

    print("Part one: Safe report count: {}".format(safe_report_count))
    print("Part two: Safe report count with problem dampener: {}".format(safe_report_count_with_problem_dampener))
