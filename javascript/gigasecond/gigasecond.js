var Gigasecond = function(date) {
  this.d = date
  this.d.setUTCSeconds(this.d.getUTCSeconds() + 1000000000)
}

Gigasecond.prototype.date = function() {
  return this.d
}

module.exports = Gigasecond
