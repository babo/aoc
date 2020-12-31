import re

def main():
    count = 0
    c = re.compile(r'^(\d+)-(\d+) (\w): (\w+)$')
    b = lambda a, b, c, d: (1 if d[int(a) - 1] == c else 0) + (1 if d[int(b) - 1] == c else 0)
    with open('./input.txt') as fd:
        for line in fd.readlines():
            m = c.match(line)
            if m:
                try:
                    if b(m[1], m[2], m[3], m[4]) == 1:
                        count += 1
                except:
                    pass
    print(count)

if __name__ == '__main__':
    main()
