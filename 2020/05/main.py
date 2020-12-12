import time

with open('input.txt') as file:
  contents = list(file.read().splitlines())

seatIds = []

def filterFunc(i):
  return ((i-1) in seatIds) and not (i in seatIds) and ((i + 1) in seatIds)

def part1():
  for boardingPass in contents:
    rowMin, rowMax = 0, 127
    columnMin, columnMax = 0, 7
    for i in boardingPass:
      if i == "F":
        rowMax = (rowMax - rowMin) / 2 + rowMin
      elif i == "B":
        rowMin = (rowMax - rowMin) / 2 + rowMin + 1
      elif i == "L":
        columnMax = (columnMax - columnMin) / 2 + columnMin
      elif i == "R":
        columnMin = (columnMax - columnMin) / 2 + columnMin + 1
    seatIds.append( rowMin * 8 + columnMin )
  return max(seatIds)

def part2():
  intList = [ i for i in range(0, 128*8) ]
  intList = list(filter(filterFunc, intList))
  return intList[0]

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay05 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay05 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay05: {(end_time - start_time)/1000000} ms \033[0m')