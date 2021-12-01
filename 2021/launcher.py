from re import match
import sys
import os
import time

def main():
  print('\033[0;34m  ~\033[0;36m~\033[0;32m  Advent of Code 2021  \033[0;36m~\033[0;34m~')
  print('\033[0;31m ~\033[0;35m~\033[0;33m  Solutions by daanbreur \033[0;35m~\033[0;31m~\n')

  if len(sys.argv) != 2:
    print(f'{getANSIEscape(60, 179, 113)}EVERYTHING mode.')
    for i in range(1,25):
      runDay(str(i))
    return

  print(f'{getANSIEscape(60, 179, 113)}REPL mode. Please enter a command.')
  print(f'{getANSIEscape(205, 92, 92)}  * exit, quit ... exit the program')
  print(f'{getANSIEscape(205, 92, 92)}  * day [1-25]  ... execute the given day ★☆◯')

  while True:
    print(f'{getANSIEscape(210,105,30)}>: \033[0m')
    line = input()
    if line == "exit" or line == "quit": break
    if match(r"day (\d+)", line):
      _, i = line.split(' ')
      runDay(i) 
      continue
    print(f'\033[0mEnter a valid command!')

  
def getANSIEscape(r,g,b):
  return f"\033[38;2;{r};{g};{b}m"

def runDay(day: str):
  os.system(f'cd {day.zfill(2)} && python main.py')


main()