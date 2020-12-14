from re import search
import time
with open("input.txt") as file:
  contents = file.read().splitlines()

def part1():
  currentMask, memory, binSum = None, {}, 0
  for i in contents:
    op, value = i.split(' = ')
    if op == 'mask': currentMask = value
    else:
      addr = int(search(r'.*?\[(.*)].*', op).group(1))
      valueBin = bin(int(value))[2:].zfill(36)
      output = ''
      for index in range(len(currentMask)):
        if str(currentMask)[index] == 'X': output += valueBin[index]
        if str(currentMask)[index] == '1': output += '1'
        if str(currentMask)[index] == '0': output += '0'
        # else: valueMasked += str( int(str(currentMask)[index]) & int(valueBin[index]) )
      memory[addr] = output
      # print(f'{op} {value}\n{valueBin}\n{currentMask}\n{output}')
  for x in memory.values(): binSum += int(x,2)
  return binSum

def part2():
  pass

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay14 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay14 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay14: {(end_time - start_time)/1000000} ms \033[0m')