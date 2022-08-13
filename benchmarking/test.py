import os


def parse_time(text):
    in_index = text.find("in") + 2
    second_index = text.find("seconds")
    print(float(text[in_index + 1:second_index]))
    return float(text[in_index + 1:second_index])


total_time = 0.0
ran_tests = 0
try:
    with open("benchmarking/data-set.txt", "r") as file:
        tests = 0
        lines = file.readlines()
        for line in lines:
            if tests == 0:
                tests = line
            else:
                with open("board.txt", "w") as board:
                    board.writelines(line)
                with os.popen("./main") as f:
                    total_time += parse_time(f.readlines()[-1])
                    ran_tests += 1
                    print(f"Running test {ran_tests}")

        print(f"Elapsed: {total_time / float(ran_tests)}")
except KeyboardInterrupt:
    avg = total_time / float(ran_tests)
    print(f"\nProgram ran {ran_tests} tests with an avg of {avg} seconds")
