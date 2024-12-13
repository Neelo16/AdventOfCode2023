from collections import deque
import sys

def neighbors(c):
    x, y = c
    return [(x + dx, y + dy) for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)]]

def calculate(steps):
    global grid
    global start
    visited = set()
    reachable = set()
    queue = deque()
    queue.append((start, steps))
    while queue:
        c, steps_left = queue.popleft()
        if steps_left % 2 == 0:
            reachable.add(c)
        if steps_left == 0:
            continue
        for neighbor in neighbors(c):
            if grid[neighbor] == '#':
                continue
            if neighbor not in visited:
                queue.append((neighbor, steps_left - 1))
                visited.add(neighbor)
    return len(reachable)


class DefaultDict(dict):
    def __missing__(self, key):
        global width
        global height
        (x, y) = key
        self[(x, y)] = self[(x % width, y % height)]
        return self[(x, y)]

grid = DefaultDict()

with open(sys.argv[1]) as f:
    lines = [l.strip() for l in f.readlines()]

with open(sys.argv[1]) as f:
    s = f.read().replace("\n", "")

width = len(lines[0])
height = len(lines)

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        grid[(x, y)] = c

for k in grid:
    if grid[k] == 'S':
        start = k
        break


print("First star:", calculate(64))

y1 = calculate(65)
y2 = calculate(131 + 65)
y3 = calculate(131 * 2 + 65)


x = 202300
ax2 = (y1 + y3 - y2 * 2) * (x * (x - 1) // 2)
bx = x * (y2 - y1)
c = y1

print("Second star:",  ax2 + bx + c)
