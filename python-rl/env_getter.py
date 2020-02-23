import gym
from stable_baselines.common.vec_env import DummyVecEnv

gym.register(id="rustyblocks-box-v0", entry_point="custom_env_boxactions:RustyBlocksEnv")
gym.register(id="rustyblocks-heu-v0", entry_point="custom_env_heu:RustyBlocksEnv")
gym.register(id="hex-v0", entry_point="hex_env:HexEnv")
def make_env(**kwargs):
  env = gym.make("rustyblocks-box-v0",**kwargs)
  env.max_invalid_tries = 7
  env = DummyVecEnv([lambda: env])
  return env
def make_env_heu(**kwargs):
  env = gym.make("rustyblocks-heu-v0",**kwargs)
  env = DummyVecEnv([lambda: env])
  return env
def make_env_hex(**kwargs):
  env = gym.make("hex-v0",**kwargs)
  env.max_invalid_tries = 7
  env = DummyVecEnv([lambda: env])
  return env

MAX_FOR_BLOCKS = 230

def get_action_from_box_answer(obs, actions):
  action = 0
  current_max = -1
  reward_modifier = 0
  actual_max = -1
  for i in range(0, len(actions)):
    action_probability = actions[i]
    newIndex = MAX_FOR_BLOCKS + i
    action_possible = (obs[int(newIndex / 10)][newIndex % 10] == 1)
    if action_probability > actual_max:
      actual_max = action_probability
    if action_possible and action_probability > current_max:
      current_max = action_probability
      action = i
  return action