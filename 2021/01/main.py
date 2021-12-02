from re import search
import time
with open("input.txt") as file:
    contents = list(map(int,file.read().splitlines()))

def part1():
    increasedCount = 0
    prev_measurement = contents[0]
    for i in range(1, len(contents)):
        if contents[i] > prev_measurement:
            # print(f"[Increased] {prev_measurement} -> {contents[i]}")
            increasedCount += 1 
        prev_measurement = contents[i]
    return increasedCount

def part2():
    data = [sum(contents[i:i+3]) for i in range(len(contents))]
    increasedCount = 0
    prev_measurement = data[0]
    for i in range(1, len(data)):
        if data[i] > prev_measurement:
            # print(f"[Increased] {prev_measurement} -> {data[i]}")
            increasedCount += 1 
        prev_measurement = data[i]
    return increasedCount


start_time = time.time_ns()
print('\033[38;2;60;179;113mDay1 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay1 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay1: {(end_time - start_time)/1000000} ms \033[0m')