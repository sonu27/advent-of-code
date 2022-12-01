import copy

with open("input.txt", "r") as f:
    d1 = [[char for char in line.strip()] for line in f.readlines()]

h = len(d1)
w = len(d1[0])


def move(d1):
    d2 = copy.deepcopy(d1)
    moved = False
    for i in range(1, len(d1)):
        for j in range(0, len(d1[i])):
            if d1[i][j] == ">":
                if j < w - 1 and d1[i][j + 1] == ".":
                    d2[i][j] = "."
                    d2[i][j + 1] = ">"
                    moved = True
                elif j == w - 1 and d1[i][0] == ".":
                    d2[i][j] = "."
                    d2[i][0] = ">"
                    moved = True

    d1 = copy.deepcopy(d2)
    for i in range(0, len(d1)):
        for j in range(0, len(d1[i])):
            if d1[i][j] == "v":
                if i < i - 1 and d1[i + 1][j] == ".":
                    d2[i][j] = "."
                    d2[i + 1][j] = ">"
                    moved = True
                elif i == h - 1 and d1[0][j] == ".":
                    d2[i][j] = "."
                    d2[0][j] = ">"
                    moved = True
    return moved, d2


moved = True
i = 0
while moved:
    moved, d1 = move(d1)
    i += 1

print(i)
