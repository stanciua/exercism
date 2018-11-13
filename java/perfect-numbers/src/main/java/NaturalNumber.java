import java.util.stream.IntStream;

class NaturalNumber {
    int number;

    NaturalNumber(int number) {
        if (number <= 0) {
            throw new IllegalArgumentException("You must supply a natural number (positive integer)");
        }
        this.number = number;
    }

    Classification getClassification() {
        int sum = IntStream.range(1, number).filter(n -> number % n == 0).sum();
        if (sum == number) {
            return Classification.PERFECT;
        } else if (sum > number) {
            return Classification.ABUNDANT;
        }
        return Classification.DEFICIENT;
    }
}
