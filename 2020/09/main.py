import time

with open('input.txt') as file:
  contents = list(map(int, file.read().splitlines()))

def part1():
  for i in range(25,len(contents)):
    notValid = True
    currentPreamble = contents[i-25:i]
    for n in currentPreamble:
      for m in currentPreamble:
        if int(n) + int(m) == int(contents[i]): notValid = False
    if notValid: 
      return contents[i]
        
    

def part2():
  mainList = []
  target = part1()
  for i in range(len(contents)):
    thisList = contents[i:].copy()
    for j in range(len(thisList)):
      mainList.append(thisList[j])
      if (target == sum(mainList)) and (len(mainList) > 1): return min(mainList) + max(mainList)
      elif target < sum(mainList): mainList.clear()


start_time = time.time_ns()
print('\033[38;2;60;179;113mDay09 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay09 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay09: {(end_time - start_time)/1000000} ms \033[0m')