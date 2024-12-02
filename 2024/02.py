def get_differences_between_levels(levels: list[int]) -> list[int]:
    diffs = []
    for index in range(1, len(levels)):
        diffs.append(levels[index] - levels[index-1])
    return diffs

def check_if_report_is_safe(report: str) -> bool:
    levels = list(map(int, report.split()))
    differences = get_differences_between_levels(levels)
    positive_count = 0
    negative_count = 0

    for diff in differences:
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

with open("02_input.txt", "r") as data:
    reports = data.readlines()

    safe_report_count = 0

    for report in reports:
        if check_if_report_is_safe(report):
            safe_report_count += 1

    print("Safe report count: {}".format(safe_report_count))