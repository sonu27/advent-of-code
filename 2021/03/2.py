with open("input.txt", "r") as f:
    data = [line.strip() for line in f.readlines()]


def mcv(arr, i, ans=[]) -> int:
    if not arr or i >= len(arr[0]):
        str1 = ''.join(str(e) for e in ans)
        return int(str1, 2)

    ones = []
    zeros = []
    for j in range(0, len(list(arr))):
        if arr[j][i] == "1":
            ones.append(arr[j])
        else:
            zeros.append(arr[j])

    if len(ones) >= len(zeros):
        ans.append(1)
        return mcv(ones, i + 1, ans)
    else:
        ans.append(0)
        return mcv(zeros, i + 1, ans)


def lcv(arr, i, ans=[]) -> int:
    if i >= 12:
        str1 = ''.join(str(e) for e in ans)
        return int(str1, 2)

    ones = []
    zeros = []
    for j in range(0, len(list(arr))):
        if arr[j][i] == "1":
            ones.append(arr[j])
        else:
            zeros.append(arr[j])

    if len(zeros) <= len(ones) or len(ones) == 0:
        ans.append(0)
        return lcv(zeros, i + 1, ans)
    else:
        ans.append(1)
        return lcv(ones, i + 1, ans)


m = mcv(data.copy(), 0)
l = lcv(data.copy(), 0)

print(m * l)
