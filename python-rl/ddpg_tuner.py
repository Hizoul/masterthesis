import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import DDPG
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym

gym.register(id="rustyblocks-v0", entry_point="custom_env_boxactions:RustyBlocksEnv")
def optimize_ddpg(trial):
    """ Learning hyperparamters we want to optimise"""
    random_exploration = trial.suggest_loguniform('random_exploration', 0.000001, 0.1)
    if random_exploration < 0.001:
      random_exploration = 0.0
    return {
        'gamma': trial.suggest_loguniform('gamma', 0.8, 0.9999),
        'nb_train_steps': trial.suggest_int('nb_train_steps', 30, 200),
        'nb_rollout_steps': trial.suggest_int('nb_rollout_steps', 50, 500),
        'nb_eval_steps': trial.suggest_int('nb_eval_steps', 50, 500),
        'batch_size': trial.suggest_int('batch_size', 64, 512),
        'buffer_size': trial.suggest_int('buffer_size', 30000, 100000),
        'tau': trial.suggest_loguniform('tau', 0.0001, 0.08),
        'random_exploration': random_exploration,
        'critic_lr': trial.suggest_loguniform('critic_lr', 0.00001, 0.01),
        'reward_scale': trial.suggest_loguniform('reward_scale', 0.5, 3.0)
    }


def optimize_agent(trial):
    """ Train the model and optimise
        Optuna maximises the negative log likelihood, so we
        need to negate the reward here
    """
    model_params = optimize_ddpg(trial)
    seed = trial.suggest_int('numpyseed', 1, 429496729)
    np.random.seed(seed)
    original_env = gym.make('rustyblocks-v0')
    original_env.max_invalid_tries = 3
    env = DummyVecEnv([lambda: original_env])
    model = DDPG("MlpPolicy", env, verbose=0, observation_range=(-126,126), **model_params)
    print("DOING LEARING a2c")
    original_env.force_progression = False
    model.learn(int(2e4*5), seed=seed)
    print("DONE LEARING a2c")
    original_env.max_invalid_tries = -1

    rewards = []
    n_episodes, reward_sum = 0, 0.0

    obs = env.reset()
    original_env.force_progression = True
    original_env.invalid_try_limit = 5000
    while n_episodes < 4:
        action, _ = model.predict(obs)
        obs, reward, done, _ = env.step(action)
        reward_sum += reward

        if done:
          rewards.append(reward_sum)
          reward_sum = 0.0
          n_episodes += 1
          obs = env.reset()

    last_reward = np.mean(rewards)
    trial.report(last_reward)

    return last_reward


if __name__ == '__main__':
    study = optuna.create_study(study_name='ddpg', storage='sqlite:///ddpg.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=999, n_jobs=1)