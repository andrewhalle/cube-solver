import json
from itertools import permutations

map = {}
for index, val in enumerate(permutations([0, 1, 2, 3, 4, 5, 6, 7])):
    map["".join([str(x) for x in val])] = index

out = open("tables/corner_permutations.json", "w")
out.write(json.dumps(map))
out.close()
