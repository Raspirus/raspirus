import sys
from setuptools import find_packages, setup

for line in sys.path:
     print(line)

print("Packages:")
print(find_packages())