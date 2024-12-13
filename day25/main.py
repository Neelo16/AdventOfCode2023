import functools

import networkx as nx


def main():
    graph = nx.Graph()
    with open("input.txt") as f:
        for line in f.readlines():
            node, edges = line.strip().split(": ")
            edges = edges.split(" ")
            for edge in edges:
                graph.add_edge(node, edge)
    edge_cuts = list(nx.minimum_edge_cut(graph))
    for edge in edge_cuts:
        graph.remove_edge(*edge)
    print("First star:", functools.reduce((lambda a, b: a * b), [len(s) for s in nx.connected_components(graph)], 1))


if __name__ == '__main__':
    main()
