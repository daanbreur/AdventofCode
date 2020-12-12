# EXPECTED: byr, iyr, eyr, hgt, hcl, ecl, pid, cid
from functools import reduce
from re import match
import time

with open('input.txt') as file:
  passports = file.read().split('\n\n')

def validate(data):
  if not "byr" in data: return False
  if not "iyr" in data: return False
  if not "eyr" in data: return False
  if not "hgt" in data: return False
  if not "hcl" in data: return False
  if not "ecl" in data: return False
  if not "pid" in data: return False

  byr = int(data["byr"])
  iyr = int(data["iyr"])
  eyr = int(data["eyr"])
  hgt = str(data["hgt"])
  pid = str(data["pid"])
  hcl = str(data["hcl"])
  ecl = str(data["ecl"])

  #print( f"byr: {byr} iyr: {iyr} eyr: {eyr} hgt: {hgt} pid: {pid} hcl: {hcl} ecl: {ecl}" )

  if not 1920 <= byr <= 2002: return False
  if not 2010 <= iyr <= 2020: return False
  if not 2020 <= eyr <= 2030: return False
  if len(pid) != 9: return False
  if not match(r"^#[0-9a-f]{6}$", hcl): return False
  if not match(r"(amb|blu|brn|gry|grn|hzl|oth)", ecl): return False
  

  if (hgt.endswith("cm")):    
    return 150 <= int(hgt.replace("cm", "")) <= 193
  elif (hgt.endswith("in")):
    return 59 <= int(hgt.replace("in", "")) <= 76
  else: return False

def part1():
  countPart1, countPart2 = 0, 0
  
  for passport in passports:
    passport = [i for i in passport.replace('\n',' ').split(' ')]
    for key, value in enumerate(passport): 
      if len(value) == 0: passport.pop(key)
    data = dict(string.split(':') for string in passport)
    if all(i in data for i in ('byr','iyr','eyr','hgt','hcl','ecl','pid')):
      countPart1+=1
      if validate(data): countPart2+=1
  return countPart1, countPart2

output1, output2 = part1()
start_time = time.time_ns()
print('\033[38;2;60;179;113mDay04 Part 1: {} \033[0m'.format(output1))
print('\033[38;2;60;179;113mDay04 Part 2: {} \033[0m'.format(output2))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay04: {(end_time - start_time)/1000000} ms \033[0m')