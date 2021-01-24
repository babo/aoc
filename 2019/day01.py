with open('day01.data') as fd:
  print(sum([int(x) // 3 - 2 for x in fd.readlines()]))
