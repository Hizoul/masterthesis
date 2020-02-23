import numpy as np
import gym
from rustyblocks import rustLib, field_to_string, field_to_array
import os
import sys
gym.register(id="rustydiscrete-v0", entry_point="custom_env:RustyBlocksEnv")
gym.register(id="rustybox-v0", entry_point="custom_env_boxactions:RustyBlocksEnv")
boxEnv = gym.make("rustybox-v0")
def generate_pretraindata_heuristics(save_interval, amount_of_games, is_box_space):
  env = gym.make("rustydiscrete-v0")
  actions = []
  boxActions = []
  observations = []
  rewards = []
  episode_returns = []
  episode_starts = []
  lastFile = ""
  startAt = 0
  directory = "D:\\4-System\\rusty\\"
  for f in os.listdir(directory):
    if f.endswith("npz"):
      lastFile = f
  # if len(lastFile) > 0:
  #   print("LOADING FILE", directory+lastFile, os.listdir(directory))
  #   loadedData = np.load(directory+lastFile)
  #   actions = loadedData["actions"].tolist()
  #   observations = loadedData["obs"].tolist()
  #   rewards = loadedData["rewards"].tolist()
  #   episode_returns = loadedData["episode_returns"].tolist()
  #   episode_starts = loadedData["episode_starts"].tolist()
  #   startAt = int(lastFile.split("_")[0]) + 1
  #   print("LOADED DATA IS", lastFile, len(episode_returns), startAt)
  #   loadedData = None
  gameField = rustLib.field_new()
  print("Beginning Games")
  for i in range(startAt, amount_of_games):
    game_over = False
    reward_sum = 0
    while not game_over:
      action = rustLib.field_counter_action_index(gameField)
      answer = rustLib.field_do_action_with_answer(gameField, action, 2)
      if answer.placed != 0:
        print("Error a block couldn't be placed", action, answer)
        sys.exit(-1)
      actionArr = np.zeros(190, dtype=float)
      actionArr[action] = 1
      boxActions.append(actionArr)
      actions.append(action)
      observations.append(field_to_array(gameField))
      game_over = answer.done == 0
      reward = answer.reward
      episode_starts.append(game_over)
      if game_over:
        winner = answer.winner
        reward += 3.0 if winner == 0 else -3.0
      rewards.append(reward)
      reward_sum += reward
      if game_over:
        episode_returns.append(reward_sum)
        print("DONE WITH GAME NUMBER", i, reward_sum)
    rustLib.field_reset(gameField)
    if i > 0 and i % save_interval == 0:
      numpy_dict = {
        'actions': np.array(actions).reshape((-1, 1)),
        'obs': np.array(observations),
        'rewards': np.array(rewards),
        'episode_returns': np.array(episode_returns),
        'episode_starts': np.array(episode_starts[:-1])
      }
      
      np.savez(directory + str(i) + "_heutistic_pretrain_discrete.npz", **numpy_dict)

      numpy_dict = {
        'actions': np.concatenate(boxActions).reshape((-1,) + boxEnv.action_space.shape),
        'obs': np.array(observations),
        'rewards': np.array(rewards),
        'episode_returns': np.array(episode_returns),
        'episode_starts': np.array(episode_starts[:-1])
      }
      
      np.savez(directory + str(i) + "_heutistic_pretrain_box.npz", **numpy_dict)
      numpy_dict = None

 

  rustLib.field_free(gameField)

  numpy_dict = {
    'actions': np.concatenate(boxActions).reshape((-1,) + boxEnv.action_space.shape) if is_box_space else  np.array(actions).reshape((-1, 1)),
    'obs':  np.array(observations),
    'rewards': np.array(rewards),
    'episode_returns': np.array(episode_returns),
    'episode_starts': np.array(episode_starts[:-1])
  }
  return numpy_dict