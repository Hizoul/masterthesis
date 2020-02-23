import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import TRPO
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym
import math
#cg_damping=0.01, vf_stepsize=0.0003, vf_iters=3
gym.register(id="rustyblocks-v0", entry_point="custom_env_boxactions:RustyBlocksEnv")
def optimize_ddpg(trial):
    """ Learning hyperparamters we want to optimise"""
    timesteps_per_batch = int(math.pow(2, trial.suggest_loguniform('timesteps_per_batch', 9, 12)))
    entcoeff = trial.suggest_loguniform('entcoeff', 0.000001, 0.1)
    if entcoeff < 0.001:
      entcoeff = 0.0
    return {
        'gamma': trial.suggest_loguniform('gamma', 0.8, 0.9999),
        'timesteps_per_batch': timesteps_per_batch,
        'cg_iters': trial.suggest_int('cg_iters', 5, 50),
        'entcoeff': entcoeff,
        'max_kl': trial.suggest_loguniform('max_kl', 0.001, 0.2),
        'lam': trial.suggest_loguniform('lam', 0.8, 0.9999),
        'cg_damping': trial.suggest_loguniform('cg_damping', 0.001, 0.2),
        'vf_stepsize': trial.suggest_loguniform('vf_stepsize', 0.00003, 0.01),
        'vf_iters': trial.suggest_int('vf_iters', 2, 10),
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
    model = TRPO("MlpPolicy", env, verbose=0, **model_params)
    print("DOING LEARING trpo")
    original_env.force_progression = False
    model.learn(int(2e4*5), seed=seed)
    print("DONE LEARING trpo")
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
    study = optuna.create_study(study_name='trpo', storage='sqlite:///trpo.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=2, n_jobs=1)