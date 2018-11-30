class RotationalCipher {
    private int shiftKey;
    private static final String lettersLowerCase = "abcdefghijklmnopqrstuvwxyz";
    private static final String lettersUpperCase = "ABCDERGHIJKLMNOPQRSTUVWXYZ";

    RotationalCipher(int shiftKey) {
        this.shiftKey = shiftKey;
    }

    String rotate(String data) {
        return data.chars().map(this::rotateLetter).collect(StringBuilder::new, StringBuilder::appendCodePoint, StringBuilder::append).toString();
    }

    private int rotateLetter(int c) {
        if (Character.isAlphabetic(c)) {
            String letters = Character.isLowerCase(c) ? RotationalCipher.lettersLowerCase: RotationalCipher.lettersUpperCase;
            int index = letters.indexOf(c);
            return letters.codePointAt((index + this.shiftKey) % letters.length());
        }
        return c;
    }

}
