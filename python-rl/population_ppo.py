import numpy as np
import os
from stable_baselines.common.vec_env import SubprocVecEnv, DummyVecEnv
from stable_baselines import PPO2
from stable_baselines.common.policies import MlpLnLstmPolicy
import gym
import json
from ray import tune

def loguniform(low=0, high=1, size=None):
    return np.exp(np.random.uniform(low, high, size))

class PPO2Trainer(tune.Trainable):
    def _setup(self, config):
        gym.register(id="rustyblocks-v0", entry_point="custom_env_boxactions:RustyBlocksEnv")
        original_env = gym.make('rustyblocks-v0')
        original_env.max_invalid_tries = 3
        original_env.force_progression = False
        self.env = DummyVecEnv([lambda: original_env])
        self.model = PPO2("MlpPolicy", self.env, verbose=0, nminibatches=1, **config)
        self.training_iteration = 0
        self.amount_of_iterations = 100

    def _train(self):
        print("DOING ONE TRAINING")
        self.training_iteration += 1
        self.env.reset()
        has_nan = False
        def learn_callback(a, b):
          has_nan = np.isnan(a["actions"]).any()
          return not has_nan
        self.model.learn(int(2e3), seed=seed, callback=learn_callback)
        print("DONE LEARING a2c, wins gotten:", original_env.wins)
        if has_nan:
          print("ERRORED WITH NAN")
          sys.exit(-1)

        rewards = []
        n_episodes, reward_sum = 0, 0.0

        obs = self.env.reset()
        while n_episodes < 100:
            action, _ = self.model.predict(obs)
            obs, reward, done, _ = self.env.step(action)
            reward_sum += reward
            if done:
                rewards.append(reward_sum)
                reward_sum = 0.0
                n_episodes += 1
                obs = self.env.reset()
        last_reward = np.mean(rewards)
        result_dict = {"episode_reward_mean": last_reward, "training_iteration": self.training_iteration, "done": self.training_iteration > self.amount_of_iterations}
        print("DONE WITH ONE TRAINING", self.training_iteration)
        return result_dict
    def _save(self, checkpoint_dir):
        print("save CHECKPOINT DIR IS", checkpoint_dir)
        path = os.path.join(checkpoint_dir, "checkpoint")
        self.model.save(os.path.join(checkpoint_dir, "model_weights"))
        with open(path, "w") as f:
            f.write(json.dumps({"training_iteration": self.training_iteration}))
        return checkpoint_dir

    def _restore(self, checkpoint_dir):
        print("load CHECKPOINT DIR IS", checkpoint_dir)
        self.env.reset()
        self.model.load(os.path.join(checkpoint_dir, "model_weights"), env=self.env)
        with open(os.path.join(checkpoint_dir, "checkpoint")) as f:
            self.training_iteration = json.loads(f.read())["training_iteration"]
    def reset_config(self, config):
        print("RESETTING CONFIG")
        self.model = PPO2("MlpPolicy", self.env, verbose=0, nminibatches=1, **config)
        self.env.reset()
        return True

if __name__ == '__main__':
    scheduler = tune.schedulers.PopulationBasedTraining(
        time_attr="training_iteration",
        metric="episode_reward_mean",
        mode="max",
        perturbation_interval=2,
        resample_probability=0.3,
        hyperparam_mutations={
        'n_steps': lambda: np.random.randint(32, 5000),
        'gamma': lambda: loguniform(0.6, 0.9999),
        'vf_coef': lambda: loguniform(0.1, 1.0),
        'max_grad_norm': lambda: loguniform(0.1, 1.0),
        'learning_rate': lambda: loguniform(0.000005, 0.05),
        'ent_coef': lambda: loguniform(1e-20, 0.01),
        'cliprange': lambda: np.random.uniform(0.05, 1.9),
        'noptepochs': lambda: np.random.randint(1, 48),
        'lam': lambda: np.random.uniform(0.6, 1.2)
    })

    analysis = tune.run(PPO2Trainer, config={
        'n_steps': tune.randint(16, 2048),
        'gamma': tune.loguniform(0.8, 0.9999),
        'vf_coef': tune.loguniform(0.1, 0.9),
        'max_grad_norm': tune.loguniform(0.1, 1.0),
        'learning_rate': tune.loguniform(0.00001, 0.05),
        'ent_coef': tune.loguniform(1e-8, 1e-1),
        'cliprange': tune.uniform(0.1, 1.5),
        'noptepochs': tune.randint(1, 48),
        'lam': tune.uniform(0.8, 1.2)
    }, resources_per_trial={"cpu": 6}, num_samples=15, reuse_actors=False,scheduler=scheduler)
    print("ANALYSIS IS", analysis.dataframe())
    print("Best config: ", analysis.get_best_config(metric="episode_reward_mean"))
