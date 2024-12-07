def look_for_words(puzzle: list[str]) -> int:
    word_count = 0
    words = ["XMAS", "XMAS"[::-1]]
    dummy_char = '*'
    for word in words:
        for line_index in range(0, len(puzzle)):
            for column_index in range(0, len(puzzle[line_index])):
                # check for horizontal words
                if column_index + len(word) <= len(puzzle[line_index]) and puzzle[line_index][column_index:column_index+len(word)] == word:
                    print("Found horizontal word {} at coordinates ({}, {})".format(word, column_index, line_index))
                    word_count += 1
                # check for vertical words     
                buffer = "" 
                for subline_iterator in range(0, len(word)):
                    try: 
                        buffer += puzzle[line_index + subline_iterator][column_index]
                    except IndexError:
                        buffer += dummy_char
                if buffer == word:
                    print("Found vertical word {} at coordinates ({}, {})".format(word, column_index, line_index))
                    word_count += 1
                # check for diagonal words going right-down
                buffer = ""
                for sub_iterator in range(0, len(word)):
                    try:
                        buffer += puzzle[line_index + sub_iterator][column_index + sub_iterator]
                    except IndexError:
                        buffer += dummy_char
                if buffer == word:
                    print("Found diagonal right-down word {} at coordinates ({}, {})".format(word, column_index, line_index))
                    word_count += 1
                # check for diagonal words going right-up
                buffer = ""
                for sub_iterator in range(0, len(word)):
                    try:
                        buffer += puzzle[line_index - sub_iterator][column_index + sub_iterator]
                    except IndexError:
                        buffer += dummy_char
                if buffer == word:
                    print("Found diagonal right-up word {} at coordinates ({}, {})".format(word, column_index, line_index))
                    word_count += 1
    return word_count


with open("04_input.txt", "r") as data:
    wordsearch_puzzle = list(map(lambda line : line.strip(), data.readlines()))
    print("Found words: {}".format(look_for_words(wordsearch_puzzle)))


