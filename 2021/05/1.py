with open("input.txt", "r") as f:
    data = [[[int(num) for num in xy.split(",")] for xy in line.strip().split(" -> ")] for line in f.readlines()]

maxX = 0
maxY = 0
for line in data:
    maxX = max(maxX, line[0][0], line[1][0])
    maxY = max(maxY, line[0][1], line[1][1])

grid = []
for _ in range(0, maxY+1):
    grid.append([0] * (maxX + 1))

for line in data:
    x1, y1 = line[0]
    x2, y2 = line[1]

    if x1 != x2 and y1 != y2:
        continue

    # print("1:", x1, y1, x2, y2)©

    if x1 == x2:
        first = min(y1, y2)
        last = max(y1, y2)
        # print("2:", first, last)
        for i in range(first, last):
            grid[i][x1] += 1
            # print(grid[i])
    elif y1 == y2:
        first = min(x1, x2)
        last = max(x1, x2)
        # print("3:", first, last)
        for i in range(first, last):
            grid[y1][i] += 1


count = 0
for line in grid:
    # print(line)
    for x in line:
        if x > 1:
            count += 1

print(count)

# for i in range(16, 128):
#     grid[i][0] += 1
#
# print(grid[17][0])
