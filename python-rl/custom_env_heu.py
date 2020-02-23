import numpy as np
import sys
from gym import spaces, Env
from gym.utils import seeding
from rustyblocks import rustLib, field_to_string, field_to_array, field_to_cnn_array, save_img,field_score

LEFT = 0
RIGHT = 1
ROTATE_LEFT = 2
ROTATE_RIGHT = 3
CHANGE_STONE = 4
PLACE_STONE = 5
MAX_FOR_BLOCKS = 230

class RustyBlocksEnv(Env):
  metadata = {'render.modes': ['human', 'ansi']}
  def __init__(self, use_cnn=False,reward_type=0,self_play=False,save_score=False):
    self.use_cnn = use_cnn
    self.reward_type = reward_type
    self.action_space = spaces.Box(0.0, 15.0, shape=(4,), dtype=np.float) # block, rotation, x
    self.observation_space = spaces.Box(-126, 126, shape=(42,10), dtype=np.int8)
    self.self_play_is_second_player = False
    self.self_play = False
    self.field = rustLib.field_new()
    self.rewards = []
    self.seed()
    if use_cnn:
      self.observation_space = spaces.Box(0, 255, shape=(64,64,3), dtype=np.uint8)
    else:
      self.observation_space = spaces.Box(-126, 126, shape=(42,10), dtype=np.int8)

  def get_obs(self):
    if self.use_cnn:
      return field_to_cnn_array(self.field, 1 if self.self_play_is_second_player else 0)
    else:
      return field_to_array(self.field, 1 if self.self_play_is_second_player else 0)
  def seed(self, seed=None):
    return [seed]
  def step(self, actions):
    answer = rustLib.field_do_action_with_answer_heu(self.field, actions[0], actions[1], actions[2], actions[3], 2)
    placed = answer.placed
    reward = answer.reward
    new_observation = self.get_obs()
    done = answer.done == 0
    if self.use_cnn:
      new_observation = new_observation.repeat(2,axis=1).repeat(2,axis=0)
    if done:
      reward += 10.0 if answer.winner == 0 else -10.0
    return new_observation, reward, done, {}

  def render(self, mode='human'):
    outfile = StringIO() if mode == 'ansi' else sys.stdout
    outfile.write(self.get_obs())
    # No need to return anything for human
    if mode != 'human':
      with closing(outfile):
        return outfile.getvalue()

  def reset(self):
    self.prev_reward = 0
    rustLib.field_reset(self.field)
    new_observation = self.get_obs()
    if self.use_cnn:
      new_observation = new_observation.repeat(2,axis=1).repeat(2,axis=0)
    return new_observation