(cat src/confession_real.txt; python3 -c 'print(" "*'"$1"',end="")') | sha256sum
(cat src/confession_fake.txt; python3 -c 'print(" "*'"$2"',end="")') | sha256sum
