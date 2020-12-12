import time

with open('input.txt') as file:
  contents = file.read().splitlines()


def part1():
  count=0
  for line in contents:
    params = line.split(' ')
    charCount = params[2].count(params[1][:-1])
    [minI,maxI] = params[0].split('-')
    #print(params, charCount, minI, maxI)
    if charCount >= int(minI) and charCount <= int(maxI):
      count+=1
  return count

def part2():
  count=0
  for line in contents:
    params = line.split(' ')
    [pos1I,pos2I] = list(map(int, params[0].split('-')))
    #print(params, pos1I, pos2I)
    if (params[2][pos1I-1] == params[1][:-1]) != (params[2][pos2I-1] == params[1][:-1]):
      count += 1
  return count

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay02 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay02 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay02: {(end_time - start_time)/1000000} ms \033[0m')