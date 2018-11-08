import java.security.InvalidParameterException;

class Scrabble {
    String word;
    Scrabble(String word) {
        this.word = word.toUpperCase();
    }

    int getScore() {
        int score = 0;
        for (int i=0; i<this.word.length(); i++) {
            switch (this.word.charAt(i)) {
                case 'A':
                case 'E':
                case 'I':
                case 'O':
                case 'U':
                case 'L':
                case 'N':
                case 'R':
                case 'S':
                case 'T':
                    score += 1;
                    break;

                case 'D':
                case 'G':
                    score += 2;
                    break;
                case 'B':
                case 'C':
                case 'M':
                case 'P':
                    score += 3;
                    break;
                case 'F':
                case 'H':
                case 'V':
                case 'W':
                case 'Y':
                    score += 4;
                    break;
                case 'K':
                    score += 5;
                    break;
                case 'J':
                case 'X':
                    score += 8;
                    break;
                case 'Q':
                case 'Z':
                    score += 10;
                    break;
                default:
                    throw new RuntimeException("Invalid character used.");
            }
        }

        return score;
    }

}
