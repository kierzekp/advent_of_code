def add_to_lists(line: str, *lists):
    values = line.split()
    if len(values) == len(lists):
        for index in range(0, len(values)):
            lists[index].append(int(values[index]))

with open("01_input.txt", "r") as input_file:
    left_list = []
    right_list = []
    right_list_occurance_map = {}

    for line in input_file.readlines():
        add_to_lists(line, left_list, right_list)

    left_list.sort()
    right_list.sort()

    rolling_sum = 0
    rolling_similarity_score = 0

    for index in range(0, len(left_list)):
        rolling_sum += abs(left_list[index] - right_list[index])
        if right_list[index] in right_list_occurance_map:
            old_value = right_list_occurance_map[right_list[index]]
            right_list_occurance_map[right_list[index]] = old_value + 1
        else:
            right_list_occurance_map[right_list[index]] = 1

    for value in left_list:
        rolling_similarity_score += value * right_list_occurance_map.get(value, 0)

    print("Total distance: {}".format(rolling_sum))
    print("Total similarity score: {}".format(rolling_similarity_score))