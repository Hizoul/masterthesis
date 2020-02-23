from stable_baselines import PPO2
from stable_baselines.common.policies import FeedForwardPolicy, register_policy
from train_func import do_train
from env_getter import make_env, make_env_heu
import tensorflow as tf
from stable_baselines.a2c.utils import conv, linear, conv_to_fc, batch_to_seq, seq_to_batch, lstm
import numpy as np

env = make_env_heu(use_cnn=False)
model = PPO2("MlpPolicy", env,n_steps=1912,gamma=0.893772595247008,vf_coef=0.124083762089201,max_grad_norm=0.790427907967279,learning_rate=0.0106800972074722,ent_coef=3.11268993114624e-07,cliprange=1.13010015406973,noptepochs=7,lam=0.80055096907225)
np.random.seed(7225283)
do_train(model, "heu", env)