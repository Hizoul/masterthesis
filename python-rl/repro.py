from stable_baselines import PPO2
from stable_baselines.common.policies import FeedForwardPolicy, register_policy
from train_func import train_until_localoptimum, do_train_multi
from env_getter import make_env, make_env_hex
import tensorflow as tf
from stable_baselines.a2c.utils import conv, linear, conv_to_fc, batch_to_seq, seq_to_batch, lstm
import numpy as np
import sys

def init_localoptima_selfplayer(seed, reward_type):
  env = make_env(use_cnn=True,reward_type=reward_type,self_play=True,save_score=True)
  def make_model(env):
    policy_kwargs = dict(net_arch=[dict(pi=[128, 128, 128],vf=[128, 128, 128])])
    return PPO2("CnnPolicy", env, verbose=1,seed=seed,self_play=True,policy_kwargs=policy_kwargs)
  train_until_localoptimum(make_model, seed, env,reward_type)

# used for reproducibility and seed dependency experiment
# seeds = [489887692, 1927674915, 17648, 1757523614, 83695292]
# print("USING SEED ", int(sys.argv[1]))
for i in range(0,5):
  seedToUse = np.random.randint(1, 2147483647)
  for u in range(0,3):
    init_localoptima_selfplayer(seedToUse, u)
