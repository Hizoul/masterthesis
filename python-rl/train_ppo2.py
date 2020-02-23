from stable_baselines import PPO2
from stable_baselines.common.policies import FeedForwardPolicy, register_policy
from train_func import do_train
from env_getter import make_env, make_env_hex
import tensorflow as tf
from stable_baselines.a2c.utils import conv, linear, conv_to_fc, batch_to_seq, seq_to_batch, lstm
import numpy as np

policy_kwargs = dict(net_arch=[dict(pi=[128, 128, 128],vf=[128, 128, 128])])
env = make_env(use_cnn=True,reward_type=0,self_play=True)
model = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="./models/selfplay_tensor/",policy_kwargs=policy_kwargs)

do_train(model, "selfplay", env)