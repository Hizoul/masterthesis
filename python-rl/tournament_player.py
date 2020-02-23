#!/usr/bin/python
# from BaseHTTPServer import BaseHTTPRequestHandler,HTTPServer
from http.server import BaseHTTPRequestHandler,HTTPServer
import json
from stable_baselines import PPO2
from env_getter import get_action_from_box_answer
import numpy as np
from os.path import join

PORT_NUMBER = 8080
basedir = "final_models"
def load(filename):
  return PPO2.load(join(basedir, filename))

players = [
  load("heu"),
  load("self-0"),
  load("self-2"),
  load("self-0-heu"),
  load("self-2-heu"),
  load("self-nobleed-2"),
  load("self-nobleed-5"),
  load("self-nobleed-2-heu"),
  load("self-nobleed-5-heu"),
]
class myHandler(BaseHTTPRequestHandler):
  def do_GET(self):
    print('Get request received')
    self.send_response(200)
    self.send_header('Content-type','text/html')
    self.end_headers()
    # Send the html message
    self.wfile.write(bytes("Hello World !", "UTF-8"))
    return
  def do_POST(self):
    content_length = int(self.headers['Content-Length'])
    post_data = self.rfile.read(content_length).decode("UTF-8")
    parsed = json.loads(post_data)
    algo_to_use = parsed["algo"]
    print("ALGO IS ", algo_to_use)
    answer = 0
    predictor = players[algo_to_use]
    obs = np.array(parsed["obs"]).reshape((42,10))
    if predictor != None:#
      algo_answer, states = predictor.predict(obs)
      answer = algo_answer if algo_to_use == 0 else get_action_from_box_answer(obs, algo_answer) 
      print("ANSWER IS", answer)
    self.send_response(200)
    self.send_header('Content-type','text/html')
    self.end_headers()
    # Send the html message
    to_send = json.dumps(answer.tolist()) if algo_to_use == 0 else answer
    print("SENT ANSWER IS", "{\"play\":"+str(to_send)+"}")
    self.wfile.write(bytes("{\"play\":"+str(to_send)+"}", "UTF-8"))
    return

server = HTTPServer(('', PORT_NUMBER), myHandler)
print('RL-Agent answerer listening on port ', PORT_NUMBER)
server.serve_forever()