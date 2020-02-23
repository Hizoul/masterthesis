import os
import sys
i = 0
print("DOING TRAIN LOOP")
while True:
  i+=1
  print("Train Step is at ", i)
  os.system("python" + " train.py" if len(sys.argv) == 1 else sys.argv[1])