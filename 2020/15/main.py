import time
with open("input.txt") as file:
  contents = list(map(int, file.read().split(',')))

def addtodict(key, val, spoken):
  key = str(key)
  if key not in spoken:
    spoken[key] = { "recent": val, "old": None }
  else:
    spoken[key]["old"] = spoken[key]["recent"]
    spoken[key]["recent"] = val
  spoken["last"] = key

def runTurns(amount):
  i, prev, nums = 0, -1, {}
  for num in range(len(contents)):
    if prev != -1: pass
    prev = num
    i+=1
  for i in range(i, amount):
    pass

  return prev

def part1():
  return runTurns(2020)

def part2():
  return runTurns(30000000)

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay15 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay15 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay15: {(end_time - start_time)/1000000} ms \033[0m')