#!/usr/bin/env python3

import hashlib
import sys


def h(x):
    m = hashlib.sha256()
    m.update(x.encode())
    return m.hexdigest()


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: ./test.py <num1> <num2>")
        exit(1)

    num1 = int(sys.argv[1])
    num2 = int(sys.argv[2])

    h1 = h(open("src/confession_real.txt", "r").read() + " " * num1)
    h2 = h(open("src/confession_fake.txt", "r").read() + " " * num2)

    print("Hash of real:", h1)
    print("Hash of fake:", h2)

    matching = 0
    for a, b in zip(reversed(h1), reversed(h2)):
        if a == b:
            matching += 1
        else:
            break
    print(matching, "matching hex digits!")
