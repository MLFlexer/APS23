from collections import defaultdict

graph = defaultdict(list)

visited = set()

def scaling(current, cap, visited):
    if current in visited:
        return False
    if current == t:
        return True
    
    visited.add(current)
    
    for adjacent in graph[current]:
        print("adjacent: ")
        print(adjacent)
        if adjacent[1] >= cap:
            if scaling(adjacent[0],cap, visited):
                adjacent[1] -= cap
                adjacent[2] += cap
                return True
            
    return False


n,m,s,t = map(int, input().split())
# n = number of nodes in graph
# m = number of edges in graph
# s = source
# t = sink

queue = []
maxcap = 0
# [[to,capacity,used capacity]]
for i in range(m):
    u,v,c = map(int, input().split())
    graph[u].append([v,c,0])
    if c > maxcap:
        maxcap = c

threshold = 2**27
while(maxcap < threshold):
    threshold = threshold//2

done = False
while(not done):
    visited = set()
    if scaling(s,threshold, visited):
        continue
    threshold = threshold//2
    if threshold == 0:
        threshold = 1
    visited = set()
    if threshold == 1 and not scaling(s,threshold, visited):
        done = True
    continue

maxflow = 0
for edge in graph[s]:
    maxflow += edge[2]

counter = 0
edgesused = set()
for key,value in graph.items():
    for e in value:
        if e[2] != 0:
            counter += 1
            edgesused.add((key,e[0],e[2]))


print(f"{n} {maxflow} {counter}")
for edge in sorted(edgesused):
    print(*edge)