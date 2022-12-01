with open("input.txt", "r") as f:
    data = [int(line) for line in f.readlines()]

count = 0
for i in range(3, len(data)):
    last = data[i - 1] + data[i - 2] + data[i - 3]
    now = data[i] + data[i - 1] + data[i - 2]

    if now > last:
        count += 1

print(count)
