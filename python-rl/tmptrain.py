import gym
import sys
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common.vec_env import DummyVecEnv,VecCheckNan
import numpy as np
from os import listdir
from os.path import isfile, join
np.seterr(all='raise')
npseed = 230386042
np.random.seed(npseed)
gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
env = gym.make("rustyblocks-v0")
env.max_invalid_tries = 7
env = VecCheckNan(DummyVecEnv([lambda: env]))
begin = 0
step_size = int(2e4)
# dirname = "models/"
# logdirname = "boardlog/"
# modeldir = "ppo2boxbestparam/"
# file_step = "2e4-"
# for f in listdir(dirname+modeldir):
#   if f.startswith(file_step):
#     end =  f[len(file_step):]
#     num = int(end[:len(end)-4])
#     if num > begin:
#       begin = num
# seed 420420420
# Instantiate the agent
# model = PPO2('MlpPolicy', env,verbose=1,max_grad_norm=1.42481794257356,cliprange=1.36870169927419, vf_coef=0.487354638658612, ent_coef=0.000130839434944482, gamma=0.993211512071304, lam=0.92669713813749, learning_rate=0.00150606967404027, n_steps=709, noptepochs=35,nminibatches=1)
# Train the agent
print("STARTING TO LEARN", begin)
model = PPO2.load("models/pretrain/50000_heutistic_pretrain_discrete.pkl", env=env)
while True:
  env.reset()
  model.learn(total_timesteps=step_size,log_interval=100)
  begin += 1
  # Save the agent
  # model.save(dirname+modeldir+file_step+str(begin))
  print("saved learnstep", begin, "total iterations are now", begin * step_size)