import numpy as np
from stable_baselines.common.vec_env import DummyVecEnv
from stable_baselines import A2C
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym
#gamma=0.99, vf_coef=0.25, ent_coef=0.01, max_grad_norm=0.5, learning_rate=0.0007, alpha=0.99, epsilon=1e-05, lr_schedule='constant'
gym.register(id="rustyblocks-v0", entry_point="custom_env_heu:RustyBlocksEnv")
def optimize_a2c(trial):
    """ Learning hyperparamters we want to optimise"""
    return {
        'n_steps': trial.suggest_int('n_steps', 3, 100),
        'gamma': trial.suggest_loguniform('gamma', 0.8, 0.9999),
        'vf_coef': trial.suggest_loguniform('vf_coef', 0.01, 0.5),
        'ent_coef': trial.suggest_loguniform('ent_coef', 0.001, 0.07),
        'max_grad_norm': trial.suggest_loguniform('max_grad_norm', 0.3, 0.8),
        'learning_rate': trial.suggest_loguniform('learning_rate', 0.00007, 0.07),
        'alpha': trial.suggest_loguniform('alpha', 0.8, 0.9999),
        'epsilon': trial.suggest_loguniform('epsilon', 1e-06,1e-04),
        'lr_schedule': trial.suggest_categorical('lr_schedule', ['linear', 'constant', 'double_linear_con', 'middle_drop', 'double_middle_drop'])
    }


def optimize_agent(trial):
    """ Train the model and optimise
        Optuna maximises the negative log likelihood, so we
        need to negate the reward here
    """
    model_params = optimize_a2c(trial)
    seed = trial.suggest_int('numpyseed', 1, 429496729)
    np.random.seed(seed)
    original_env = gym.make('rustyblocks-v0')
    original_env.max_invalid_tries = 3
    env = DummyVecEnv([lambda: original_env])
    model = A2C("MlpPolicy", env, verbose=0, **model_params)
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
    study = optuna.create_study(study_name='a2c_heu', storage='sqlite:///a2c_heu.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=999, n_jobs=1)