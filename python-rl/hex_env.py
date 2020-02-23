import numpy as np
import sys
from gym import spaces, Env
from gym.utils import seeding
from rustyblocks import rustLib, hex_to_array

class HexEnv(Env):
  metadata = {'render.modes': ['human', 'ansi']}
  def __init__(self):
    self.action_space = spaces.MultiDiscrete([11, 11]) # x,y
    self.observation_space = spaces.Box(0, 11, shape=(11,11), dtype=np.int64)
    self.game = rustLib.hex_new()
    self.rewards = []
    self.seed()

  def seed(self, seed=None):
    return [seed]

  def step(self, actions):
    answer = rustLib.hex_do_action_with_answer(self.game, actions[0], actions[1])
    placed = answer.placed
    if placed == 0:
      reward = answer.reward
    else:
      reward = -0.001
    new_observation = hex_to_array(self.game)
    done = answer.done == 0
    if done:
      print("mstch done", reward, new_observation)
    return new_observation, reward, done, {}

  def render(self, mode='human'):
    outfile = StringIO() if mode == 'ansi' else sys.stdout
    outfile.write(hex_to_array(self.game))
    # No need to return anything for human
    if mode != 'human':
      with closing(outfile):
        return outfile.getvalue()

  def reset(self):
    rustLib.hex_reset(self.game)
    self.rewards = []
    print("RESET IS DONE NOW IS", hex_to_array(self.game))
    return hex_to_array(self.game)