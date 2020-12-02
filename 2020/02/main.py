with open('input.txt') as file:
  contents = file.read().splitlines()


def part1():
  count=0
  for line in contents:
    params = line.split(' ')
    charCount = params[2].count(params[1][:-1])
    [minI,maxI] = params[0].split('-')
    print(params, charCount, minI, maxI)
    if charCount >= int(minI) and charCount <= int(maxI):
      count+=1
  return count

def part2():
  count=0
  for line in contents:
    params = line.split(' ')
    charCount = params[2].count(params[1][:-1])
    [pos1I,pos2I] = list(map(int, params[0].split('-')))
    print(params, charCount, pos1I, pos2I)
    if (params[2][pos1I-1] == params[1][:-1]) != (params[2][pos2I-1] == params[1][:-1]):
      count += 1
  return count

print('Day02 Part 1: {} '.format(part1()))
print('Day02 Part 2: {} '.format(part2()))