with open("./input.txt") as fd:
    l = fd.readlines()
len (l)
l
map(l, int)
map(l, lambda x: int(x))
ll=[int(x) for x in l]
ll
len(ll)
lls = set(ll)
for x in lls:
    if 2020-x in lls:
        print(x, 2020-x)
1611*409
min(ll)
duplo=set()
for x in ll:
    for y in ll:
        if x!=y and x+y<2020-132:
            duplo[x+y]=(x, y)
duplo={}
for x in ll:
    for y in ll:
        if x!=y and x+y<2020-132:
            duplo[x+y]=(x, y)
len(duplo)
for x in ll:
    if 2020-x in duplo:
        print(x, duplo[2020-x])
for x in ll:
    if 2020-x in duplo:
        print(x, duplo[2020-x])
        print(x*duplo[2020-x][0]*duplo[2020-x][1])
