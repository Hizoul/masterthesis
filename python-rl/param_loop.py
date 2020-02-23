#!/usr/bin/env python
import subprocess
import time
cpu_nums = 2

to_research = ["ppo2_tuner_box.py", "trpo_tuner_box.py", "a2c_tuner_box.py"]
i = 0
processes = []
while True:
  print("CHECKING PROESSES", len(processes))
  newProcesses = []
  for process in processes:
    print("PROCESS IS", process.poll())
    if process.poll() == None:
      newProcesses.append(process)
  diff = cpu_nums - len(newProcesses)
  print("after check is", len (newProcesses), cpu_nums, diff)
  for sub in range(0, diff):
    print("INITING NEW")
    newProcesses.append(subprocess.Popen('python ' + to_research[i % len(to_research)], shell=True))
    i += 1
  processes = newProcesses
  time.sleep(30)