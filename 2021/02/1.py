with open("input.txt", "r") as f:
    data = [line.strip().split(" ") for line in f.readlines()]

x = 0
y = 0
for line in data:
    if line[0] == "up":
        y -= int(line[1])
    if line[0] == "down":
        y += int(line[1])
    if line[0] == "forward":
        x += int(line[1])

print(x*y)
