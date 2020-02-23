import numpy as np
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import ACKTR
from stable_baselines.common.policies import MlpLnLstmPolicy
import optuna
import gym
#amma=0.99, nprocs=1, n_steps=20, ent_coef=0.01, vf_coef=0.25, vf_fisher_coef=1.0, learning_rate=0.25, max_grad_norm=0.5, kfac_clip=0.001, lr_schedule='linear',
gym.register(id="rustyblocks-v0", entry_point="custom_env:RustyBlocksEnv")
def optimize_acktr(trial):
    """ Learning hyperparamters we want to optimise"""
    return {
        'n_steps': trial.suggest_int('n_steps', 3, 100),
        'gamma': trial.suggest_loguniform('gamma', 0.8, 0.9999),
        'vf_coef': trial.suggest_loguniform('vf_coef', 0.1, 0.6),
        'ent_coef': trial.suggest_loguniform('ent_coef', 0.001, 0.07),
        'max_grad_norm': trial.suggest_loguniform('max_grad_norm', 0.1, 0.9),
        'learning_rate': trial.suggest_loguniform('learning_rate', 0.01, 0.5),
        'vf_fisher_coef': trial.suggest_loguniform('vf_fisher_coef', 0.8, 1.2),
        'kfac_clip': trial.suggest_loguniform('kfac_clip', 0.0001,0.01),
        'lr_schedule': trial.suggest_categorical('lr_schedule', ['linear', 'constant', 'double_linear_con', 'middle_drop', 'double_middle_drop'])
    }


def optimize_agent(trial):
    """ Train the model and optimise
        Optuna maximises the negative log likelihood, so we
        need to negate the reward here
    """
    model_params = optimize_acktr(trial)
    seed = trial.suggest_int('numpyseed', 1, 429496729)
    np.random.seed(seed)
    original_env = gym.make('rustyblocks-v0')
    original_env.max_invalid_tries = 3
    env = DummyVecEnv([lambda: original_env])
    model = ACKTR("MlpPolicy", env, nprocs=1,verbose=0, **model_params)
    print("DOING LEARING acer")
    original_env.force_progression = False
    model.learn(int(2e4), seed=seed)
    print("DONE LEARING acer")
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
    study = optuna.create_study(study_name='acktr', storage='sqlite:///acktr.db', load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=999, n_jobs=1)