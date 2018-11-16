import java.util.Arrays;
import java.util.stream.IntStream;

class IsbnVerifier {
    private static boolean isValidChar(char c) {
        return  Character.isDigit(c) || c == 'x' || c == 'X';
    }
    boolean isValid(String stringToVerify) {
        // If other characters different than 0..9 and X, reject the ISBN
        if (!IntStream.range(0, stringToVerify.length())
                .mapToObj(i -> stringToVerify.charAt(i))
                .allMatch(c -> IsbnVerifier.isValidChar(c) || c == '-')) {
            return false;
        }

        int[] array = IntStream.range(0, stringToVerify.length()).mapToObj(i -> stringToVerify.charAt(i)).filter(c -> IsbnVerifier.isValidChar(c)).mapToInt(c -> {
            if (c == 'x' || c == 'X') {
                return 10;
            }
            return Character.getNumericValue(c);
        }).toArray();

        // If X is present and is not the last character, reject the ISBN
        if (array.length == 10 && !IntStream.range(0, array.length - 1).map(i -> array[i]).allMatch(e -> e != 10)) {
           return false;
        }

        // valid lenghts are: 9 or 10 (with check character)
        if (array.length != 10 && array.length != 9) {
            return false;
        }

        return (IntStream.range(0, array.length).reduce(0, (acc, e) -> acc += array[e] * (array.length - e)) % 11 == 0);
    }

}
