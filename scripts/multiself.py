import json
import os
from os.path import isfile, join
import numpy as np
import math
import sys
baseDir = "D:\\4-System\\0master\\multiself"

def findmax(agent):
  listing = os.listdir(join(baseDir))
  maxVal = 0
  for f in listing:
    if f.startswith(agent+"-") and f.endswith("all.json"):
      end = f.index("all.json")
      val = int(f[2:end])
      if val > maxVal:
        maxVal = val
  return maxVal

def process_dir():
  listing = os.listdir(join(baseDir))
  agents = ["0","1","2","3","4"]
  for agent in agents:
    allEpisodes = []
    per100k = []
    maxVal = findmax(agent)
    print("max js", maxVal)
    total = 0
    entryAmount = 0
    for i in range(1, maxVal+1):
      with open(join(baseDir, agent+"-"+str(i)+"all.json"), 'r') as myfile:
        fileData = myfile.read()
        data = json.loads(fileData)
        for entry in data:
          if not math.isnan(entry):
            allEpisodes.append(entry)
            total += entry
            entryAmount += 1
        if i > 0 and i % 5 == 0:
          per100k.append((total / entryAmount) / 100)
          total = 0
          entryAmount = 0
    with open(join(baseDir, agent+"-all-rewards.json"), 'w') as myfile:
      myfile.write(json.dumps(allEpisodes))
    with open(join(baseDir, agent+"-per100k-rewards.json"), 'w') as myfile:
      myfile.write(json.dumps(per100k))
  maxEntries100k = sys.maxsize
  maxEntriesAll = sys.maxsize
  for agent in agents:
    with open(join(baseDir, agent+"-per100k-rewards.json"), 'r') as myfile:
      fileData = myfile.read()
      dataAmount = len(json.loads(fileData))
      if dataAmount < maxEntries100k:
        maxEntries100k = dataAmount
    with open(join(baseDir, agent+"-all-rewards.json"), 'r') as myfile:
      fileData = myfile.read()
      dataAmount = len(json.loads(fileData))
      if dataAmount < maxEntriesAll:
        maxEntriesAll = dataAmount
  
  for agent in agents:
    newData = []
    with open(join(baseDir, agent+"-per100k-rewards.json"), 'r') as myfile:
      fileData = myfile.read()
      newData = json.loads(fileData)[:maxEntries100k]
    with open(join(baseDir, agent+"-per100k-rewards.json"), 'w') as myfile:
      myfile.write(json.dumps(newData))
    with open(join(baseDir, agent+"-all-rewards.json"), 'r') as myfile:
      fileData = myfile.read()
      newData = json.loads(fileData)[:maxEntriesAll]
    with open(join(baseDir, agent+"-all-rewards.json"), 'w') as myfile:
      myfile.write(json.dumps(newData))
      

process_dir()