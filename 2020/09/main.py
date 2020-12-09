with open('input.txt') as file:
  contents = file.read().splitlines()

def part1():
  for i in range(25,len(contents)):
    notValid = True
    currentPreamble = contents[i-25:i]
    for n in currentPreamble:
      for m in currentPreamble:
        if int(n) + int(m) == int(contents[i]): notValid = False
    if notValid: return int(contents[i])
        
    

def part2():
  pass
    

print('Day09 Part 1: {} '.format(part1()))
print('Day09 Part 2: {} '.format(part2()))