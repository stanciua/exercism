const Immutable = require("Immutable")

class PhoneNumber {
    constructor(phone) {
        this.phone = phone
    }

    number() {
        const numbers = Immutable.Set.of(...
            "0123456789");
        let number = Immutable.List.of(...this.phone).filter(ch => numbers.has(ch));
        const badNumber = "0000000000";
        if (number.size < 10 || number.size > 12) return badNumber;
        if (number.size === 11) {
            if (number.first() === "1") {
                number = number.rest()
            } else {
                return badNumber;
            }
        }
        return number.join("");
    }

    areaCode() {
        return this.number().substr(0, 3);
    }

    toString() {
        let n = this.number();
        return `(${this.areaCode()}) ${n.substr(3, 3)}-${n.substr(6, 4)}`
    }
}

module.exports = PhoneNumber