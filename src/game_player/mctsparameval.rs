use crate::mcts::rewrite::{MCTSBot, MctsConfig};
use serde::{Deserialize, Serialize};
use crate::game_player::tournament::{Tournament, array_to_table};
use crate::ai::heuristic::HeuristicBot;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct MctsConfigNoTranspos {
  pub exploration_constant: f64,
  pub use_rave: bool,
  pub use_pool_rave: bool,
  pub rave_beta_param: f64, // beta - sim / beta = new node alpha,
  pub play_strategy: u8,
  pub lookahead_limit: usize,
  pub thought_time: u128
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ParamRes {
  pub avg_scores: (f64,f64),
  pub wins: u64,
  pub elos: (f64, f64),
  pub config: MctsConfigNoTranspos
}

pub fn eval_params() {
  let mut bots: Vec<MCTSBot> = Vec::new();
  let times = vec![1000];//vec![100, 500, 1000, 3000, 6000, 10000];
  let exploration_consts = vec![1.0];
  let lookahead_limits = vec![999];
  let use_rave = vec![true];
  let use_pool_rave = vec![false];
  let rave_beta = vec![100.0, 500.0, 1000.0, 2500.0, 5000.0];
  let strats = vec![0];
  for time in times {
    for expl in exploration_consts.iter() {
      for lok in lookahead_limits.iter() {
        for strat in strats.iter() {
          let sim_amount = if *strat == 0 {1} else {10};
          for ur in use_rave.iter() {
            if *ur {
              for upr in use_pool_rave.iter() {
                for rb in rave_beta.iter() {
                  let mut b = MCTSBot::new_with_time(time, 162, "m", 201);
                  b.config.lookahead_limit = *lok;
                  b.config.exploration_constant = *expl;
                  b.config.play_strategy = *strat;
                  b.config.use_rave = *ur;
                  b.config.use_pool_rave = *upr;
                  b.config.rave_beta_param = *rb;
                  b.config.simulation_amount = sim_amount;
                  bots.push(b);
                }
              }
            } else {
              let mut b = MCTSBot::new_with_time(time, 162, "m", 201);
              b.config.lookahead_limit = *lok;
              b.config.exploration_constant = *expl;
              b.config.play_strategy = *strat;
              b.config.simulation_amount = sim_amount;
              bots.push(b);
            }
          }
        }
      }
    }
  }
  println!("am {}", bots.len());
  let mut result: Vec<ParamRes> = Vec::new();
  let mut i =0;
  for bot in bots {
    let config = MctsConfigNoTranspos {
      exploration_constant: bot.config.exploration_constant,
      lookahead_limit: bot.config.lookahead_limit,
      play_strategy: bot.config.play_strategy,
      use_rave: bot.config.use_rave,
      use_pool_rave: bot.config.use_pool_rave,
      thought_time: bot.config.thought_time,
      rave_beta_param: bot.config.rave_beta_param
    };
    println!("PREFILLING");
    bot.prefill_rave(100);
    println!("DONE PREFILL");
    let h2 = HeuristicBot::new(0);
    let mut t = Tournament::new(vec![Box::new(bot), Box::new(h2)], 100);
    t.run(Option::None);
    t.persist(format!("tournament_{}.json",i));
    t.make_ratings();
    let res = ParamRes {
      wins: t.wins[0],
      elos: t.get_elo_tuple(),
      avg_scores: (t.average_player_score(0),t.average_player_score(1)),
      config
    };
    println!("RES IS {:?}", res);
    std::fs::write("mctsparams_next.json", serde_json::to_string(&result).unwrap()).unwrap();
    result.push(res);
    i += 1;
  }
  result.sort_by(|a, b| b.avg_scores.0.partial_cmp(&a.avg_scores.0).unwrap());
  println!("SORTED IS {:?}", result);
  for res in result {
    if res.avg_scores.0 > 0.0 {
      println!("RESULT WITH {:?}", res);
    }
  }
}
