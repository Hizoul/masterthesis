import json
import os
from os.path import isfile, join
import numpy as np
import math

files = [
  "heuristic-rewards-per20k.json", "0-2-rewards-per20k.json",
  "2-0-rewards-per20k.json", "long-1-rewards.json", "long-2-rewards.json",
  "nobleed-1-rewards-per20k.json","nobleed-2-rewards-per20k.json"
]
agents = ["0","1","2","3","4"]
for agent in agents:
  files.append(agent+"-per100k-rewards.json")

for f in files:
  with open(join(f), 'r') as myfile:
    fileData = myfile.read()
    data = json.loads(fileData)
    currentMax = 0
    best_model = 0
    for i in range(0, len(data)):
      if i % 5 == 0:
        if data[i] > currentMax:
          currentMax = data[i]
          best_model = i
    
    print(f, "best model is", best_model, "with reward", data[best_model], "freeindex best is ", np.argmax(data), data[np.argmax(data)])