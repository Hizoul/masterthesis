import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import PPO2
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym
import time
gym.register(id="rustyblocks-v0", entry_point="custom_env_heu:RustyBlocksEnv")
def optimize_ppo2(trial):
    """ Learning hyperparamters we want to optimise"""
    return {
        'n_steps': trial.suggest_int('n_steps', 32, 5000),
        'gamma': trial.suggest_loguniform('gamma', 0.6, 0.9999),
        'vf_coef': trial.suggest_loguniform('vf_coef', 0.1, 1),
        'max_grad_norm': trial.suggest_loguniform('max_grad_norm', 0.1, 1),
        'learning_rate': trial.suggest_loguniform('learning_rate', 0.000005, 0.05),
        'ent_coef': trial.suggest_loguniform('ent_coef', 1e-20, 0.01),
        'cliprange': trial.suggest_uniform('cliprange', 0.05, 1.9),
        'noptepochs': trial.suggest_int('noptepochs', 1, 48),
        'lam': trial.suggest_uniform('lam', 0.6, 1.2)
    }

def ppo_study(use_cnn=False,simple_reward=False):
  def optimize_agent(trial):
      """ Train the model and optimise
          Optuna maximises the negative log likelihood, so we
          need to negate the reward here
      """
      model_params = optimize_ppo2(trial)
      seed = trial.suggest_int('numpyseed', 1, 2147483647)
      np.random.seed(seed)
      original_env = gym.make('rustyblocks-v0',use_cnn=use_cnn,simple_reward=simple_reward)
      env = DummyVecEnv([lambda: original_env])
      policy = "CnnPolicy" if use_cnn else "MlpPolicy"
      policy_kwargs = dict(net_arch=[dict(pi=[128, 128, 128],vf=[128, 128, 128])])
      model = PPO2(policy, env, verbose=0, nminibatches=1, policy_kwargs=policy_kwargs,**model_params)
      print("DOING LEARING ppo2")
      has_nan = False
      def learn_callback(a, b):
        has_nan = np.isnan(a["actions"]).any()
        return not has_nan
      model.learn(int(2e4*5), seed=seed, callback=learn_callback)
      print("DONE LEARING ppo2, wins gotten ", original_env.wins)
      if has_nan:
        trial.report(-15.0)
        return -15.0

      rewards = []
      n_episodes, reward_sum = 0, 0.0

      obs = env.reset()
      original_env.wins = 0
      start = time.time()
      while n_episodes < 1000:
          action, _ = model.predict(obs)
          obs, reward, done, _ = env.step(action)
          reward_sum += reward
          if done:
            rewards.append(reward_sum)
            reward_sum = 0.0
            n_episodes += 1
            obs = env.reset()

      end = time.time()
      last_reward = np.mean(rewards)
      trial.report(last_reward)
      print("done testing parameters average reward and wins and time_elapsed are:", last_reward, original_env.wins, end - start)
      return last_reward
  name = "ppo2_cnn-" + str(use_cnn) + "_simple-" + str(simple_reward)
  dbpath = 'sqlite:///' + name + '.db'
  study = optuna.create_study(study_name=name, storage=dbpath, load_if_exists=True,direction="maximize")
  study.optimize(optimize_agent, n_trials=999, n_jobs=1)

if __name__ == '__main__':
  ppo_study()