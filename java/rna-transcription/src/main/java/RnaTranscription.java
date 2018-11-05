class RnaTranscription {

    String transcribe(String dnaStrand) {
        if (dnaStrand.length() == 0) {
            return dnaStrand;
        }

        StringBuilder rnaStrand = new StringBuilder();
        for (int i=0; i < dnaStrand.length(); i++) {
           rnaStrand.append(transcribeNucleotide(dnaStrand.charAt(i)));
        }
        return rnaStrand.toString();
    }

    char transcribeNucleotide(char nucleotide) {
        char transcribedNucleotide;
        switch(nucleotide) {
            case 'G':
                transcribedNucleotide = 'C';
                break;
            case 'C':
                transcribedNucleotide = 'G';
                break;
            case 'T':
                transcribedNucleotide = 'A';
                break;
            case 'A':
                transcribedNucleotide = 'U';
                break;
            default:
                transcribedNucleotide = ' ';
                throw new RuntimeException("Invalid nucleotide letter");

        }

        return transcribedNucleotide;
    }

}
