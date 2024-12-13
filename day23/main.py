import time
from collections import defaultdict


class Node:
    def __init__(self, position):
        self.edges = set()
        self.position = position


class Edge:
    def __init__(self, source, target, weight):
        self.source = source
        self.target = target
        self.weight = weight

    def __repr__(self):
        return f'Edge(from={self.source.position}, to={self.target.position}, weight={self.weight})'

    def __hash__(self):
        return hash((self.source, self.target, self.weight))

    def __eq__(self, other):
        return self.source == other.source and self.target == other.target and self.weight == other.weight


class DefaultNodeDict(dict):
    def __missing__(self, key):
        self[key] = Node(key)
        return self[key]


def possible_neighbors(position):
    x, y = position
    return [(x + dx, y + dy) for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]]


def is_slope(c):
    return c in '<>v^'


def build_graph(start, goal, grid, graph):
    queue = list([(graph[start], start, 0, set())])
    explored = set()
    while queue:
        start_node, pos, distance, visited = queue.pop()
        visited.add(pos)
        for neighbor in possible_neighbors(pos):
            if neighbor in visited or grid[neighbor] == '#' or (
                    is_slope(grid[neighbor]) and pos != valid_entrance(grid[neighbor], neighbor)):
                continue
            if neighbor == goal or is_slope(grid[neighbor]):
                start_node.edges.add(Edge(start_node, graph[neighbor], distance + 1))
                if is_slope(grid[neighbor]) and neighbor not in explored:
                    explored.add(neighbor)
                    queue.append((graph[neighbor], neighbor, 0, set(visited)))
            elif grid[neighbor] == '.':
                queue.append((start_node, neighbor, distance + 1, visited))
                visited.add(neighbor)


def topological_sort(graph):
    incoming = {}
    for position in graph:
        incoming[position] = 0

    for position in graph:
        for edge in graph[position].edges:
            incoming[edge.target.position] += 1

    ordered_nodes = []
    while incoming:
        nodes = [graph[p] for p in incoming if incoming[p] == 0]
        for node in nodes:
            del incoming[node.position]
            for edge in node.edges:
                incoming[edge.target.position] -= 1
        ordered_nodes.extend(nodes)
    return ordered_nodes


def longest_path(start, goal, graph):
    ordered = topological_sort(graph)
    distances = dict((position, float('-inf')) for position in graph)
    distances[start] = 0
    for node in ordered:
        for edge in node.edges:
            distances[edge.target.position] = max(distances[edge.target.position],
                                                  distances[node.position] + edge.weight)
    return distances[goal]


def valid_entrance(slope, position):
    x, y = position
    return {
        '>': (x - 1, y),
        '<': (x + 1, y),
        'v': (x, y - 1),
        '^': (x, y + 1)
    }[slope]


def uphill_longest_path(start_pos, goal_pos, graph):
    start = graph[start_pos]
    goal = graph[goal_pos]
    queue = [(start, 0, set())]
    longest = float('-inf')
    while queue:
        node, distance, visited = queue.pop()
        visited.add(node)
        for edge in node.edges:
            neighbor = edge.target
            if neighbor == goal:
                longest = max(longest, distance + edge.weight)
                continue
            if neighbor in visited:
                continue
            queue.append((neighbor, distance + edge.weight, set(visited)))
    return longest


def build_part_2_graph(start, goal, grid, graph):
    visited = set()
    queue = [start]
    while queue:
        pos = queue.pop()
        visited.add(pos)
        if len([n for n in possible_neighbors(pos) if is_slope(grid[n])]) > 1:
            graph[pos] = Node(pos)
        for n in possible_neighbors(pos):
            if n != goal and grid[n] != "#" and n not in visited:
                queue.append(n)
    graph[start] = Node(start)
    graph[goal] = Node(goal)

    for node in graph.values():
        queue = [(node.position, 0)]
        visited = set()
        while queue:
            pos, distance = queue.pop()
            visited.add(pos)
            for n in possible_neighbors(pos):
                if n in graph and n not in visited:
                    node.edges.add(Edge(node, graph[n], distance + 1))
                elif grid[n] != '#' and n not in visited:
                    queue.append((n, distance + 1))
    for node in graph.values():
        for edge in node.edges:
            edge.target.edges.add(Edge(edge.target, edge.source, edge.weight))


def main():
    with open("input.txt") as file:
        lines = [line.strip() for line in file.readlines()]

    width = len(lines[0])
    height = len(lines)
    start = (1, 0)
    goal = (width - 2, height - 1)
    grid = defaultdict(lambda: '#')

    for y in range(height):
        for x in range(width):
            grid[(x, y)] = lines[y][x]

    graph = DefaultNodeDict()
    build_graph(start, goal, grid, graph)
    print(time.strftime("%a, %d %b %Y %H:%M:%S +0000"))
    print("First star:", longest_path(start, goal, graph))
    print(time.strftime("%a, %d %b %Y %H:%M:%S +0000"))
    graph.clear()
    build_part_2_graph(start, goal, grid, graph)
    print(time.strftime("%a, %d %b %Y %H:%M:%S +0000"))
    print("Second star:", uphill_longest_path(start, goal, graph))
    print(time.strftime("%a, %d %b %Y %H:%M:%S +0000"))


if __name__ == "__main__":
    main()
