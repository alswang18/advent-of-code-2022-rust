res = 0
cur = 0
with open("input.txt", 'r') as f:
    f.readlines().strip()
    for line in f:
        line = line.strip("\n")
        if line == "":
            cur = 0
        else:
            cur += int(line)
        res = max(res, cur)