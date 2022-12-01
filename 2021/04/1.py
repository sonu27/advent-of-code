with open("input.txt", "r") as f:
    nums, *boards = f.read().split("\n\n")
    nums = [int(i) for i in nums.split(",")]
    boards = [[[int(col) for col in row.split()] for row in board.split("\n")] for board in boards]


def mark_board(board, number):
    for row in board:
        for col in range(0, 5):
            if row[col] == number:
                row[col] = -1


def check_board(board) -> bool:
    for row in board:
        c = 0
        for col in range(0, 5):
            if row[col] == -1:
                c += 1
        if c == 5:
            return True

    for col in range(0, 5):
        c = 0
        for row in range(0, 5):
            if board[row][col] == -1:
                c += 1
        if c == 5:
            return True

    return False


def count_board(board) -> int:
    count = 0
    for row in board:
        for col in row:
            if col != -1:
                count += col
    return count


for num in nums:
    for b in boards:
        mark_board(b, num)
        if check_board(b):
            print(count_board(b)*num)
            exit(1)
