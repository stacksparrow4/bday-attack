#!/usr/bin/env python3

import hashlib
import sys


def h(x):
    m = hashlib.sha256()
    m.update(x)
    return m.hexdigest()


if __name__ == "__main__":
    h1 = h(open("real_forged.txt", "rb").read())
    h2 = h(open("fake_forged.txt", "rb").read())

    print("Hash of real:", h1)
    print("Hash of fake:", h2)

    matching = 0
    for a, b in zip(reversed(h1), reversed(h2)):
        if a == b:
            matching += 1
        else:
            break
    print(matching, "matching hex digits!")
