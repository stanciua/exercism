import java.util.stream.LongStream;

class LargestSeriesProductCalculator {
    private String input;
    LargestSeriesProductCalculator(String inputNumber) {
        if (inputNumber.chars().filter(c -> !Character.isDigit(c)).count() != 0) {
            throw new IllegalArgumentException("String to search may only contain digits.");
        }
        input = inputNumber;
    }

    long calculateLargestProductForSeriesLength(int numberOfDigits) {
        long maxProduct = 0;
        if (this.input.length() < numberOfDigits) {
            throw new IllegalArgumentException("Series length must be less than or equal to the length of the string to search.");
        }
        if (this.input.length() == 0) {
            return 1;
        }
        if (numberOfDigits < 0) {
            throw new IllegalArgumentException("Series length must be non-negative.");
        }
        long[] digits = this.input.chars().mapToLong(Character::getNumericValue).toArray();
        for (int i = 0; i < digits.length; i++) {
            if (i + numberOfDigits > digits.length) {
                break;
            }
            long product = LongStream.range(i, i + numberOfDigits).reduce(1, (acc, e) -> acc *= digits[(int)e]);
            if (product > maxProduct) {
                maxProduct = product;
            }
        }

        return maxProduct;
    }
}
