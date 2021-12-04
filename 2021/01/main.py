from re import search
import time

def part1(inputData):
    increasedCount = 0
    prev_measurement = inputData[0]
    for i in range(1, len(inputData)):
        if inputData[i] > prev_measurement:
            # print(f"[Increased] {prev_measurement} -> {inputData[i]}")
            increasedCount += 1 
        prev_measurement = inputData[i]
    return increasedCount

def part2(inputData):
    data = [sum(inputData[i:i+3]) for i in range(len(inputData))]
    increasedCount = 0
    prev_measurement = data[0]
    for i in range(1, len(data)):
        if data[i] > prev_measurement:
            # print(f"[Increased] {prev_measurement} -> {data[i]}")
            increasedCount += 1 
        prev_measurement = data[i]
    return increasedCount

if __name__ == '__main__':
    with open("input.txt") as file: contents = list(map(int,file.read().splitlines()))
    start_time = time.time_ns()
    print('\033[38;2;60;179;113mDay1 Part 1: {} \033[0m'.format(part1(contents)))
    print('\033[38;2;60;179;113mDay1 Part 2: {} \033[0m'.format(part2(contents)))
    end_time = time.time_ns()
    print(f'\033[38;2;60;179;113mDay1: {(end_time - start_time)/1000000} ms \033[0m')