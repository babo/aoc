import re


def bone(pattern, nums):
    n = len(nums)
    d = len(pattern) - sum(nums) - (n - 1)
    cache = {}

    def vdot(fr, ln):
        return "#" not in pattern[fr : fr + ln]

    def vhash(fr, ln):
        return "." not in pattern[fr : fr + ln]

    def inner(offset, pos, extra):
        key = (offset, pos, extra)
        if key in cache:
            return cache[key]
        count = 0
        if pos >= n:
            if vdot(offset, extra):
                count = 1
        else:
            if pos > 0 and pos < n:
                if not vdot(offset, 1):
                    return 0
                offset += 1

            for i in range(extra + 1):
                if not vdot(offset, i):
                    continue
                if pos < n and not vhash(offset + i, nums[pos]):
                    continue
                count += inner(offset + i + nums[pos], pos + 1, extra - i)
        cache[key] = count
        return count

    return inner(0, 0, d)


def puzzle(filename, repeat=1):
    with open(filename) as fd:
        content = [line.strip() for line in fd.readlines()]

    rtv = []
    m = re.compile(r"\.+")
    for line in content:
        pat = line.split()[0]
        extended = "?".join([pat] * repeat).strip(".")
        reduced = m.sub(".", extended)
        nums = [int(x) for x in line.strip().split()[1].split(",")] * repeat
        rtv.append((reduced, nums))
    return rtv


def solve(filename, repeat=1):
    rtv = 0
    for reduced, nums in puzzle(filename, repeat):
        d = bone(reduced, nums)
        rtv += d
    return rtv


def main():
    a = solve("input.txt", 1)
    print(a)
    b = solve("input.txt", 5)
    print(b)


if __name__ == "__main__":
    main()
