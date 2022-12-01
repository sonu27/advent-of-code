with open("input.txt", "r") as f:
    data = [line.strip() for line in f.readlines()]

half = len(data) / 2
mcb = [0] * len(data[0])

for line in data:
    for i in range(0, len(list(line))):
        if line[i] == "1":
            mcb[i] += 1

for i in range(0, len(mcb)):
    if mcb[i] > half:
        mcb[i] = 1
    else:
        mcb[i] = 0

lcb = mcb.copy()
for i in range(0, len(lcb)):
    if lcb[i] == 0:
        lcb[i] = 1
    else:
        lcb[i] = 0

str1 = ''.join(str(e) for e in mcb)
str2 = ''.join(str(e) for e in lcb)

a1 = int(str1, 2)
a2 = int(str2, 2)

print(a1*a2)
