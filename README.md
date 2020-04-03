# Comparing AI approaches for Tetris Link
This repository contains the code used to produce the results of the experiments in my master thesis.

## Requirements / Used libraries & versions
- Rust 1.38
- Python 3
  - tensorflow 1.14
  - stable-baselines 2.6
  - gym 0.15

Below is an unordered table of experiments and their code.

|  Experiment | Code  |
|---|---|
| Branching Factor (3.2) | `src/plot/possible_plays/mod.rs`  |
| First Player Advantage (3.3) | `src/game_player/advantage_checker.rs` |
| Transposition Table Benchmark (4.1.5) | `benches/game_logic/transposition.rs` |
| Performance Rust (5.1.1) | `benches/game_logic/field.rs` |
| Human Play web interface (6.2.1) | [Web Version](https://github.com/Hizoul/contetro)  and [the experiment code part](https://github.com/Hizoul/contetro/blob/master/isofw-web/src/expertIndex.tsx) |
| Reproducibility (6.3) | `python-rl/repro.py` and `scripts/reprodocubilityDataProcessor.py` |
| Observation Space (6.4) | `python-rl/searchparams.py` |
| Reward Function Effectiveness (6.5) | `scripts/reprodocubilityDataProcessor.py` |
| MCTS Effectiveness (6.6) | `src/game_player/mctsparameval.rs` |
| RL Agents Training (6.7) | `python-rl/selfplay.rs` |
| Tournament (6.8) | `src/game_player/tournament.rs` |
| MCTS Verification (6.9) | `src/hex/hex_eval.rs` |
| Untested and unused AlphaZero Rust pytorch implementation | `src/rl/aznet.rs` |
