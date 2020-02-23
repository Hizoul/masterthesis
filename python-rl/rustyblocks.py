#!/usr/bin/env python3
import math
import sys, ctypes
from os.path import abspath, dirname, realpath
from ctypes import c_char_p, c_uint32, c_uint8, c_int8, Structure, POINTER,c_void_p,c_int64,c_int16,c_double,c_uint64,c_char_p
import numpy as np
from numpy.ctypeslib import ndpointer
from PIL import Image


parent_folder = dirname(realpath(__file__)) + "/"
class Tuple(Structure):
    _fields_ = [("placed", c_uint8),("reward", c_double),("done", c_uint8),("winner", c_uint8)]
    def __str__(self):
        return "({},{},{},{})".format(self.placed, self.reward,self.done,self.winner)

class GameFieldS(Structure):
    pass

# libpath = "../target/debug/"
libpath = "../target/release/"
libname = "rustyblocks"
prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
absolute_lib_path = abspath(parent_folder + libpath + prefix + libname + extension)
print("Loading rustyblocks from ", absolute_lib_path)
rustLib = ctypes.cdll.LoadLibrary(absolute_lib_path)

rustLib.field_new.restype = POINTER(GameFieldS)
rustLib.field_free.argtypes = (POINTER(GameFieldS), )
rustLib.hex_new.restype = POINTER(GameFieldS)
rustLib.field_clone.argtypes = (POINTER(GameFieldS), )
rustLib.field_clone.restype = POINTER(GameFieldS)
rustLib.hex_free.argtypes = (POINTER(GameFieldS), )
rustLib.hex_do_action_with_answer.argtypes = (POINTER(GameFieldS), c_uint8, c_uint8)
rustLib.hex_do_action_with_answer.restype = Tuple
rustLib.hex_to_array.restype = ndpointer(dtype=ctypes.c_uint8, shape=(121,))
rustLib.free_hex_array.argtypes = (ndpointer(dtype=ctypes.c_uint8, shape=(121,)), )
rustLib.hex_reset.argtypes = (POINTER(GameFieldS), )
rustLib.field_to_string.argtypes = (POINTER(GameFieldS), )
rustLib.field_to_string.restype = c_void_p
rustLib.field_to_json_log.argtypes = (POINTER(GameFieldS), )
rustLib.field_to_json_log.restype = c_void_p
rustLib.field_restore_log.argtypes = (POINTER(GameFieldS), c_char_p)
rustLib.field_to_array.argtypes = (POINTER(GameFieldS), c_uint8)
rustLib.field_to_array.restype = ndpointer(dtype=c_int8, shape=(420,))
rustLib.free_field_array.argtypes = (ndpointer(dtype=c_int8, shape=(420,)), )
rustLib.field_get_score.argtypes = (POINTER(GameFieldS),)
rustLib.field_get_score.restype = ndpointer(dtype=c_int16, shape=(2,))
rustLib.free_score_array.argtypes = (ndpointer(dtype=c_int16, shape=(2,)), )
rustLib.field_do_action.argtypes = (POINTER(GameFieldS), c_uint8)
rustLib.field_do_action.restype = c_uint8
rustLib.field_do_action_with_answer.argtypes = (POINTER(GameFieldS), c_uint8, c_uint8)
rustLib.field_do_action_with_answer.restype = Tuple
rustLib.field_do_action_with_answer_heu.argtypes = (POINTER(GameFieldS), c_double,c_double,c_double,c_double, c_uint8)
rustLib.field_do_action_with_answer_heu.restype = Tuple
rustLib.field_do_action_self_play.argtypes = (POINTER(GameFieldS), c_uint8, c_int8)
rustLib.field_do_action_self_play.restype = Tuple
rustLib.field_is_game_over.argtypes = (POINTER(GameFieldS), )
rustLib.field_is_game_over.restype = c_uint8
rustLib.field_get_reward.argtypes = (POINTER(GameFieldS), )
rustLib.field_get_reward.restype = c_double
rustLib.field_counter_action.argtypes = (POINTER(GameFieldS), )
rustLib.field_counter_action.restype = c_int64
rustLib.field_counter_action_index.argtypes = (POINTER(GameFieldS), )
rustLib.field_counter_action_index.restype = c_uint8
rustLib.field_reset.argtypes = (POINTER(GameFieldS), )
rustLib.free_string.argtypes = (c_void_p, )
rustLib.eval_heuristic_weights.argtypes = (c_double,c_double,c_double,c_double)
rustLib.eval_heuristic_weights.restype = c_uint64

def field_to_string(field):
  ptr = rustLib.field_to_string(field)
  try:
    return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
  finally:
    rustLib.free_string(ptr)
def field_to_log(field):
  ptr = rustLib.field_to_json_log(field)
  try:
    return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
  finally:
    rustLib.free_string(ptr)

def field_to_array(field, self_play_is_second_player):
  original_array = rustLib.field_to_array(field, self_play_is_second_player)
  result = original_array.copy().reshape((42,10))
  rustLib.free_field_array(original_array)
  return result

def field_score(field):
  original_array = rustLib.field_get_score(field)
  result = original_array.copy()
  rustLib.free_score_array(original_array)
  return result

def field_to_cnn_array(field, self_play_is_second_player):
  original_array = rustLib.field_to_array(field, self_play_is_second_player)
  result = original_array.copy()
  newres = np.zeros(shape=(32,32,3), dtype=np.uint8)
  for i in range(0, 200):
    y = math.floor(i / 10)
    x = i % 10
    current = 0
    val = 0
    if result[i] == 1 or result[i] == 2:
      current = 0
      if result[i] == 1:
        val = 128
      else:
        val = 255
    elif result[i] == 3 or result[i] == 4:
      current = 2
      if result[i] == 3:
        val = 128
      else:
        val = 255
    newres[y][x][current] = val
  for i in range(200, 210):
    y = math.floor((i-200) / 10)
    x = (i % 10) + 10
    val = result[i] + 124
    newres[y][x][1] = val
  for i in range(210, 230):
    y = math.floor((i-200) / 10)
    x = (i % 10) + 10
    val = result[i]
    if val == -1:
      val = 0
    elif val > 0:
      val = val * 50
    newres[y][x][1] = val
  for i in range(230, 420):
    y = math.floor((i-200) / 10)
    x = (i % 10) + 10
    val = result[i]
    if val == -1:
      val = 0
    else:
      val = 255
    newres[y][x][1] = val
  rustLib.free_field_array(original_array)
  return newres

def hex_to_array(field):
  original_array = rustLib.hex_to_array(field)
  result = original_array.copy().reshape((11,11))
  rustLib.free_hex_array(original_array)
  return result

def field_string_to_array(field_string):
  field_array = np.zeros((20,10), dtype=np.uint8)
  row = 0
  col = 0
  for i in range(len(field_string)):
    entry = field_string[i]
    if entry == '\n':
      col = 0
      row += 1
    elif row < 20 and col < 10:
      field_array[row][col] = ord(entry) - 48
      col += 1
  return field_array

def save_img(data, name):
  Image.fromarray(data, 'RGB').save(name)