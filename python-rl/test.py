import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import PPO2
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym

gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
env = gym.make('rustyblocks-v0')
env = DummyVecEnv([lambda: env])
print("VEC ENV IS", env)
print("SUB IS", env[0])