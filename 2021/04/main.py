import time, re, math
with open("input.txt") as file:
    temp = file.read().split('\n\n')
    numbers = list(map(int,temp[0].split(',')))
    gameboards = [ re.sub("\s+"," ",x.replace("\n", " ")).split() for x in temp[1:] ]

def playBoards():
    global gameboards
    for i in numbers:
        # print(f"[i] Rolled Number {i}")
        gameboards = [ [ re.sub(f"^{str(i)}$", "x", k) for k in x] for x in gameboards ]
        for idx, gameboard in enumerate(gameboards):
            if any(["".join(gameboard[i*5:i*5+5])=="xxxxx" for i in range(5)]) or any(["".join(gameboard[i::5])=="xxxxx" for i in range(5)]):
                yield sum([int(i) for i in gameboard if type(i)== int or i.isdigit()]) * i
                del gameboards[idx]

def part1():
    return playedBoards[0]
                
def part2():
    return playedBoards[-1]

start_time = time.time_ns()
playedBoards = list(playBoards())
print('\033[38;2;60;179;113mDay4 Part 1: {} \033[0m'.format(part1()))
print('\033[38;2;60;179;113mDay4 Part 2: {} \033[0m'.format(part2()))
end_time = time.time_ns()
print(f'\033[38;2;60;179;113mDay4: {(end_time - start_time)/1000000} ms \033[0m')