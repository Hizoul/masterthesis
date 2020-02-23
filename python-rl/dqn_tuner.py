import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import DQN
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym

gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
def optimize_dqn(trial):
    """ Learning hyperparamters we want to optimise"""
    return {
        'gamma': trial.suggest_loguniform('gamma', 0.9, 0.9999),
        'learning_rate': trial.suggest_loguniform('learning_rate', 1e-5, 1.),
        'buffer_size': trial.suggest_int('buffer_size', 40, 100000),
        'batch_size': trial.suggest_int('batch_size', 8, 128),
        'exploration_fraction': trial.suggest_loguniform('exploration_fraction', 0.01, 0.5),
        'exploration_final_eps': trial.suggest_loguniform('exploration_final_eps', 0.01, 0.1)
    }


def optimize_agent(trial):
    """ Train the model and optimise
        Optuna maximises the negative log likelihood, so we
        need to negate the reward here
    """
    model_params = optimize_dqn(trial)
    seed = trial.suggest_int('numpyseed', 1, 429496729)
    np.random.seed(seed)
    original_env = gym.make('rustyblocks-v0')
    original_env.max_invalid_tries = 3
    env = DummyVecEnv([lambda: original_env])
    model = DQN("MlpPolicy", env, verbose=0, **model_params)
    print("DOING LEARING dqn")
    original_env.force_progression = False
    model.learn(int(2e4), seed=seed)
    print("DONE LEARING dqn")
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
    study = optuna.create_study(study_name='dqn', storage='sqlite:///dqn.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=999, n_jobs=1)