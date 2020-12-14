import time
import math
with open('input.txt') as file:
  timestamp, busIds = file.read().splitlines()
  timestamp = int(timestamp)
  busIds = [ int(n) for n in busIds.split(',') if n != 'x' ]

def part1():
  minDiff, minId = math.inf, 0
  for busId in busIds:
    currentTime = 0 
    while currentTime <= timestamp: currentTime += busId
    if minDiff > (currentTime - timestamp):
      minDiff = currentTime - timestamp
      minId = busId
  return minId * minDiff

def part2():
  pass

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay13 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay13 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay13: {(end_time - start_time)/1000000} ms \033[0m')