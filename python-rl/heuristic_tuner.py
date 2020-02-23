import optuna
from rustyblocks import rustLib
import timeit

def optimize_agent(trial):
    connectability_weight = trial.suggest_uniform('connectability_weight', 0.0, 5)
    enemy_block_wheight = trial.suggest_uniform('enemy_block_wheight', 0.0, 5)
    touching_weight = trial.suggest_uniform('touching_weight', 0.0, 5)
    score_weight = trial.suggest_uniform('score_weight', 0.1, 25)
    start = timeit.timeit()#
    last_reward = rustLib.eval_heuristic_weights(connectability_weight, enemy_block_wheight, touching_weight, score_weight)
    end = timeit.timeit()
    print("Finding reward took", end, "reward is", last_reward)
    return last_reward

if __name__ == '__main__':
    study = optuna.create_study(study_name='heuristic_self', storage='sqlite:///optuna_parameterfinder_heuristic.db',load_if_exists=True,direction="maximize")
    study.optimize(optimize_agent, n_trials=100, n_jobs=1)