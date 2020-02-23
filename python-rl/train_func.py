import gym
import sys
from stable_baselines.common.policies import MlpPolicy
from stable_baselines import PPO2
from stable_baselines.common import set_global_seeds
from stable_baselines.common.vec_env import DummyVecEnv,VecCheckNan
import numpy as np
from os import listdir
from os.path import isfile, join
import json

avg_rewards = []
def do_train(model, modeldir, env):
  global avg_rewards
  begin = 0
  step_size = int(2e4)
  dirname = "/local/s2084740/models/"
  logdirname = "boardlog/"
  file_step = "20k-"
  for f in listdir(dirname+modeldir):
    if f.startswith(file_step):
      end =  f[len(file_step):]
      num = int(end[:len(end)-4])
      if num > begin:
        begin = num
  print("STARTING TO LEARN ", modeldir, begin)
  def learn_callback(a, b):
    obs_len = len(a["observations"])
    if obs_len > 6:
      last_observations = a["observations"][obs_len-6:obs_len]
      all_equal = True
      for i in range(0, len(last_observations)-1):
        if last_observations[i] != last_observations[min(i+1, obs_len-1)]:
          all_equal = False
      if all_equal:
        print("ABORTING DUE TO LOCAL OPTIMUM / REPEATING STEPS")
      return not all_equal
    return True
  def reward_callback(loc, glob):
    global avg_rewards
    arr = loc["masks"]
    ends = np.where(arr == True)
    ends_to_use = ends[0]
    i = 0
    start = 0
    for end in ends_to_use:
      rewards = loc["true_reward"][start:end-1]
      avg_rewards.append(np.average(rewards))
      start = end
  if begin > 0:
    print("LOADING FILE", dirname+modeldir+"/"+file_step+str(begin))
    model.load(dirname+modeldir+"/"+file_step+str(begin)+".pkl",env=env)
  while True:
    env.reset()
    model.learn(total_timesteps=step_size,log_interval=500,callback=reward_callback)
    begin += 1
    # Save the agent
    model.save(dirname+modeldir+"/"+file_step+str(begin))
    with open(dirname+modeldir+"/"+str(begin)+"-rewards.json", "a") as myfile:
        myfile.write(json.dumps(avg_rewards, cls=NumpyEncoder))
    avg_rewards = []
    print("saved learnstep", begin, "total iterations are now", begin * step_size)

def inc_with_max(current, max_num):
  next_num = current + 1
  if next_num >= max_num:
    next_num = 0
  return next_num

def do_train_multi(models, modeldir, env):
  global avg_rewards
  model_amount = len(models)
  begin = np.zeros(model_amount, dtype=np.int)
  step_size = 20000
  dirname = "models/"
  logdirname = "boardlog/"
  file_step = "100k-"
  for i in range(0, model_amount):
    for f in listdir(dirname+modeldir):
      if f.startswith(file_step+"-"+str(i)+"-"):
        end =  f[len(file_step+"-"+str(i)+"-"):]
        num = int(end[:len(end)-4])
        if num > begin[i]:
          begin[i] = num
  
  def learn_callback(a, b):
    arr = a["masks"]
    ends = np.where(arr == True)
    ends_to_use = ends[0]
    if len(ends_to_use) > 3:
      ends_to_use = ends_to_use[len(ends_to_use)-3:len(ends_to_use)]
    last_observations = []
    for index in ends_to_use:
      last_observations.append(np.array(a["obs"][index-1]))
    obs_len = len(last_observations)
    all_equal = True
    for i in range(0, len(last_observations)-1):
      if not np.array_equal(last_observations[i],last_observations[i+1]):
        all_equal = False
    if all_equal:
      print("ABORTING DUE TO LOCAL OPTIMUM / REPEATING STEPS")
      with open("local_optima.txt", "a") as myfile:
          myfile.write("1")
    return not all_equal
  prev = []
  def reward_callback(loc, glob):
    global avg_rewards
    arr = loc["masks"]
    ends = np.where(arr == True)
    ends_to_use = ends[0]
    i = 0
    start = 0
    for end in ends_to_use:
      rewards = loc["true_reward"][start:end-1]
      avg_rewards.append(np.average(rewards))
      start = end
  print("STARTING TO LEARN ", modeldir, begin)
  for i in range(0, model_amount):
    if begin[i] > 0:
      models[i].load(dirname+modeldir+"/"+file_step+"-"+str(i)+"-"+str(begin[i]),env=env)
      print("FOR ", begin[i], "loading", dirname+modeldir+"/"+file_step+"-"+str(i)+"-"+str(begin[i]))
  model_index = 0
  opponent_index = 0
  prev_opponent = 1
  prev_model = 1
  while True:
    env.reset()
    while opponent_index == prev_opponent:
      opponent_index = np.random.randint(0, model_amount)
    prev_opponent = opponent_index
    # while opponent_index == model_index:
    #   opponent_index = np.random.randint(0, model_amount)
    models[model_index].opponent = models[opponent_index]
    print("ABOUT TO PLAY WITH ", model_index, opponent_index)
    models[model_index].learn(callback=reward_callback,total_timesteps=step_size,log_interval=500,tb_log_name="model"+str(model_index))
    models[model_index].opponent = None
    begin[model_index] += 1
    # print avg reward to file
    print("WRITING TO", dirname+modeldir+"/"+str(model_index)+"-all.json")
    with open(dirname+modeldir+"/"+str(model_index)+"-"+str(begin[model_index])+"all.json", "a") as myfile:
        myfile.write(json.dumps(avg_rewards, cls=NumpyEncoder))
    avg_rewards = []
    # Save the agent
    if begin[model_index] % 5 == 0:
      models[model_index].save(dirname+modeldir+"/"+file_step+"-"+str(model_index)+"-"+str(begin[model_index]))
      print("saved learnstep", begin[model_index], "total iterations are now", begin[model_index] * step_size)
    else:
      print("finished learnstep", begin[model_index], " not saving yet total iterations are now", begin[model_index] * step_size)
    while model_index == prev_model:
      model_index = np.random.randint(0, model_amount)
    prev_model = model_index

class NumpyEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        elif isinstance(obj, np.floating):
            return float(obj)
        elif isinstance(obj, np.ndarray):
            return obj.tolist()
        else:
            return super(NumpyEncoder, self).default(obj)

step_number = 0
reached = False
def train_until_localoptimum(make_model, seed, env,reward_type):
  global step_number
  global reached
  reached = False
  def learn_callback(a, b):
    global step_number
    global reached
    step_number = a["timestep"]
    if step_number > 10000:
      reached = False
      return False
    arr = a["masks"]
    ends = np.where(arr == True)
    ends_to_use = ends[0]
    if len(ends_to_use) > 3:
      ends_to_use = ends_to_use[len(ends_to_use)-3:len(ends_to_use)]
    last_observations = []
    for index in ends_to_use:
      last_observations.append(np.array(a["obs"][index-1]))
    obs_len = len(last_observations)
    all_equal = True
    for i in range(0, len(last_observations)-1):
      if not np.array_equal(last_observations[i],last_observations[i+1]):
        all_equal = False
    if all_equal:
      reached = True
    return not all_equal
  model_index = 0
  opponent_index = 0
  avg_reward = []
  for i in range(0, 3):
    env.reset()
    # while opponent_index == model_index:
    #   opponent_index = np.random.randint(0, model_amount)
    print("i in seed", i, seed)
    set_global_seeds(seed)
    model = make_model(env)
    model.opponent = model
    model.learn(total_timesteps=999999999,log_interval=99999,seed=seed,callback=learn_callback)
    # Save the agent
    reward = get_avg_reward_for_model(model, env, 10)
    print("LAST SCORE AFTER LEARN IS ", env.envs[0].last_score, reward)
    avg_reward.append([reward, step_number, env.envs[0].last_score, reached])
    model.save("models/self_seed/"+str(reward_type)+"-"+str(seed)+"-"+str(i))
    print("Evaluated seed at index", seed, i, reward_type, reached, step_number, avg_reward)
  with open("models/self_seed/reard-"+str(reward_type)+"-"+str(seed)+"-rewards.json", "w") as myfile:
      myfile.write(json.dumps(avg_reward, cls=NumpyEncoder))
    

def get_avg_reward_for_model(model, env, episodes):
  obs = env.reset()
  n_episodes = 0
  reward_sum = 0.0
  rewards = []
  while n_episodes < episodes:
      action, _ = model.predict(obs)
      obs, reward, done, _ = env.step(action)
      reward_sum += reward

      if done:
        rewards.append(reward_sum)
        reward_sum = 0.0
        n_episodes += 1
        obs = env.reset()
  return np.mean(rewards)