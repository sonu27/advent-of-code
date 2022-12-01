with open("input.txt", "r") as f:
    data = [line.strip().split(" ") for line in f.readlines()]

x = 0
y = 0
a = 0
for line in data:
    if line[0] == "up":
        a -= int(line[1])
    if line[0] == "down":
        a += int(line[1])
    if line[0] == "forward":
        x += int(line[1])
        y += int(line[1]) * a

print(x*y)
