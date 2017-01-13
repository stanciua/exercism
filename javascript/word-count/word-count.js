var _ = require("lodash");
var XRegExp = require("xregexp");

var Words = function() {};

Words.prototype.count = function(phrase) {
  function unquoteWord(word) {
    if (word.startsWith("'") && word.endsWith("'")) {
      return word.substr(1, word.length - 2);
    }
    return word;
  }

  let results = {};
  let allowedChars = _
    .filter(
      phrase.toLowerCase(),
      ch => ch.search(XRegExp("\\pL|\\pN|,|\\s+|'")) !== -1
    )
    .join("");
  for (let word of _.filter(allowedChars.split(/\s+|,/), word => word.length != 0)) {
    word = unquoteWord(word).trim();
    let val = results[word];
    if (val === undefined || typeof val !== "number") {
      results[word] = 1;
    } else {
      results[word] = val + 1;
    }
  }
  return results;
};

module.exports = Words;
