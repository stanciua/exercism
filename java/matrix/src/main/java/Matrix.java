import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

class Matrix {
    private int[][] matrix;

    Matrix(String matrixAsString) {
        String[] lines = matrixAsString.split(System.getProperty("line.separator"));
        matrix = new int[lines.length][];
        for (int i = 0; i < lines.length; i++) {
            int[] row = Arrays.stream(lines[i].split(" ")).mapToInt(ein -> Integer.parseInt(ein)).toArray();
            matrix[i] = row;
        }
    }

    int[] getRow(int rowNumber) {
        return matrix[rowNumber];
    }

    int[] getColumn(int columnNumber) {
        return Arrays.stream(matrix).mapToInt(array -> array[columnNumber]).toArray();

    }
}
