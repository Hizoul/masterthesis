import gym
import sys
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common.vec_env import DummyVecEnv

gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
env = gym.make("rustyblocks-v0")
env = DummyVecEnv([lambda: env])
model = PPO2.load("models/bothscoresandwin-2e5-4.pkl")

# Enjoy trained agent
obs = env.reset()
dones = False
while not dones:
    prev_obs = obs
    action, _states = model.predict(obs)
    res = env.step(action)
    obs = res[0]
    dones = res[2]
    if dones:
      print("LAST OBS IS", prev_obs)
      sys.exit(0)