import time

with open('input.txt') as file:
  instructions = file.read().splitlines()

def getSpacer(space, lengthStr):
  return " "*(space-lengthStr)

def part1():
  accumulator, index = 0, 0
  history = []
  stopped = False
  while not stopped and index < len(instructions):
    if index in history: return accumulator

    opcode = instructions[index].split(' ')[0]
    arg = instructions[index].split(' ')[1]
    history.append(index)
    #print(f"Instruction: {instructions[index]} | OPCode: {opcode} Argument: {arg}")
    if opcode == "acc": accumulator += int(arg)
    if opcode == "jmp":
      index += int(arg)
    else: index += 1


def part2():
  accumulator = 0
  for i in range(len(instructions)):
    temp=instructions.copy()
    opcode, arg = temp[i].split(' ')
    if not (opcode == "jmp" or opcode == "nop"): continue
    if opcode == "jmp": temp[i] = f"nop {arg}"  
    elif opcode == "nop": temp[i] = f"jmp {arg}"
    index = 0
    history = []
    stopped = False
    while not stopped and index < len(temp):
      if index in history:
        accumulator = 0
        stopped = True
        break

      opcode = temp[index].split(' ')[0]
      arg = temp[index].split(' ')[1]
      history.append(index)
      #print(f"Instruction: {temp[index]}{getSpacer(10, len(temp[index]))}| OPCode: {opcode}{getSpacer(6, len(opcode))}Argument: {arg}")
      if opcode == "acc": accumulator += int(arg)
      if opcode == "jmp":
        index += int(arg)
      else: index += 1
    if not stopped:
      return accumulator
    
start_time = time.time_ns()
print('\033[38;2;60;179;113mDay08 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay08 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay08: {(end_time - start_time)/1000000}ms')