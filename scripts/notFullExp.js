const fs = require("fs")
const baseDir = "python-rl/models/self_seed"
const fileNames = fs.readdirSync(baseDir)
const notComplete = []
for (const fileName of fileNames) {
  if (fileName.startsWith("reard-0")) {
    const end = fileName.indexOf("-rewards.json")
    const seed = fileName.substring("reard-0-".length, end)
    if (!(fs.existsSync(`${baseDir}/reard-1-${seed}-rewards.json`) && fs.existsSync(`${baseDir}/reard-2-${seed}-rewards.json`))) {
      notComplete.push(seed)
    }
  }
}
console.log("incopmlete runs are ", notComplete)