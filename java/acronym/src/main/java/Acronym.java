import java.util.Arrays;
import java.util.regex.Pattern;

class Acronym {
    String phrase;
    final static String pattern = "[\\s\\p{Punct}&&[^']]";
    Acronym(String phrase) {
        this.phrase = phrase.toUpperCase();
    }

    String getAcronym() {
        String[] atoms = this.phrase.split(pattern);
        StringBuilder acronym = new StringBuilder();
        for (String atom : atoms) {
            if (atom.isEmpty()) {
                continue;
            }
           acronym.append(atom.charAt(0));
        }

        return acronym.toString();
    }

}
