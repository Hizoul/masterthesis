const files_to_read = ["heuristicamounts.json", "heuristicamounts_rand.json", "randomamounts.json"]

const data = []
const fs = require("fs")
const fn1 = () => {
  for (const fileName of files_to_read) {
    const fileContent = fs.readFileSync(fileName)
    const parsedContent = JSON.parse(fileContent)
    for (let turn = 0; turn < parsedContent.length; turn++) {
      let dataPoint = {
        symbol: fileName, turn, total: 0,
        minTwo: 0, minOne: 0, neutral: 0, plusOne: 0, plusTwo: 0, plusThree: 0, plusBig: 0
      }
      let amount = parsedContent[turn].length
      for (const turnPoints of parsedContent[turn]) {
        dataPoint.total += turnPoints.length
        for (const point of turnPoints) {
          if (point <= -2) {dataPoint.minTwo++}
          else if (point == -1) {dataPoint.minOne++}
          else if (point == 0) {dataPoint.neutral++}
          else if (point == 1) {dataPoint.plusOne++}
          else if (point == 2) {dataPoint.plusTwo++}
          else if (point == 3) {dataPoint.plusThree++}
          else if (point > 3) {dataPoint.plusBig++}
        }
      }
      if (dataPoint.total > 0) {
        dataPoint.total /= amount
      }
      if (dataPoint.minTwo > 0) {
        dataPoint.minTwo /= amount
      }
      if (dataPoint.minOne > 0) {
        dataPoint.minOne /= amount
      }
      if (dataPoint.neutral > 0) {
        dataPoint.neutral /= amount
      }
      if (dataPoint.plusOne > 0) {
        dataPoint.plusOne /= amount
      }
      if (dataPoint.plusTwo > 0) {
        dataPoint.plusTwo /= amount
      }
      if (dataPoint.plusThree > 0) {
        dataPoint.plusThree /= amount
      }
      if (dataPoint.plusBig > 0) {
        dataPoint.plusBig /= amount
      }
      if (dataPoint.total > 0) {
        data.push(dataPoint)
      }
    }
  }
  let datas = data
  let data2 = []
  for (const data of datas) {
    let turn = data.turn
    let symbol = "NONE"
    if (data.symbol == "heuristicamounts.json") {
      symbol = "H"
    } else if (data.symbol == "heuristicamounts_rand.json") {
      symbol = "H-R"
    } else if (data.symbol == "randomamounts.json") {
      symbol = "R"
    }
    data2.push({
      symbol, turn, point: data.minTwo, type: "Minus Two"
    })
    data2.push({
      symbol, turn, point: data.minOne, type: "Minus One"
    })
    data2.push({
      symbol, turn, point: data.neutral, type: "Neutral"
    })
    data2.push({
      symbol, turn, point: data.plusOne, type: "Plus One"
    })
    data2.push({
      symbol, turn, point: data.plusTwo, type: "Plus Two"
    })
    data2.push({
      symbol, turn, point: data.plusThree, type: "Plus Three"
    })
    data2.push({
      symbol, turn, point: data.plusBig, type: "Plus More"
    })
  }

  fs.writeFileSync("algo_points.json", JSON.stringify(data2))
}
const fn2 = () => {
  for (const fileName of files_to_read) {
    const fileContent = fs.readFileSync(fileName)
    const parsedContent = JSON.parse(fileContent)
    for (let turn = 0; turn < parsedContent.length; turn++) {
      for (const turnPoints of parsedContent[turn]) {
        for (const point of turnPoints) {
          let type = "NONE"
          if (point <= -2) {type = "Minus Two"}
          else if (point == -1) {type = "Minus One"}
          else if (point == 0) {type = "Neutral"}
          else if (point == 1) {type = "Plus One"}
          else if (point == 2) {type = "Plus Two"}
          else if (point == 3) {type = "Plus Three"}
          else if (point > 3) {type = "Plus More"}
          let symbol = "NONE"
          if (fileName == "heuristicamounts.json") {
            symbol = "H"
          } else if (fileName == "heuristicamounts_rand.json") {
            symbol = "H-R"
          } else if (fileName == "randomamounts.json") {
            symbol = "R"
          }
          data.push({
            symbol, turn, point, type
          })
        }
      }
    }
  }

  fs.writeFileSync("algo_points.json", JSON.stringify(data))
}

fn1()