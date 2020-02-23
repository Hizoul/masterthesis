from stable_baselines import A2C
from train_func import do_train
from env_getter import make_env

env = make_env()
model = A2C('MlpPolicy', env,verbose=1,n_steps=71,epsilon=2.36561309730191e-05,alpha=0.930340721692521,learning_rate=0.00223553342110284,max_grad_norm=0.50523672966627,ent_coef=0.00105748404479239,vf_coef=0.0492764482019269,gamma=0.953839228968649)

do_train(model, "a2cbox", env)