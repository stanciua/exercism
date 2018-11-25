import java.util.Arrays;
import java.util.stream.*;

class LargestSeriesProductCalculator {
    int[] numbers;
    LargestSeriesProductCalculator(String inputNumber) {
        numbers = inputNumber.chars().filter(c -> Character.isDigit(c)).map(c -> Character.getNumericValue(c)).toArray();
        if (numbers.length != inputNumber.length()) {
            throw new IllegalArgumentException("String to search may only contain digits.");
        }
    }

    long calculateLargestProductForSeriesLength(int numberOfDigits) {
        if (numbers.length < numberOfDigits) {
            throw new IllegalArgumentException("Series length must be less than or equal to the length of the string to search.");
        }
        if (numberOfDigits < 0) {
            throw new IllegalArgumentException("Series length must be non-negative.");
        }
        int maxProduct = 0;
        for (int i = 0; i + numberOfDigits - 1 < numbers.length; i++) {
           int currentProduct = IntStream.range(0 + i, numberOfDigits + i).map(idx -> numbers[idx]).reduce(1, (a, b) -> a * b);
           maxProduct = Integer.max(currentProduct, maxProduct);
        }

        return maxProduct;
    }
}
