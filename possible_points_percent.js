const fs = require("fs")
const d = JSON.parse(fs.readFileSync("all_possible_points_parallel_5.json"))
const depth = 4

let pointsAtDepth = d[depth]
let total = 0

for (const k of Object.keys(pointsAtDepth)) {
  total += pointsAtDepth[k]
}
const percentages = {}
let percent_negative = 0
let percent_non_optimal = 0
let percent_positive = 0
for (const k of Object.keys(pointsAtDepth)) {
  percentages[k] = pointsAtDepth[k] / total
  if (k.startsWith("minus_")) {
    percent_negative += percentages[k]
    percent_non_optimal += percentages[k]
  }
  if (k.startsWith("plus_")) {
    percent_positive += percentages[k]
  }
}
percent_non_optimal += percentages["neutral"]
console.log(`Postive: ${percent_positive}; Negative: ${percent_negative}; NonOptimal ${percent_non_optimal}; Total: ${total}`, percentages)
