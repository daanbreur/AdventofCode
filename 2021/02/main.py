from re import search
import time

def part1(inputData):
    horizontal_position, depth = 0, 0
    for instruction in inputData:
        [op, num] = instruction.split(' ')
        if op == 'forward': horizontal_position+=int(num)
        elif op == 'down': depth+=int(num)
        elif op == 'up': depth-=int(num)
    return horizontal_position*depth

def part2(inputData):
    aim, horizontal_position, depth = 0, 0, 0
    for instruction in inputData:
        [op, num] = instruction.split(' ')
        if op == 'forward': 
            horizontal_position+=int(num)
            depth+=aim*int(num)
        elif op == 'down': aim+=int(num)
        elif op == 'up': aim-=int(num)
    return horizontal_position*depth


if __name__ == '__main__':
    with open("input.txt") as file: instructions = list(file.read().splitlines())
    start_time = time.time_ns()
    print('\033[38;2;60;179;113mDay2 Part 1: {} \033[0m'.format(part1(instructions)))
    print('\033[38;2;60;179;113mDay2 Part 2: {} \033[0m'.format(part2(instructions)))
    end_time = time.time_ns()
    print(f'\033[38;2;60;179;113mDay2: {(end_time - start_time)/1000000} ms \033[0m')