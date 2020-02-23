from env_getter import make_env, make_env_hex
from ppo2_tuner_box import ppo_study
from a2c_tuner_box import a2c_study

ppo_study(use_cnn=False,simple_reward=False)
ppo_study(use_cnn=False,simple_reward=True)
ppo_study(use_cnn=True,simple_reward=False)
ppo_study(use_cnn=True,simple_reward=True)
a2c_study(use_cnn=False,simple_reward=False)
a2c_study(use_cnn=False,simple_reward=True)
a2c_study(use_cnn=True,simple_reward=False)
a2c_study(use_cnn=True,simple_reward=True)