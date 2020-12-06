with open('input.txt') as file:
  answerGroups = file.read().split('\n\n')

def part1():
  count = 0
  for answerGroup in answerGroups:
    answerGroup = answerGroup.replace('\n','').replace(' ', '')
    count += len(set(answerGroup.lower()))
  return count

def part2():
  pass


print('Day04 Part 1: {} '.format(part1()))
print('Day04 Part 2: {} '.format(part2()))