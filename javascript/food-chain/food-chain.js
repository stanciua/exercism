class FoodChain {
  constructor() {}

  verse(no) {
    const first = `I know an old lady who swallowed a`;
    const fly = `I don't know why she swallowed the fly. Perhaps she'll die.\n`
    const spider = `It wriggled and jiggled and tickled inside her.\n`
    const bird = `How absurd to swallow a bird!\n`
    const cat = `Imagine that, to swallow a cat!\n`
    const dog = `What a hog, to swallow a dog!\n`
    const goat = `Just opened her throat and swallowed a goat!\n`
    const cow = `I don't know how she swallowed a cow!\n`
    const horse = `She's dead, of course!\n`

    const animals = [
      ['fly', fly],
      ['spider', spider],
      ['bird', bird],
      ['cat', cat],
      ['dog', dog],
      ['goat', goat],
      ['cow', cow],
      ['horse', horse],
    ];

    let animal = animals[no - 1][0];
    let output = first + ` ${animal}.\n` + animals[no - 1][1];

    if (no != 1 && no != 8) {
      for (let i = no - 1; i >= 1; i--) {
        let currentAnimal = animals[i][0];
        let previousAnimal = animals[i - 1][0];
        let dotOrContinuation = currentAnimal == "bird" ? " that wriggled and jiggled and tickled inside her." : ".";
        const swallowed = `She swallowed the ${currentAnimal} to catch the ${previousAnimal}${dotOrContinuation}\n`
        if (i > 0) {
          output += swallowed;
        }
      }
    }

    if (no != 1 && no != 8) {
      return output + fly;
    }


    return output;
  }

  verses(from, to) {
    let output = "";
    for (let i = from; i <= to; i++) {
      output += this.verse(i) + '\n';
    }
    return output;
  }
}

module.exports = FoodChain