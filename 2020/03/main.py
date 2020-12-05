import math
with open('input.txt') as file:
  contents = file.read().splitlines()


def part1():
  x, y, count = 0, 0, 0
  while x < len(contents[0]) and y < len(contents):
    #print( f'X: {x} ; Y: {y} ; ICON: {contents[y][x]}' )
    if contents[y][x] == '#': count += 1
    x += 3; y += 1
    if (x >= len(contents[0])): x -= math.floor(x / len(contents[0])) * len(contents[0])
  return count

def part2():
  slopes, treeCount = [ [1,1] , [3,1] , [5,1] , [7,1] , [1,2] ], 1
  for slope in slopes:
    x, y, count = 0, 0, 0
    while x < len(contents[0]) and y < len(contents):
      #print( f'X: {x} ; Y: {y} ; ICON: {contents[y][x]}' )
      if contents[y][x] == '#': count += 1
      x += slope[0]; y += slope[1]
      if (x >= len(contents[0])): x -= math.floor(x / len(contents[0])) * len(contents[0])
    #print( f'SLOPE: {slope} ; COUNT: {count}' )
    treeCount *= count
  return treeCount

print('Day03 Part 1: {} '.format(part1()))
print('Day03 Part 2: {} '.format(part2()))