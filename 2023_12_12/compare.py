
def read_file_into_map(filename):
    with open(filename) as f:
        next(f)
        d = dict()
        for line in f.readlines():
            d.update({ line.split(':')[0]: line.split(':')[1] })
        return d;

non_cache = read_file_into_map("./non_cache.txt")
cache = read_file_into_map("./cache.txt")

for row, val in non_cache.items():
    if cache[row] != val:
        print(f"None cached had {val} and cached had {cache[row]}")
