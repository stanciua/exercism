class BeerSong {
    verse(number) {

        let output = ""
        let first = ""
        let second = ""

        if (number == 0) {
            first = "No more bottles"
            second = "Go to the store and buy some more, 99 bottles of beer on the wall."
            output += `${first} of beer on the wall, ${first[0].toLowerCase() + first.substr(1)} of beer.\n` +
                `${second}\n`
            return output
        } else if (number == 1) {
            first = "1 bottle"
            second = "Take it down and pass it around, no more bottles of beer on the wall."
            output += `${first} of beer on the wall, ${first} of beer.\n` +
                `${second}\n`

        } else if (number == 2) {
            first = `${number}`
            second = `${number - 1} bottle`
            output +=
                `${first} bottles of beer on the wall, ${first} bottles of beer.\n` +
                `Take one down and pass it around, ${second} of beer on the wall.\n`
        } else if (number <= 99) {
            first = `${number}`
            second = `${number - 1}`
            output += `${first} bottles of beer on the wall, ${first} bottles of beer.\n` +
                `Take one down and pass it around, ${second} bottles of beer on the wall.\n`

        } else {
            throw Error(`Invalid number of beers: ${number}`)
        }

        return output
    }

    sing(from, to) {
        let output = ""
        output = this.sing_impl(from, to == undefined ? 0 : to, output)
        return output
    }

    sing_impl(from, to, output) {
        if (from == to) {
            output += this.verse(from)
            return output
        }
        output += this.verse(from) + "\n"
        return this.sing_impl(from - 1, to, output)
    }
}
module.exports = BeerSong