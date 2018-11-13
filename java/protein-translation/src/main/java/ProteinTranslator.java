import java.util.ArrayList;
import java.util.List;

class ProteinTranslator {

    List<String> translate(String rnaSequence) {
        List<String> output = new ArrayList<>();
        StringBuilder remaningSequence = new StringBuilder(rnaSequence);
        while (remaningSequence.length() > 0) {
            String protein =getProteinFromCodons(remaningSequence.substring(0, 3));
            if (protein == "STOP") {
                break;
            }
            output.add(protein);
            remaningSequence = remaningSequence.delete(0, 3);
        }

        return output;
    }

    String getProteinFromCodons(String codon) {
        String output;
        switch (codon) {
            case "AUG":
                output = "Methionine";
                break;
            case "UUU":
            case "UUC":
                output = "Phenylalanine";
                break;
            case "UUA":
            case "UUG":
                output = "Leucine";
                break;
            case "UCU":
            case "UCC":
            case "UCA":
            case "UCG":
                output = "Serine";
                break;
            case "UAU":
            case "UAC":
                output = "Tyrosine";
                break;
            case "UGU":
            case "UGC":
                output = "Cysteine";
                break;
            case "UGG":
                output = "Tryptophan";
                break;
            case "UAA":
            case "UAG":
            case "UGA":
                output = "STOP";
                break;
            default:
                throw new RuntimeException("Invalid codon found.");
        }

        return output;
    }


}