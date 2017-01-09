var R = require('ramda');

var Hamming = function(){
  function compute(strand1, strand2) {
    if (strand1.length !== strand2.length) throw Error("DNA strands must be of equal length.")
    return R.zip(strand1, strand2).filter(([a, b]) => a !== b).length
  }

  return {
    compute: compute
  }
}

module.exports = Hamming

