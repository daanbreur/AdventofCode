import time

with open('input.txt') as file:
  contents = list(map(int, file.read().splitlines()))


def part1():
  for i in contents:
    for j in contents:
      if i + j == 2020: return i * j

def part2():
  for i in contents:
    for j in contents:
      for n in contents:
        if i + j + n == 2020: return i * j * n

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay01 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay01 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay01: {(end_time - start_time)/1000000} ms \033[0m')