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

print('Day01 Part 1: {} '.format(part1()))
print('Day02 Part 2: {} '.format(part2()))