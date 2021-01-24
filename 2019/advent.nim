import options
import streams

type IntState* [T] = (seq[T], T, T)

proc intcode* [T](code: seq[T], instruction: T, baseptr: T, input: proc (): T, output: proc(x: T): void): Option[IntState[T]] =
  var code = code
  var instruction: T = instruction
  var baseptr: T = baseptr

  proc get_code(pos: T): T =
    if pos < code.len: code[pos] else: 0

  proc mode(inst: T, arg: T): T =
    var d: T
    case arg
      of 1: d = 100
      of 2: d = 1000
      of 3: d = 10000
      else: assert arg > 0 and arg < 4
    return (get_code(inst) %% (d * 10)) /% d

  proc get(inst: T, arg: T): T =
    let m = mode(inst, arg)
    case m
      of 0: return get_code(get_code(inst + arg))
      of 1: return get_code(inst + arg)
      of 2: return get_code(baseptr + get_code(inst + arg))
      else: assert m >= 0 and m < 3

  proc put(inst: T, arg: T, value: T): void =
    let m = mode(inst, arg)
    var pos = 0'i64
    case m
      of 0: pos = get_code(inst + arg)
      of 1: pos = inst + arg
      of 2: pos = baseptr + get_code(inst + arg)
      else: assert m >= 0 and m < 3
    if pos >= code.len:
      for i in code.len .. pos + 1:
        code.add(0)
    code[pos] = value

  while true:
    assert instruction >= 0
    case get_code(instruction) %% 100
      of 1: put(instruction, 3, get(instruction, 1) + get(instruction, 2)); instruction += 4
      of 2: put(instruction, 3, get(instruction, 1) * get(instruction, 2)); instruction += 4
      of 3: put(instruction, 1, input()); instruction += 2
      of 4: output(get(instruction, 1)); instruction += 2; break
      of 5:
        if get(instruction, 1) != 0:
          instruction = get(instruction, 2)
        else:
          instruction += 3
      of 6:
        if get(instruction, 1) == 0:
          instruction = get(instruction, 2)
        else:
          instruction += 3
      of 7: put(instruction, 3, if get(instruction, 1) < get(instruction, 2) : 1 else: 0); instruction += 4
      of 8: put(instruction, 3, if get(instruction, 1) == get(instruction, 2) : 1 else: 0); instruction += 4
      of 9: baseptr += get(instruction, 1); instruction += 2
      of 99: return none(IntState[T])
      else: echo "error in the code: ", get_code(instruction), instruction; doAssert(false, "bye now")

  return some((code, instruction, baseptr))

proc runIntCode*[T](code: seq[T], input: proc (): T, output: proc(x: T): void): void =
  var state: IntState[T]
  let s1: T = 0
  let s2: T = 0
  var step = intcode[T](code, s1, s2, input, output)
  while step.isSome():
    state = step.get()
    step = intcode(state[0], state[1], state[2], input, output)

proc readInput*(filename: string): string =
  var strm = newFileStream(filename)
  assert not isNil(strm)
  let input = strm.readAll()
  strm.close()
  return input
