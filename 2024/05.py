page_ordering_rules = []
page_updates = []

with open("05_test_input.py", "r") as input:
    breakline_hit = False
    data = input.readlines()
    for line in data:
        if line.isspace():
            breakline_hit = True
        else:
            if not breakline_hit:
                page_ordering_rules.append(line.strip())
            else:
                page_updates.append(line.strip())

print(page_ordering_rules)
print(page_updates)