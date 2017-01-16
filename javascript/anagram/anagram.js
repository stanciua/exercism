const Immutable = require("Immutable");

class Anagram {
    constructor(word) {
        this.word = word;
    }

    matches(words) {
        if (!Array.isArray(words)) {
            words = Immutable.List.of(...arguments).toJS()
        }
        return Immutable.List
            .of(...words)
            .filter(w => w.toLowerCase() != this.word.toLowerCase())
            .map(w => [w, this.charMap(w)])
            .filter(([w, m]) => m.equals(this.charMap(this.word)))
            .map(([w, m]) => w)
            .toJS();
    }
    charMap(word) {
        return Immutable.List
            .of(...word.toLowerCase())
            .reduce((r, c) => r.update(c, 0, value => value + 1), Immutable.Map());
    }
}

module.exports = Anagram;