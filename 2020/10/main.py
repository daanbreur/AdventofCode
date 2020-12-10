import time

with open('input.txt') as file:
  contents = list(map(int, file.read().splitlines()))


def part1():
  prev, count1, count3 = 0, 0, 1
  data = contents.copy()
  data.sort()
  for i in data:
    if i - prev == 1: count1 += 1
    elif i - prev == 3: count3 += 1
    prev = i
  return count1 * count3

def part2():
  pass

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay10 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay10 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay10: {(end_time - start_time)/1000000} ms \033[0m')