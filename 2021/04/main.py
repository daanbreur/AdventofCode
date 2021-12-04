import time, re, math
with open("input.txt") as file:
    temp = file.read().split('\n\n')
    numbers = list(map(int,temp[0].split(',')))
    gameboards = [ re.sub("\s+"," ",x.replace("\n", " ")).split() for x in temp[1:] ]

def playBoards():
    global gameboards
    for i in numbers:
        print(f"[i] Rolled Number {i}")
        gameboards = [ [ re.sub(f"^{str(i)}$", "x", k) for k in x] for x in gameboards ]
        for idx, gameboard in enumerate(gameboards):
            if ''.join(gameboard[0:5]) == "xxxxx" or ''.join(gameboard[5:10]) == "xxxxx" or ''.join(gameboard[10:15]) == "xxxxx" or ''.join(gameboard[15:20]) == "xxxxx" or ''.join(gameboard[20:25]) == "xxxxx" or \
                (gameboard[0]=="x" and gameboard[5]=="x" and gameboard[10]=="x" and gameboard[15]=="x" and gameboard[20]=="x") == True or \
                (gameboard[1]=="x" and gameboard[6]=="x" and gameboard[11]=="x" and gameboard[16]=="x" and gameboard[21]=="x") == True or \
                (gameboard[2]=="x" and gameboard[7]=="x" and gameboard[12]=="x" and gameboard[17]=="x" and gameboard[22]=="x") == True or \
                (gameboard[3]=="x" and gameboard[8]=="x" and gameboard[13]=="x" and gameboard[18]=="x" and gameboard[23]=="x") == True or \
                (gameboard[4]=="x" and gameboard[9]=="x" and gameboard[14]=="x" and gameboard[19]=="x" and gameboard[24]=="x") == True: 
                yield sum([int(i) for i in gameboard if type(i)== int or i.isdigit()]) * i
                del gameboards[idx]

playedBoards = list(playBoards())

def part1():
    return playedBoards[0]
                
def part2():
    return playedBoards[-1]

start_time = time.time_ns()
print('\033[38;2;60;179;113mDay4 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay4 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay4: {(end_time - start_time)/1000000} ms \033[0m')