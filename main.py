import time
from yaml import dump

output = dump(dict(a=1, b=2))
print(output)

for i in range(0, 50):
    print(i)
    time.sleep(0.5)

