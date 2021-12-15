import time
import math

maps = {
  'costs': {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
  },
  'autocomplete_costs': {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
  },
  'opening': {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
  },
  'closing': {
    ")": "(",
    "]": "[",
    "}": "{",
    ">": "<",
  }
}

def findCorruptedInLine(line):
  stack = []
  for idx, i in enumerate(line):
    if i in maps['closing']:
      expecting = maps["closing"][i]
      curr = stack.pop()
      if expecting != curr:
        print(f"Invalid token \033[38;2;209;62;29m'{curr}'\033[0m expected \033[38;2;209;62;29m'{expecting}'\033[0m ")
        return maps["costs"][i], []
    else:
      stack.append(i)     
  return 0, [maps['opening'][i] for i in stack[::-1]]



def part1(inputData):
  points = 0
  for idx, instruction in enumerate(inputData):
    points += findCorruptedInLine(instruction)[0]
  return points

def part2(inputData):
  points = []
  incompleteLines = [j[1] for i in inputData if (j := findCorruptedInLine(i))[0] == 0]
  for idx, instruction in enumerate(incompleteLines):
    point = 0
    for char in instruction:
      point = point * 5 + maps["autocomplete_costs"][char]
    points.append(point)

  points.sort()
  return points[len(points)//2]

if __name__ == '__main__':
  with open("input.txt") as file: data = list(file.read().splitlines())
  start_time = time.time_ns()
  print('\033[38;2;60;179;113mDay10 Part 1: {} \033[0m'.format(part1(data)))
  print('\033[38;2;60;179;113mDay10 Part 2: {} \033[0m'.format(part2(data)))
  end_time = time.time_ns()
  print(f'\033[38;2;60;179;113mDay10: {(end_time - start_time)/1000000} ms \033[0m')