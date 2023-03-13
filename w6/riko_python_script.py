{a:{b:c-graph[a][b] for b,c in d.items() if graph[a][b]<c} for a,d in orggraph.items() }

new_graph = {}
for from_node, edges in orggraph.items():
    for to_node, cap in edges.items():
        if graph[from_node][to_node] < cap:
            new_graph[from_node] = {to_node: cap - graph[from_node][to_node]}
