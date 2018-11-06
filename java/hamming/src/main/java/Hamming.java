class Hamming {

    String leftStrand;
    String rightStrand;
    Hamming(String leftStrand, String rightStrand) {
        if (leftStrand.length() != rightStrand.length()) {
            throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
        }

        this.leftStrand = leftStrand;
        this.rightStrand = rightStrand;
    }

    int getHammingDistance() {
        int counter = 0;
        int maxDifference = 0;
        for (int i=0; i<leftStrand.length(); i++) {
            if (leftStrand.charAt(i) != rightStrand.charAt(i)) {
                counter++;
            }
            if (counter > maxDifference) {
                maxDifference = counter;
            }
        }

        return maxDifference;
    }
}
