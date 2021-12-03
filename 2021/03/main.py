import time
import math
with open("input.txt") as file:
  data = list(file.read().splitlines())

def part1():
  ones, zeros = [0 for _ in range(len(data[0]))], [0 for _ in range(len(data[0]))]
  gamma_rate, epsilon_rate = "", ""
  for byte in data:
    for i, bit in enumerate(byte):
      if bit == '1': ones[i] += 1 
      else: zeros[i] += 1
  print(f"{ones} {zeros}")
    
  for i in range(len(data[0])):
    gamma_rate += "1" if ones[i] >= zeros[i] else "0"
    epsilon_rate += "1" if zeros[i] > ones[i] else "0"

  print(f"[+] {gamma_rate} -> {int(gamma_rate,2)}")
  print(f"[-] {epsilon_rate} -> {int(epsilon_rate,2)}")
  print(f"[=] {gamma_rate} * {epsilon_rate} -> {int(gamma_rate,2)} * {int(epsilon_rate,2)} = {int(gamma_rate,2) * int(epsilon_rate,2)}")

  return int(gamma_rate,2) * int(epsilon_rate,2)

def part2():
  o2rating, co2rating = data.copy(), data.copy()
  o2ratingCount = [[0 for _ in range(len(o2rating[0]))],[0 for _ in range(len(o2rating[0]))]]
  co2ratingCount = [[0 for _ in range(len(co2rating[0]))],[0 for _ in range(len(co2rating[0]))]]

  for i in range(len(o2rating[0])):
    if len(o2rating) == 1: break
    for byte in o2rating:
      if byte[i] == '1': o2ratingCount[1][i] += 1 
      else: o2ratingCount[0][i] += 1
    temp = []
    for byte in o2rating:
      if byte[i] == ("1" if o2ratingCount[1][i] >= o2ratingCount[0][i] else "0"): temp.append(byte)
      # print(f"[{'+' if byte[i] == ('1' if o2ratingCount[1][i] >= o2ratingCount[0][i] else '0') else '-'}] {byte} Needed Bit: {('1' if o2ratingCount[1][i] >= o2ratingCount[0][i] else '0')}")
    if len(temp) == 0: break
    else: o2rating = temp.copy()
  
  for i in range(len(co2rating[0])):
    if len(co2rating) == 1: break
    for byte in co2rating:
      if byte[i] == '1': co2ratingCount[1][i] += 1 
      else: co2ratingCount[0][i] += 1
    temp = []
    for byte in co2rating:
      if byte[i] == ("1" if co2ratingCount[0][i] > co2ratingCount[1][i] else "0"): temp.append(byte)
      # print(f"[{'+' if byte[i] == ('1' if co2ratingCount[0][i] > co2ratingCount[1][i] else '0') else '-'}] {byte} Needed Bit: {('1' if co2ratingCount[0][i] > co2ratingCount[1][i] else '0')}")
    if len(temp) == 0: break
    else: co2rating = temp.copy()

    
  print(f"[+] {o2rating[0]} -> {int(o2rating[0],2)}")
  print(f"[-] {co2rating[0]} -> {int(co2rating[0],2)}")
  print(f"[=] {o2rating[0]} * {co2rating[0]} -> {int(o2rating[0],2)} * {int(co2rating[0],2)} = {int(o2rating[0],2) * int(co2rating[0],2)}")

  return int(o2rating[0],2) * int(co2rating[0],2)

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay2 Part 3: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay2 Part 3: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay3: {(end_time - start_time)/1000000} ms \033[0m')