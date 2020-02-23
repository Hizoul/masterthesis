from stable_baselines import PPO2
from stable_baselines.common.policies import FeedForwardPolicy, register_policy
from train_func import do_train_multi
from env_getter import make_env, make_env_hex
import tensorflow as tf
from stable_baselines.a2c.utils import conv, linear, conv_to_fc, batch_to_seq, seq_to_batch, lstm
import numpy as np

policy_kwargs = dict(net_arch=[dict(pi=[128, 128, 128],vf=[128, 128, 128])])
env = make_env(use_cnn=True,reward_type=0,self_play=True)
model1 = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="D:\\4-System\\tensor",policy_kwargs=policy_kwargs)
model2 = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="D:\\4-System\\tensor",policy_kwargs=policy_kwargs)
model3 = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="D:\\4-System\\tensor",policy_kwargs=policy_kwargs)
model4 = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="D:\\4-System\\tensor",policy_kwargs=policy_kwargs)
model5 = PPO2("CnnPolicy", env, verbose=1,self_play=True,tensorboard_log="D:\\4-System\\tensor",policy_kwargs=policy_kwargs)
do_train_multi([model1, model2, model3, model4, model5], "selfplay", env)