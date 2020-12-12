import time
with open('input.txt') as file:
  contents = list(map(list, file.read().splitlines()))

def part1():
  for y in range(len(contents)):
    for x in range(len(contents[0])):
      print(contents[y][x], end="")
    print("")

def part2():
  pass

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay11 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay11 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay11: {(end_time - start_time)/1000000} ms \033[0m')