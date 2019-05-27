import json

from sympy.utilities.iterables import multiset_permutations

map = {}
for index, val in enumerate(multiset_permutations([0, 1, 2, 3, 4, 5, 12, 12, 12, 12, 12, 12])):
    map[",".join([str(x) for x in val])] = index

out = open("tables/edges1_permutations.json", "w")
out.write(json.dumps(map))
out.close()
