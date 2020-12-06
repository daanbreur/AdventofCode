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


print('Day04 Part 1: {} '.format(part1()))
print('Day04 Part 2: {} '.format(part2()))