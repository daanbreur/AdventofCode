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
  pass

print('Day02 Part 1: {} '.format(part1()))
print('Day02 Part 2: {} '.format(part2()))