nums = [
        52,
        54,
        56,
        60,
        62,
        63,
        65,
        65,
        67,
        69,
        70,
        71,
        72,
        76,
        77,
        78,
        81,
        82,
        83,
        85,
        86,
        87,
        89,
        93,
        95,
        98]

def main():
    primes = [2]
    for i in range(3, 99, 2):
        for p in primes:
            if i % p == 0:
                break
        else:
            primes.append(i)
    print(primes)
    primes.reverse()
    all = set()
    for n in nums:
        nn = n
        d = []
        for p in primes:
            if p < nn and nn % p == 0:
                d.append(p)
                nn //= p
        if not d:
            all.add(n)
        else:
            for x in d:
                all.add(x)
        print(n, d)
    all = list(all)
    all.sort()
    print(all)

if __name__ == '__main__':
    main()
