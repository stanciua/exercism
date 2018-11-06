import java.awt.*;

public class PangramChecker {
    public final static String ALPHABET = "abcdefghijklmnopqrstuvwxyz".toUpperCase();
    public final static int ALPHABET_LENGTH = ALPHABET.length();

    public boolean isPangram(String input) {
        if (input.isEmpty()) {
            return false;
        }

        int counter = 0;
        String inputUppercased = input.toUpperCase();
        for (int i=0; i<PangramChecker.ALPHABET_LENGTH; i++) {

            if (inputUppercased.contains(String.valueOf(ALPHABET.charAt(i)))) {
                counter++;
            }
        }

        return (counter == ALPHABET_LENGTH);
    }



}
