import gym
import sys
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common.vec_env import DummyVecEnv,VecCheckNan
import numpy as np
from os import listdir
from os.path import isfile, join
np.seterr(all='raise')
# npseed = 5438364
# np.random.seed(npseed)
gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
env = gym.make("rustyblocks-v0")
env = DummyVecEnv([lambda: env])
begin = 0
step_size = int(2e4)
dirname = "models/"
logdirname = "boardlog/"
modeldir = "lstm/"
file_step = "2e4-"
for f in listdir(dirname+modeldir):
  if f.startswith(file_step):
    end =  f[len(file_step):]
    num = int(end[:len(end)-4])
    if num > begin:
      begin = num
# seed 420420420
# Instantiate the agent
model = PPO2('MlpLstmPolicy', env,verbose=1,tensorboard_log=logdirname,cliprange_vf=-1,learning_rate=0.003,nminibatches=1)
# Train the agent
print("STARTING TO LEARN", begin)
if begin > 0:
  model.load(dirname+modeldir+file_step+str(begin),env=env)
# while True:
env.reset()
model.learn(total_timesteps=step_size,log_interval=100)
begin += 1
# Save the agent
model.save(dirname+modeldir+file_step+str(begin))
print("saved learnstep", begin, "total iterations are now", begin * step_size)