from scipy.stats import mannwhitneyu
import json

with open("score_first.json", 'r') as first:
  fileData = first.read()
  first_data = json.loads(fileData)
  with open("score_second.json", 'r') as second:
    fileData2 = second.read()
    second_data = json.loads(fileData2)
    res = mannwhitneyu(first_data, second_data)
    print("Wilcox score result", res)
    
with open("score_first_won.json", 'r') as first:
  fileData = first.read()
  first_data = json.loads(fileData)
  with open("score_second_won.json", 'r') as second:
    fileData2 = second.read()
    second_data = json.loads(fileData2)
    res = mannwhitneyu(first_data, second_data)
    print("Wilcox win result", res)

with open("score_diff_1.json", 'r') as first:
  fileData = first.read()
  first_data = json.loads(fileData)
  with open("score_diff_2.json", 'r') as second:
    fileData2 = second.read()
    second_data = json.loads(fileData2)
    res = mannwhitneyu(first_data, second_data)
    print("Wilcox scorediff result", res)
with open("score_sample_2.json", 'r') as first:
  fileData = first.read()
  first_data = json.loads(fileData)
  with open("score_sample_2.json", 'r') as second:
    fileData2 = second.read()
    second_data = json.loads(fileData2)
    res = mannwhitneyu(first_data, second_data)
    print("Wilcox scoresample result", res)