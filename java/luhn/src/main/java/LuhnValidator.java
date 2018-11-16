import java.util.stream.IntStream;

class LuhnValidator {

    boolean isValid(String candidate) {

        String candidateNoSpaces = IntStream.range(0, candidate.length()).mapToObj(candidate::charAt).filter(c -> c != ' ').collect(StringBuilder::new, StringBuilder::append, (x, y) -> {
        }).toString();
        //  only digits or spaces are allowed
        if (!IntStream.range(0, candidateNoSpaces.length()).mapToObj(candidateNoSpaces::charAt).allMatch(Character::isDigit)) {
            return false;
        }

        int[] numbers = IntStream.range(0, candidateNoSpaces.length())
                .map(i -> Character.getNumericValue(candidateNoSpaces.charAt(i)))
                .toArray();

        // length shouldn't have a length smaller or equal to 1
        if (numbers.length <= 1) {
            return false;
        }

        return IntStream.range(0, numbers.length)
                .map(i -> numbers.length - i - 1)
                .map(i -> {
                    if ((numbers.length % 2 != 0 && i % 2 != 0) || (numbers.length % 2 == 0 && i == 0)) {
                        int num = numbers[i] * 2;
                        num = num > 9 ? num - 9 : num;
                        return num;
                    }
                    return numbers[i];
                }).sum() % 10 == 0;
    }

}
