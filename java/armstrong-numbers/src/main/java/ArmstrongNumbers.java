class ArmstrongNumbers {

	boolean isArmstrongNumber(int numberToCheck) {
	    String number = String.valueOf(numberToCheck);
	    int numberOfDigits = number.length();

	    if (numberOfDigits == 1) {
	    	return true;
		}

		int divider = (int)Math.pow(10, numberOfDigits - 1);
	    int reminder = numberToCheck;
	    int amstrongNumber = 0;
	    while (reminder != 0) {
	        int digit = reminder / divider;
	        reminder %= divider;
	        amstrongNumber += (int)Math.pow(digit, numberOfDigits);
	        divider /= 10;
        }

		if (amstrongNumber == numberToCheck) {
		    return true;
        }

        return false;
	}
}
