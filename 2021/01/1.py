with open("input.txt", "r") as f:
    data = [int(line) for line in f.readlines()]

last = 0
c = -1
for line in data:
    if last < line:
        c += 1
    last = line

print(c)
