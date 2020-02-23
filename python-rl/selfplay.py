from stable_baselines import PPO2
from stable_baselines.common.policies import FeedForwardPolicy, register_policy
from train_func import train_until_localoptimum, do_train_multi
from env_getter import make_env, make_env_hex
import tensorflow as tf
from stable_baselines.a2c.utils import conv, linear, conv_to_fc, batch_to_seq, seq_to_batch, lstm
import numpy as np
import sys

env = make_env(use_cnn=False,reward_type=0,self_play=True,save_score=False)
def make_model(env):
  policy_kwargs = dict(net_arch=[dict(pi=[128, 128, 128],vf=[128, 128, 128])])
  return PPO2("MlpPolicy", env, verbose=1,self_play=True,policy_kwargs=policy_kwargs)
models = []
for i in range(0, 5):
  models.append(make_model(env))
do_train_multi(models, "multiself", env)