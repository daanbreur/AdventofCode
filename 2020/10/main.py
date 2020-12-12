import time

with open('input.txt') as file:
  contents = list(map(int, file.read().splitlines()))


def part1():
  prev, count1, count3 = 0, 0, 1
  data = contents.copy()
  data.sort()
  for i in data:
    if i - prev == 1: count1 += 1
    elif i - prev == 3: count3 += 1
    prev = i
  return count1 * count3

def part2():
  data = contents.copy()
  data.sort() 
  data.append(data[-1]+1)
  temp = {0: 1}
  for r in data:
    temp[r] = temp.get(r-3,0) + temp.get(r-2,0) + temp.get(r-1,0)
  return temp[data[-1]]

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay10 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay10 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay10: {(end_time - start_time)/1000000} ms \033[0m')