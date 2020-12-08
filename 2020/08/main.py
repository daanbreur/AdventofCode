with open('input.txt') as file:
  instructions = file.read().splitlines()

def part1():
  accumulator, index = 0, 0
  history = []
  stopped = False
  while not stopped and index < len(instructions):
    if index in history: return accumulator

    opcode = instructions[index].split(' ')[0]
    arg = instructions[index].split(' ')[1]
    history.append(index)
    print(f"Instruction: {instructions[index]} | OPCode: {opcode} Argument: {arg}")
    if opcode == "acc": accumulator += int(arg)
    if opcode == "jmp":
      index += int(arg)
    else: index += 1


def part2():
  pass

print('Day08 Part 1: {} '.format(part1()))
print('Day08 Part 2: {} '.format(part2()))