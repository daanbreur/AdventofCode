import time

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
  #print(f"{count} - {len(bags)}")
  if after > now: return checkBags(bags, rules)
  else: return bags

def sumBags(rules, name):
  innerBags, count = None, 1
  for bag in rules:
    if bag['bag'] == name: innerBags = bag['content']
  
  for innerBag in innerBags:
    if not innerBag.startswith("no"):
      count += sumBags(rules, innerBag.replace(innerBag.split(" ")[0] + " ", "") + ("" if innerBag.endswith("s") else "s")) * int(innerBag.split(" ")[0])
  return count

def part1():
  rules = []
  for line in contents:
    data = line.split(" contain ")
    dataDict = dict()
    dataDict['bag'] = data[0]
    dataDict['content'] = data[1].replace(".", "").split(", ")
    rules.append(dataDict)
    #print(dataDict)
  bags = []
  bags.append("shiny gold bag")
  bags = checkBags(bags, rules)
  return len(bags)-1

def part2():
  rules = []
  for line in contents:
    data = line.split(" contain ")
    dataDict = dict()
    dataDict['bag'] = data[0]
    dataDict['content'] = data[1].replace(".", "").split(", ")
    rules.append(dataDict)
    #print(dataDict)
  return sumBags(rules, "shiny gold bags") -1

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay07 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay07 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay07: {(end_time - start_time)/1000000}ms')