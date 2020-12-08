with open('input.txt') as file:
  contents = file.read().splitlines()

count = 0

def checkBags(bags, rules):
  global count
  count += 1
  now = len(bags)
  for rule in rules:
    for innerBag in rule['content']:
      if innerBag.endswith('bag'): innerBag += 's'
      for bag in bags:
        if (bag in innerBag) and not (rule['bag'] in bags):
          bags.append(rule['bag'] + ("" if rule['bag'].endswith("s") else "s"))
  after = len(bags)
  print(f"{count} - {len(bags)}")
  if after > now: return checkBags(bags, rules)
  else: return bags
  # stuff

def part1():
  rules = []
  for line in contents:
    data = line.split(" contain ")
    dataDict = dict()
    dataDict['bag'] = data[0]
    dataDict['content'] = data[1].replace(".", "").split(", ")
    rules.append(dataDict)
    print(dataDict)
  bags = []
  bags.append("shiny gold bag")
  bags = checkBags(bags, rules)
  return len(bags)-1

def part2():
  pass

print('Day07 Part 1: {} '.format(part1()))
print('Day07 Part 2: {} '.format(part2()))