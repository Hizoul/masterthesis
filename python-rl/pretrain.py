import gym
import sys
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common.vec_env import DummyVecEnv,VecCheckNan
import numpy as np
from os import listdir
from os.path import isfile, join
from stable_baselines.gail import ExpertDataset
from generate_pretraindata import generate_pretraindata_heuristics
# Using only one expert trajectory
# you can specify `traj_limitation=-1` for using the whole dataset

is_box_space = False
dirname = "D:\\4-System\\rusty\\"
filename="400000_heutistic_pretrain_"
filename += "box" if is_box_space else "discrete"
dataset = ExpertDataset(expert_path=dirname + filename+'.npz', batch_size=1,sequential_preprocessing=True)
# pretrain_data = generate_pretraindata_heuristics(int(5e4), int(100e5), is_box_space)
# dataset = ExpertDataset(traj_data=pretrain_data, batch_size=1,sequential_preprocessing=True)
np.seterr(all='raise')

env = gym.make("rustybox-v0" if is_box_space else "rustydiscrete-v0")

env.max_invalid_tries = 7
env = VecCheckNan(DummyVecEnv([lambda: env]))

# Instantiate the agent
model = PPO2('MlpPolicy', env, nminibatches=1)

model.pretrain(dataset, n_epochs=1)
# Save the agent
model.save("models/pretrain/"+filename)