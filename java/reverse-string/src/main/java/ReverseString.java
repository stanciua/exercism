class ReverseString {

    String reverse(String inputString) {
        if (inputString.length() == 0) {
            return inputString;
        }

        StringBuilder output = new StringBuilder();
        for (int i=inputString.length() - 1; i>=0; i--) {
            output.append(inputString.charAt(i));
        }

        return output.toString();
    }
  
}