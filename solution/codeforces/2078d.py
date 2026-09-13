# Submitted: `PyPy 3-64`
import sys
input = sys.stdin.readline
for _ in range(int(input().strip())):
    n = int(input().strip())
    a = [None] * n
    for i in range(n):
        l = input().split()
        lp = int(l[1]) * (1 if l[0]=='x' else -1)
        rp = int(l[3]) * (1 if l[2]=='x' else -1)
        a[i] = (lp, rp)
    # print(a)
    d = [None] * n
    d[0] = (0, 0)
    for i in range(1,n):
        m1=d[i-1][0] + (((1+max(d[i-1][0], d[i-1][1])) * (a[n-i][0] - 1)) if a[n-i][0] > 0 else 0)
        m2=d[i-1][1] + (((1+max(d[i-1][0], d[i-1][1])) * (a[n-i][1] - 1)) if a[n-i][1] > 0 else 0)
        d[i] = (m1, m2)
    # print(d)
    left, right = 1, 1
    for i in range(n):
        pl = ((left * (a[i][0] - 1)) if a[i][0] > 0 else (-a[i][0])) + ((right * (a[i][1] - 1)) if a[i][1] > 0 else (-a[i][1]))
        if d[n-i-1][0] > d[n-i-1][1]:
            left += pl
        else:
            right += pl
    print(left + right)