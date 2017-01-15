var Immutable = require("Immutable")

class Pangram {
  constructor(phrase) {
    this.phrase = phrase
  }

  isPangram() {
    const alphabet = Immutable.Set.of(
      ...
      "abcdefghijklmnopqrstuvwxyz")

    return Immutable.Set.of(...this.phrase.toLowerCase()).filter(c => alphabet.has(c)).equals(alphabet)
  }
}

module.exports = Pangram
