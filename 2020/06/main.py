import time

with open('input.txt') as file:
  answerGroups = file.read().split('\n\n')

def part1():
  count = 0
  for answerGroup in answerGroups:
    answerGroup = answerGroup.replace('\n','').replace(' ', '')
    count += len(set(answerGroup.lower()))
  return count

def part2():
  count = 0
  for answerGroup in answerGroups:
    answerGroup = [ i for i in answerGroup.replace('\n',' ').strip().split(' ')]
    for char in answerGroup[0]:
      all = True
      for i in answerGroup[1:len(answerGroup)]:
        if char not in i:
          all = False
      if all: count += 1
  return count

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay06 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay06 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay06: {(end_time - start_time)/1000000} ms \033[0m')