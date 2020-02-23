import json
import os
from os.path import isfile, join
import numpy as np
import math
baseDir = "D:\\4-System\\0master\\"

def process_dir(dirname):
  listing = os.listdir(join(baseDir, dirname))
  allEpisodes = []
  per20k = []
  for f in listing:
    process_file(dirname, f, allEpisodes, per20k)
  with open(join(baseDir, dirname, "all.json"), 'w') as myfile:
    myfile.write(json.dumps(allEpisodes))
  with open(join(baseDir, dirname, "heuristic-rewards-per20k.json"), 'w') as myfile:
    myfile.write(json.dumps(per20k))

def process_file(dirname, content, allEpisodes, per20k):
  if content.endswith("-rewards.json"):
    with open(join(baseDir, dirname, content), 'r') as myfile:
      fileData = myfile.read()
      data = json.loads(fileData)
      total = 0
      entryAmount = 0
      for entry in data:
        if not math.isnan(entry):
          allEpisodes.append(entry)
          total += entry
          entryAmount += 1
      per20k.append(total / entryAmount)
      
process_dir("heu")
process_dir("selfexp_done1")
process_dir("selfexp_done11")
process_dir("selfexp_done111")
process_dir("selfexp_done2")
process_dir("selfexp_done22")
process_dir("selfexp_done222")
process_dir("selfexp_long1")
process_dir("selfexp_long2")
process_dir("selfexp_nobleed_1")
process_dir("selfexp_nobleed_2")