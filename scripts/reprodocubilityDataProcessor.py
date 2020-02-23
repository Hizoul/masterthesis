import json
import os
from os.path import isfile, join
import numpy as np
baseDir = "python-rl/models/"
table_data_repro = [
  ["Reward Type", "All Equal", "Amount of Steps", "Episode Reward", "Score"]
]
table_data_reward = [
  ["Reward Type", "Amount of Steps", "Episode Reward", "Score"]]
reward_types = ["0", "1", "2"]
def process_dir(dir_name, entry_name, table_data):
  global table_data_repro
  global table_data_reward
  global reward_types
  listing = os.listdir(join(baseDir, dir_name))
  reward_type_res = {}
  print("iter is",)
  for reward_type in reward_types:
    res = {
      "allEqual": [],
      "allExceptFirst": [],
      "deviationReward": [],
      "avgReward": [],
      "avgScore": [],
      "scoreDeviation": [],
      "avgReach": [],
      "reachDeviation": []
    }
    for f in listing:
      process_file(dir_name, f, res, reward_type)
    print("ACESS WITH ", reward_type)
    reward_type_res[reward_type] = res
  for reward_type in reward_types:
    d = reward_type_res[reward_type]
    total = len(d["allEqual"])
    print("RES IS", np.count_nonzero(d["allExceptFirst"]), d["allExceptFirst"])
    table_data.append([entry_name+"-"+reward_type, str(np.count_nonzero(d["allExceptFirst"]))+" of "+str(total), np.average(d["avgReach"]), np.average(d["reachDeviation"]),
     np.average(d["avgReward"]), np.average(d["deviationReward"]),
     np.average(d["avgScore"]), np.average(d["scoreDeviation"])])
    table_data_repro.append([entry_name+"-"+reward_type, str(np.count_nonzero(d["allExceptFirst"]))+" of "+str(total), np.average(d["reachDeviation"]), np.average(d["deviationReward"]), np.average(d["scoreDeviation"])])
    
    table_data_reward.append([entry_name+"-"+reward_type, np.average(d["avgReach"]),
     np.average(d["avgReward"]),
     np.average(d["avgScore"]),])

def all_equal(arr):
  return len(set(arr)) == 1

def process_file(dir_name, content, res, reward_type):
  if content.endswith(".json") and content[6] == reward_type:
    with open(join(baseDir, dir_name, content), 'r') as myfile:
      fileData = myfile.read()
      data = json.loads(fileData)
      print("chec",data)
      reward = []
      step = []
      score = []
      for entry in data:
        reward.append(entry[0])
        step.append(entry[1])
        score.append(entry[2][0])
        score.append(entry[2][1])
      print("EQ", all_equal(reward[1:]), all_equal(reward))
      res["allEqual"].append(all_equal(reward))
      res["allExceptFirst"].append(all_equal(reward[1:]))
      res["deviationReward"].append(np.std(np.array(reward[1:])/100))
      res["avgReward"].append(np.average(np.array(reward[1:])/100))
      res["reachDeviation"].append(np.std(step[1:]))
      res["avgReach"].append(np.average(step[1:]))
      res["scoreDeviation"].append(np.std(score[2:]))
      res["avgScore"].append(np.average(score[2:]))

def array_to_latex(data):
  size = len(data[0])
  table = ""
  table += "\\begin{table}[]\n\\begin{tabular}{"
  for i in range(0,size):
    table += "|l"
  table += "|}\n\\hline\n"
  top_index = 0
  for label in data[0]:
    table += label
    if top_index >= size-1:
      table += " \\\\ \\hline\n"
    else:
      table += " & "
    top_index += 1
  top_index = 0
  for entries in data[1:]:
    sub_index = 0
    for entry in entries:
      if isinstance(entry, (int, np.int, np.float)):
        table += str(round(entry, 2))
      else:
        table += str(entry)
      if sub_index >= size-1:
        table += " \\\\ \\hline\n"
      else:
        table += " & "
      sub_index += 1
  table += "\\end{tabular}\n\\end{table}"
  return table

def make_table_from_dirs():
  global table_data_reward
  global table_data_repro
  dirs=[["self_seed", "self"]]
  table_data = []
  table_data.append([
    "Reward Type", "All Equal", "Step Avg", "Step Std", "Reward Avg", "Reward Std", "Score Avg", "Score Std"
  ])
  for directory in dirs:
    process_dir(directory[0], directory[1], table_data)
  print("table", table_data)
  print(array_to_latex(table_data))
  print(array_to_latex(table_data_repro))
  print(array_to_latex(table_data_reward))

make_table_from_dirs()