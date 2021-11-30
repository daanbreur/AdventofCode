import time
with open('input.txt') as file:
  contents = file.read().splitlines()

def part1():
  northPos,eastPos, angle = 0, 0, 0
  for i in contents:
    direction, number = i[0], int(i[1:])
    if direction == 'N': northPos+=number
    elif direction == 'S': northPos-=number
    elif direction == 'E': eastPos+=number
    elif direction == 'W': eastPos-=number
    elif direction == 'R': angle=(angle+number)%360
    elif direction == 'L': angle=(angle-number)%360
    elif direction == 'F':
      if angle == 0: northPos-=number
      elif angle == 90: eastPos+=number
      elif angle == 180: northPos+=number
      elif angle == 270: eastPos-=number
  return abs(eastPos) + abs(northPos)

def part2():
  wpNorthPos,wpEastPos = 1, 10
  northPos,eastPos = 0, 0
  for i in contents:
    direction, number = i[0], int(i[1:])
    if direction == 'N': wpNorthPos+=number
    elif direction == 'S': wpNorthPos-=number
    elif direction == 'E': wpEastPos+=number
    elif direction == 'W': wpEastPos-=number
    elif direction == 'R': 
      for i in range(int(number / 90)):
        tmp = wpNorthPos
        northPos = wpEastPos * -1
        eastPos = tmp
    elif direction == 'L': 
      for i in range(int(number / 90)):
        tmp = eastPos
        eastPos = northPos * -1
        northPos = tmp
    elif direction == 'F':
      northPos = wpNorthPos * number
      eastPos = wpEastPos * number
  return abs(northPos) + abs(eastPos)

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay12 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay12 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay12: {(end_time - start_time)/1000000} ms \033[0m')