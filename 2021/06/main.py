import time
import math

def part1(inputData):
  fishes = [0]*9
  for i in inputData:
    fishes[i] += 1
  
  for _ in range(80):
    pregnant = fishes[0]
    fishes = fishes[1:] + [0]
    fishes[6] += pregnant
    fishes[8] += pregnant

  return sum(fishes)

def part2(inputData):
  fishes = [0]*9
  for i in inputData:
    fishes[i] += 1
  
  for _ in range(256):
    pregnant = fishes[0]
    fishes = fishes[1:] + [0]
    fishes[6] += pregnant
    fishes[8] += pregnant

  return sum(fishes)

if __name__ == '__main__':
  with open("input.txt") as file: data = list(map(int,file.read().split(',')))
  start_time = time.time_ns()
  print('\033[38;2;60;179;113mDay6 Part 1: {} \033[0m'.format(part1(data)))
  print('\033[38;2;60;179;113mDay6 Part 2: {} \033[0m'.format(part2(data)))
  end_time = time.time_ns()
  print(f'\033[38;2;60;179;113mDay6: {(end_time - start_time)/1000000} ms \033[0m')