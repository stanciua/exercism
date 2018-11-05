class Twofer {
    String twofer(String name) {
        StringBuilder output = new StringBuilder("One for ");
        if (name == null) {
            output.append("you");
        } else {
            output.append(name);
        }
        output.append(", one for me.");

        return output.toString();
    }
}
