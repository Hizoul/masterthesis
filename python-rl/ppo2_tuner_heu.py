import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import PPO2
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym

gym.register(id="rustyblocks-v0", entry_point="custom_env_heu:RustyBlocksEnv")
def optimize_ppo2(trial):
    """ Learning hyperparamters we want to optimise"""
    return {
        'n_steps': trial.suggest_int('n_steps', 16, 2048),
        'gamma': trial.suggest_loguniform('gamma', 0.8, 0.9999),
        'vf_coef': trial.suggest_loguniform('vf_coef', 0.1, 0.9),
        'vf_coef': trial.suggest_loguniform('max_grad_norm', 0.4, 1.6),
        'learning_rate': trial.suggest_loguniform('learning_rate', 0.00001, 0.05),
        'ent_coef': trial.suggest_loguniform('ent_coef', 1e-8, 1e-1),
        'cliprange': trial.suggest_uniform('cliprange', 0.1, 1.5),
        'noptepochs': trial.suggest_int('noptepochs', 1, 48),
        'lam': trial.suggest_uniform('lam', 0.8, 1.2),
        'noptepochs': trial.suggest_int('noptepochs', 2, 8)
    }


def optimize_agent(trial):
    """ Train the model and optimise
        Optuna maximises the negative log likelihood, so we
        need to negate the reward here
    """
    model_params = optimize_ppo2(trial)
    seed = trial.suggest_int('numpyseed', 1, 429496729)
    np.random.seed(seed)
    original_env = gym.make('rustyblocks-v0')
    original_env.max_invalid_tries = 3
    env = DummyVecEnv([lambda: original_env])
    model = PPO2("MlpPolicy", env, verbose=0, nminibatches=1, **model_params)
    print("DOING LEARING ppo2")
    original_env.force_progression = False
    model.learn(int(2e4*5), seed=seed)
    print("DONE LEARING ppo2")
    original_env.max_invalid_tries = -1

    rewards = []
    n_episodes, reward_sum = 0, 0.0

    obs = env.reset()
    original_env.force_progression = True
    original_env.invalid_try_limit = 5000
    while n_episodes < 500:
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
    study = optuna.create_study(study_name='ppo2_heu', storage='sqlite:///ppo2_heu.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=999, n_jobs=1)