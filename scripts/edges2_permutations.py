import json

from sympy.utilities.iterables import multiset_permutations

map = {}
for index, val in enumerate(multiset_permutations([12, 12, 12, 12, 12, 12, 6, 7, 8, 9, 10, 11])):
    map[",".join([str(x) for x in val])] = index

out = open("tables/edges2_permutations.json", "w")
out.write(json.dumps(map))
out.close()
