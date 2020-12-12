import time
from copy import deepcopy
with open('input.txt') as file:
  contents = list(map(list, file.read().splitlines()))

def part1():
  seatingMap, prevMap = deepcopy(contents), None
  while seatingMap != prevMap:
    prevMap = deepcopy(seatingMap)
    for y in range(len(contents)):
      for x in range(len(contents[0])):
        state = prevMap[y][x]
        neighbors = countNeighbors(prevMap, x, y)

        if state == 'L' and neighbors == 0: seatingMap[y][x] = '#'
        elif state == '#' and neighbors >= 4: seatingMap[y][x] = 'L'
        
        #print( f'X,Y: {x:.0f} {y:.0f} | {state} -> {seatingMap[y][x]}' )
  return sum( x.count('#') for x in seatingMap )

def countNeighbors(_map, x, y):
  count = 0
  for j in range(-1, 2):
    for i in range(-1, 2):
      if y+j < 0 or y+j>=len(_map) or x+i < 0 or x+i>=len(_map[0]) or (j == 0 and i == 0): continue
      count += ( 1 if _map[y+j][x+i] == '#' else 0 )
  return count
 
def part2():
  pass

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay11 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay11 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay11: {(end_time - start_time)/1000000} ms \033[0m')