import sys
import gym
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common.vec_env import DummyVecEnv,VecCheckNan
import numpy as np
from os import listdir
from os.path import isfile, join
from stable_baselines.gail import ExpertDataset
from generate_pretraindata import generate_pretraindata_heuristics
from rustyblocks import rustLib, field_to_array, field_to_log

MAX_FOR_BLOCKS = 230
botToUse = sys.argv[1]
# oldLog = sys.argv[2]
oldLog = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":0,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":8,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":3,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadConsidering\":{\"play_index\":0}}]}"

is_box_space = True
dirname = "D:\\4-System\\rusty\\"
filename="50000_heutistic_pretrain_"
filename += "box" if is_box_space else "discrete"
np.seterr(all='raise')

origEnv = gym.make("rustybox-v0" if is_box_space else "rustydiscrete-v0")

origEnv.max_invalid_tries = 7
env = VecCheckNan(DummyVecEnv([lambda: origEnv]))

# Instantiate the agent
model = PPO2.load("models/ppo2boxbestparam/2e4-30.pkl", env=env)
# model.load("models/pretrain/"+filename)

rustLib.field_restore_log(origEnv.field, oldLog.encode('utf-8'))
obs = field_to_array(origEnv.field)
actions, _states = model.predict(obs)
if is_box_space:
  action = 0
  current_max = -1
  for i in range(0, len(actions)):
    action_probability = actions[i]
    newIndex = MAX_FOR_BLOCKS + i
    action_possible = obs[int(newIndex / 10)][newIndex % 10] == 1
    if action_probability > current_max and action_possible:
      current_max = action_probability
      action = i
    rustLib.field_do_action(origEnv.field, action)
else:
  rustLib.field_do_action(origEnv.field, actions)
print("1;"+field_to_log(origEnv.field))