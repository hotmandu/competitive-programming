# Submitted: `PyPy 3-64` 
import sys
import random

import math

inputx = sys.stdin.readline
printx = sys.stdout.write

x2 = int(inputx())

largex = 1_000_000
sqrtx = math.floor(math.sqrt(largex))

nps = set()
ps = []

for p in range(2, largex):
    if p not in nps:
        ps.append(p)
        ecl = p ** 2
        while ecl <= largex:
            nps.add(ecl)
            ecl = ecl + p

pss = set(ps)

k = None

t = x2
for p in ps:
    if t == 1:
        break
    if t % p == 0 and (k is None or k > max(x2 - p, p)):
        k = max(x2 - p, p)
    while t % p == 0:
        t //= p

min_k = None

for p1 in filter(lambda x: x <= x2, ps):
    t = k // p1 * p1 + p1
    if t > x2: continue
    for p in ps:
        if t == 1:
            break
        if t in pss:
            if min_k is None or min_k > max((k // p1 * p1 + p1) - t, t):
                min_k = max((k // p1 * p1 + p1) - t, t)
            break
        if t % p == 0 and (min_k is None or min_k > max((k // p1 * p1 + p1) - p, p)):
            min_k = max((k // p1 * p1 + p1) - p, p)
        while t % p == 0:
            t //= p

#print(k,min_k)

if min_k is None:
    print(max(3, k + 1))
else:
    print(max(3, min(k, min_k) + 1))

# x2 -> 이전에 고른 p는 x2의 소인수 중 하나. -> x1은 다음 조건 만족:
# x2 - p < x1. 또한, p < x1, 2 < x1
# max(x2 - p, p) < x1 <= x2
# 따라서 min(max(x2 - p, p)) = k 라고 하면, k < x1 <= x2 인 x1 중에, min(max(x1 - p', p')) 찾는 문제
# 각 p당, 후보 x1은 1개.



