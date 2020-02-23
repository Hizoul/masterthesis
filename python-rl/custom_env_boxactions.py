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
    self.action_space = spaces.Box(0.0, 1.0, shape=(190,), dtype=np.float) # block, rotation, x
    self.observation_space = spaces.Box(-126, 126, shape=(42,10), dtype=np.int8)
    self.field = rustLib.field_new()
    self.rewards = []
    self.seed()
    self.wins = 0
    self.prev_reward = 0
    self.self_play = self_play
    self.self_play_is_second_player = False
    self.prev_obs = self.get_obs()
    self.last_score = field_score(self.field)
    self.save_score = save_score
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
    action = 0
    current_max = -1
    reward_modifier = 0
    actual_max = -1
    for i in range(0, len(actions)):
      action_probability = actions[i]
      action_possible = False
      if len(self.prev_obs) == 0:
        action_possible = True
      else:
        if self.use_cnn:
          action_possible = self.prev_obs[int((i+30) / 10)][(i % 10)+10][1] == 255
        else:
          newIndex = MAX_FOR_BLOCKS + i
          action_possible = (self.prev_obs[int(newIndex / 10)][newIndex % 10] == 1)
      if action_probability > actual_max:
        actual_max = action_probability
      if action_possible and action_probability > current_max:
        current_max = action_probability
        action = i
      elif not action_possible and action_probability > 0.3:
        reward_modifier -= 0.001
    if current_max <= 0.5:
      reward_modifier -= 0.003
    else:
      reward_modifier += 0.01
    scold_for_values_above_this = current_max - 0.1
    for i in range(0, len(actions)):
      action_probability = actions[i]
      if action_probability >= scold_for_values_above_this:
        reward_modifier -= 0.0004
    if actual_max == current_max:
      reward_modifier += 0.05
    else:
      reward_modifier -= 0.05
    if self.self_play:
      answer = rustLib.field_do_action_self_play(self.field, action, 1 if self.self_play_is_second_player else 0)
    else:
      answer = rustLib.field_do_action_with_answer(self.field, action, 2)
    placed = answer.placed
    if placed != 0:
      print("ERROR PLACING!", answer, action, actions)
      sys.exit(-1)
      return self.prev_obs, 0, True, {}
    new_observation = self.get_obs()
    reward = answer.reward + reward_modifier
    done = answer.done == 0
    self.prev_obs = new_observation
    if self.use_cnn:
      new_observation = new_observation.repeat(2,axis=1).repeat(2,axis=0)
    if done:
      if self.save_score:
        self.last_score = field_score(self.field)
      reward += 10.0 if answer.winner == 0 else -10.0
      if answer.winner == 0:
        self.wins += 1
    if self.reward_type == 1:
      reward = 0 
      if done:
        reward = 1.0 if answer.winner == 0 else -1.0
    elif self.reward_type == 2:
      reward = answer.reward - self.prev_reward
      if done:
        reward += 10.0 if answer.winner == 0 else -10.0
      self.prev_reward = answer.reward
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