import re

import numpy as np
import sympy


class Hailstone:
    def __init__(self, pos, v):
        self.x, self.y, self.z = pos
        self.vx, self.vy, self.vz = v

    def __repr__(self):
        return f"Hailstone(pos=({self.x}, {self.y}, {self.z}), vel=(x={self.vx},y={self.vy},z={self.vz})"

    @property
    def pos(self):
        return self.x, self.y, self.z

    @property
    def vel(self):
        return self.vx, self.vy, self.vz

    def p1_intersect(self, other):
        a = np.array(([self.vx, -other.vx], [self.vy, -other.vy]))
        b = np.array([other.x - self.x, other.y - self.y])
        if np.linalg.det(a) == 0:
            return []
        x, y = np.linalg.solve(a, b)
        if x < 0 or y < 0:
            return []
        return [self.x + x * self.vx, self.y + x * self.vy]


def parse_line(line):
    match = re.match(r"(-?\d+), +(-?\d+), +(-?\d+) +@ +(-?\d+), +(-?\d+), +(-?\d+)", line)
    values = [int(g) for g in match.groups()]
    return Hailstone(values[:3], values[3:])


def part2(hailstones):
    x, y, z, vx, vy, vz = sympy.symbols('x y z vx vy vz')
    times = []
    equations = []
    for i, hailstone in enumerate(hailstones[:3]):
        t = sympy.symbols(f"t{i}")
        px, py, pz = hailstone.pos
        dx, dy, dz = hailstone.vel
        equations.append(sympy.Eq(px + dx * t, x + vx * t))
        equations.append(sympy.Eq(py + dy * t, y + vy * t))
        equations.append(sympy.Eq(pz + dz * t, z + vz * t))
        times.append(t)
    variables = sympy.solve(equations, (x, y, z, vx, vy, vz, *times), dict=True)[0]
    return variables[x] + variables[y] + variables[z]


def main():
    with open("input.txt") as file:
        hailstones = [parse_line(line.strip()) for line in file.readlines()]
    limits = range(200000000000000, 400000000000000 + 1)
    count = 0
    for (i, h1) in enumerate(hailstones):
        for h2 in hailstones[i + 1:]:
            intersection = h1.p1_intersect(h2)
            if intersection:
                x, y = intersection
                if limits[0] <= x <= limits[-1] and limits[0] <= y <= limits[-1]:
                    count += 1
    print("First star:", count)
    print("Second star:", part2(hailstones))


if __name__ == '__main__':
    main()
