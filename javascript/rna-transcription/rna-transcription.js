var R = require('ramda')
var DnaTranscriber = function() {
  function dnaToRna(nucleotide) {
    switch (nucleotide) {
      case "G": return "C"
      case "C": return "G"
      case "T": return "A"
      case "A": return "U"
      default: throw Error("Invalid DNA nucleotide")
    }
  }
  function toRna(strand) {
    return R.map(dnaToRna, strand).join("")
  }
  return {
    toRna: toRna
  }
}

module.exports = DnaTranscriber

