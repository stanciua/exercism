import java.util.stream.IntStream;

class SumOfMultiples {
    private int number;
    private int[] set;
    SumOfMultiples(int number, int[] set) {
        this.number =  number;
        this.set = set;
    }

    int getSum() {
        return IntStream.range(1, number)
                .filter(n -> IntStream.of(this.set)
                        .filter(in -> in != 0 && n % in == 0)
                        .count() != 0)
                .sum();
    }

}
