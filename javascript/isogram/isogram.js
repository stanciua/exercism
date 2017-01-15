var Immutable = require("Immutable")

var Isogram = function(word) {
  this.word = word;
}

Isogram.prototype.isIsogram = function() {
  return Immutable.Seq.of(...this.word.toLowerCase()).reduce((r, c) =>
    r.update(c, 0, value => value + 1), Immutable.Map()
  ).filter((v, k) => k !== " " && k !== "-").every((v, k) => v === 1)
}

module.exports = Isogram