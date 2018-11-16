import java.lang.reflect.Array;
import java.util.Arrays;
import java.util.Map;

class Proverb {
    private final class Pair {
        public final int p1;
        public final int p2;

        Pair(int p1, int p2) {
            this.p1 = p1;
            this.p2 = p2;
        }
    }

    String[] words;

    Proverb(String[] words) {
        this.words = words;
    }

    Pair[] getListOfPairs(boolean isModernProverb) {
        Pair[] pairs;
        if (isModernProverb) {
            pairs = new Pair[]{new Pair(0,1), new Pair(1,4), new Pair(4,5), new Pair(0,0)};
        } else {
            pairs = new Pair[]{new Pair(0, 1), new Pair(1, 2), new Pair(2, 3), new Pair(3, 4), new Pair(4, 5), new Pair(5, 6), new Pair(0, 0)};
        }

        return pairs;
    }
    String recite() {
        if (this.words.length == 0) {
            return "";
        }


        int[] numericWords = Arrays.stream(words).mapToInt(w -> wordToInt(w)).toArray();
        Pair[] pairs = getListOfPairs(Arrays.stream(this.words).anyMatch(e -> e == "pin"));
        String proverb = Arrays.stream(pairs).collect(StringBuilder::new, (output, p) -> {
            boolean p1Present = Arrays.stream(numericWords).anyMatch(e -> p.p1 == e);
            boolean p2Present = Arrays.stream(numericWords).anyMatch(e -> p.p2 == e);
            if (p1Present && p2Present) {
                if (p.p1 == 0 && p.p2 == 0) {
                    output.append("And all for the want of a ");
                    output.append(IntToWord(p.p1));
                    output.append(".");
                } else {
                    output.append("For want of a ");
                    output.append(IntToWord(p.p1));
                    output.append(" the ");
                    output.append(IntToWord(p.p2));
                    output.append(" was lost.\n");
                }
            }
        }, (x, y) -> {
        }).toString();


        return proverb;
    }

    private String IntToWord(int word) {
        boolean isModernProverb = Arrays.stream(this.words).anyMatch(e -> e == "pin");
        String wordString;
        switch (word) {
            case 0:
                wordString = isModernProverb ? "pin" : "nail";
                break;
            case 1:
                wordString = isModernProverb ? "gun" : "shoe";
                break;
            case 2:
                wordString = "horse";
                break;
            case 3:
                wordString = "rider";
                break;
            case 4:
                wordString = isModernProverb ? "soldier" : "message";
                break;
            case 5:
                wordString = "battle";
                break;
            case 6:
                wordString = "kingdom";
                break;
            default:
                throw new RuntimeException("Invalid numeric word received");
        }

        return wordString;
    }

    private int wordToInt(String word) {
        int code = 0;
        switch (word) {
            case "nail":
            case "pin":
                code = 0;
                break;
            case "shoe":
            case "gun":
                code = 1;
                break;
            case "horse":
                code = 2;
                break;
            case "rider":
                code = 3;
                break;
            case "message":
            case "soldier":
                code = 4;
                break;
            case "battle":
                code = 5;
                break;
            case "kingdom":
                code = 6;
                break;
            default:
                throw new RuntimeException("Invalid word received");
        }

        return code;
    }
}
