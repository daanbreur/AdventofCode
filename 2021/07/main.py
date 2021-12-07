import time
import math
import numpy as np

def part1(inputData):
  return int(sum(abs(np.median(inputData) - inputData[i]) for i in range(len(inputData))))

def part2(inputData):
  return int(min((sum(((abs(pos-i)+(abs(pos-i)**2))/2) for i in inputData) for pos in range(min(inputData), max(inputData)))))

if __name__ == '__main__':
  with open("input.txt") as file: data = list(map(int,file.read().split(',')))
  start_time = time.time_ns()
  print('\033[38;2;60;179;113mDay7 Part 1: {} \033[0m'.format(part1(data)))
  print('\033[38;2;60;179;113mDay7 Part 2: {} \033[0m'.format(part2(data)))
  end_time = time.time_ns()
  print(f'\033[38;2;60;179;113mDay7: {(end_time - start_time)/1000000} ms \033[0m')