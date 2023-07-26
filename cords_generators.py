import random

def gen2(n, xmax, ymax):
    f = open("./cords", "w")
    for i in range(n):
        a, b = random.random() * xmax * random.choice([1, -1]), random.random() * ymax * random.choice([1, -1])
        print(f"{a} {b}", file=f)
    f.flush()
    f.close()

def gen3(n, xmax, ymax, zmax):
    f = open("./cords", "w")
    for i in range(n):
        a, b, c = random.random() * xmax * random.choice([1, -1]), random.random() * ymax * random.choice([1, -1]), random.random() * zmax * random.choice([1, -1])
        print(f"{a} {b} {c}", file=f)
    f.flush()
    f.close()