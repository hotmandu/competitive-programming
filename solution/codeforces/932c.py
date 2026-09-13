# Submitted: `PyPy 3-64`
import sys
import random

inputx = sys.stdin.readline
printx = sys.stdout.write

n, a, b = map(int, inputx().split())

d = [None] * (n + 1)
d[0] = (0, 0)

for i in range(0, n):
    if d[i] is not None:
        if i + a <= n:
            d[i + a] = (d[i][0] + 1, d[i][1])
        if i + b <= n:
            d[i + b] = (d[i][0], d[i][1] + 1)

if d[n] is None:
    printx('-1')
else:
    ca, cb = d[n]
    ans = []
    for i in range(ca):
        ans.append(a * (i + 1) - 1)
        ans.extend(range(a * i, a * (i + 1) - 1))
    for i in range(cb):
        ans.append(a * ca + b * (i + 1) - 1)
        ans.extend(range(a * ca + b * i, a * ca + b * (i + 1) - 1))
    printx(' '.join(map(lambda x: str(x + 1), ans)))
