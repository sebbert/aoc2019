import re

INS_REGEX = re.compile("([LRUD])(\d+)")

def parse_ins(ins_str):
  direction, delta_str = re.match(INS_REGEX, ins_str).group(1,2)
  return (direction, int(delta_str))

def parse_wire(wire_str):
  return [parse_ins(ins) for ins in wire_str.split(",")]

def get_wires():
  with open("./input", "r") as file:
    wires = file.readlines()
    return [parse_wire(w) for w in wires]

def add_vec(a, b):
  (ax, ay), (bx, by) = a, b
  return (ax+bx, ay+by)

dir_to_vec_dict = {
  "U": ( 0,  1),
  "D": ( 0, -1),
  "L": (-1,  0),
  "R": ( 1,  0)
}
def dir_vec(direction):
  return dir_to_vec_dict.get(direction)

def all_points(wire):
  cur_pos = (0,0)
  points = dict()
  steps = 0

  for ins in wire:
    direction, delta = ins
    direction_vec = dir_vec(direction)
    for i in range(0, delta):
      steps += 1
      cur_pos = add_vec(cur_pos, direction_vec)
      points[cur_pos] = steps

  return points

[points0, points1] = [all_points(wire) for wire in get_wires()]

def find_intersections():
  for point, steps0 in points0.items():
    steps1 = points1.get(point)
    if steps1:
      yield steps0 + steps1

closest_point = min(find_intersections())

print(closest_point)
