def fuelReq(mass):
  fuel = 0
  while mass > 0:
    mass = mass // 3 - 2
    fuel += max(mass, 0)
  return fuel

with open('day01.data') as fd:
  print(sum([fuelReq(int(x)) for x in fd.readlines()]))
