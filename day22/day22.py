from collections import defaultdict
import sys

class Block:
    def __init__(self, left_corner, right_corner):
        x1, y1, z1 = left_corner
        x2, y2, z2 = right_corner
        x1, x2 = min(x1, x2), max(x1, x2)
        y1, y2 = min(y1, y2), max(y1, y2)
        z1, z2 = min(z1, z2), max(z1, z2)
        self.x_values = range(x1, x2 + 1)
        self.y_values = range(y1, y2 + 1)
        self.z_values = range(z1, z2 + 1)
        self.blocks_below = set()
        self.blocks_above = set()

    def overlaps(self, other):
        x_overlaps = len(range(max(self.x_values[0], other.x_values[0]), min(self.x_values[-1], other.x_values[-1]) + 1)) > 0
        y_overlaps = len(range(max(self.y_values[0], other.y_values[0]), min(self.y_values[-1], other.y_values[-1]) + 1)) > 0
        return x_overlaps and y_overlaps

    @property
    def top_z(self):
        return self.z_values[-1]

    @property
    def bottom_z(self):
        return self.z_values[0]

    def lower(self):
        self.z_values = range(self.bottom_z - 1, self.top_z)

    def __repr__(self):
        return f'Block(x: {self.x_values}, y: {self.y_values}, z: {self.z_values})'

def parse(line):
    left, right = line.split("~")
    left = tuple(int(x) for x in left.split(","))
    right = tuple(int(x) for x in right.split(","))
    return Block(left, right)

with open(sys.argv[1]) as f:
    blocks = [parse(l.strip()) for l in f.readlines()]

lines = defaultdict(set)

for block in blocks:
    for z in block.z_values:
        lines[z].add(block)

changed = True

while changed:
    changed = False
    for z in reversed(range(min(lines) + 1, max(lines) + 1)):
        for block in [b for b in lines[z]]:
            if block.bottom_z == 1 or any(b != block and b.overlaps(block) for b in lines[block.bottom_z - 1]):
                continue
            changed = True
            lines[z].remove(block)
            block.lower()
            lines[block.bottom_z].add(block)

for z in reversed(sorted(lines)):
    for block in lines[z]:
        if block.bottom_z == 1:
            continue

        for other in lines[block.bottom_z - 1]:
            if other.overlaps(block):
                block.blocks_below.add(other)
                other.blocks_above.add(block)

print("First star:", len([block for block in blocks if all(len(b.blocks_below) > 1 for b in block.blocks_above)]))

def affected_blocks(block, supports):
    supported = [b for b in block.blocks_above if not any(support not in supports for support in b.blocks_below)]
    for b in supported:
        supports.add(b)
    for b in supported:
        affected_blocks(b, supports)
    return supports

count = 0
for block in blocks:
    # We don't count the block itself as being affected, so we subtract one
    count += len(affected_blocks(block, set([block]))) - 1

print("Second star:", count)
