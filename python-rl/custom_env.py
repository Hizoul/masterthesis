import numpy as np
import sys
from gym import spaces, Env
from gym.utils import seeding
from rustyblocks import rustLib, field_to_string, field_to_array

LEFT = 0
RIGHT = 1
ROTATE_LEFT = 2
ROTATE_RIGHT = 3
CHANGE_STONE = 4
PLACE_STONE = 5
MAX_FOR_BLOCKS = 230

INITIAL_OBS = [[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0],
[0,0,-1,-1,-1,-1,-1,-1,-1,-1],
[5,5,5,5,5,5,5,5,5,5],
[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,-1,1,-1,1,-1,1],
[1,1,1,1,1,1,1,1,1,-1],
[-1,1,-1,-1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,-1,-1,-1,1],
[1,1,-1,1,1,1,-1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,-1,1,1,1,-1,-1,1,-1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,-1,1,-1,1,-1,1],
[-1,1,-1,-1,-1,-1,-1,-1,-1,-1]]

class RustyBlocksEnv(Env):
  metadata = {'render.modes': ['human', 'ansi']}
  def __init__(self, spots=37):
    self.action_space = spaces.Discrete(190) # block, rotation, x
    self.observation_space = spaces.Box(-126, 126, shape=(42,10), dtype=np.int8)
    self.field = rustLib.field_new()
    self.invalid_tries = 0
    self.reward_finding_right = False
    self.rewards = []
    self.seed()
    self.force_progression = False
    self.invalid_try_limit = 1000
    self.amount_limitsurpass = 0
    self.max_invalid_tries = -1
    self.prev_obs = INITIAL_OBS

  def seed(self, seed=None):
    return [seed]

  def step(self, actionWanted):
    current_lowest_dist = 190
    action = 0
    reward_modifier = 0
    for i in range(0, 190):
      action_possible = False
      if len(self.prev_obs) == 0:
        action_possible = True
      else:
        newIndex = MAX_FOR_BLOCKS + i
        action_possible = self.prev_obs[int(newIndex / 10)][newIndex % 10] == 1
      action_probability = abs(actionWanted - i)
      if action_probability < current_lowest_dist and action_possible:
        current_lowest_dist = action_probability
        action = i
    if action == actionWanted:
      reward_modifier = 0.5 if action == actionWanted else -0.5
    if self.invalid_tries > self.invalid_try_limit:
      self.amount_limitsurpass += 1
      if self.max_invalid_tries != -1 and self.amount_limitsurpass >= self.max_invalid_tries:
        print("ABORTING LEARNING DUE TO TOO MANY WRONG TRIES", self.amount_limitsurpass)
        sys.exit(-1)
      print("ANOTHER 1k wrong tries", self.force_progression)
      self.invalid_tries = 0
      self.reward_finding_right = True
      if self.force_progression:
        print("Forcing game to progress")
        rustLib.field_counter_action(self.field, 1)
        is_over = rustLib.field_is_game_over(self.field) == 1
        if not is_over:
          rustLib.field_counter_action(self.field, 0)
        return field_to_array(self.field), -2.0, rustLib.field_is_game_over(self.field) == 1, {}
    answer = rustLib.field_do_action_with_answer(self.field, action, 2)
    placed = answer.placed
    reward = answer.reward
    done = answer.done == 0
    if placed == 0:
      self.invalid_tries = 0
      self.amount_limitsurpass = 0
      if self.reward_finding_right:
        self.reward_finding_right = False
        reward += 1
        print("Gave an extra bonus for finding the right combo after a lot of invalid tries", reward)
    else:
      self.invalid_tries += 1
      reward = -1
    if done:
      winner = answer.winner
      reward += 10.0 if winner == 0 else -10.0
      nprew = np.array(self.rewards)
      print("game is over average reward is", np.average(nprew), " median is", np.median(nprew), " high and low are ", np.min(nprew), np.max(nprew))
    new_observation = field_to_array(self.field)
    self.prev_obs = new_observation
    self.rewards.append(reward)
    return new_observation, reward, done, {}

  def render(self, mode='human'):
    outfile = StringIO() if mode == 'ansi' else sys.stdout
    outfile.write(field_to_array(self.field))
    # No need to return anything for human
    if mode != 'human':
      with closing(outfile):
        return outfile.getvalue()

  def reset(self):
    rustLib.field_reset(self.field)
    self.invalid_tries = 0
    self.reward_finding_right = False
    self.rewards = []
    self.amount_limitsurpass = 0
    return field_to_array(self.field)