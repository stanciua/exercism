import java.util.*;
import java.util.stream.IntStream;

class Yacht {
    int[] dice;
    YachtCategory yachtCategory;
    Yacht(int[] dice, YachtCategory yachtCategory) {
        this.dice = dice;
        this.yachtCategory = yachtCategory;
    }

    int score() {
        int score = 0;
       switch(this.yachtCategory) {
           case BIG_STRAIGHT:
               int[] bigStraight = {2, 3, 4, 5, 6};

               if (Arrays.equals(bigStraight, IntStream.of(this.dice).sorted().toArray())) {
                   score = 30;
               }
               break;
           case LITTLE_STRAIGHT:
               int[] littleStraight = {1,2,3,4,5};
               if (Arrays.equals(littleStraight, IntStream.of(this.dice).sorted().toArray())) {
                   score = 30;
               }
               break;
           case YACHT:
               int element = this.dice[0];
               if (IntStream.of(this.dice).allMatch(e -> e == element)) {
                   score = 50;
               }
               break;
           case CHOICE:
               score = IntStream.of(this.dice).sum();
               break;
           case ONES:
               score = (int)IntStream.of(this.dice).filter(e -> e == 1).sum();
               break;
           case TWOS:
               score = (int)IntStream.of(this.dice).filter(e -> e == 2).sum();
               break;
           case THREES:
               score = (int)IntStream.of(this.dice).filter(e -> e == 3).sum();
               break;
           case FOURS:
               score = (int)IntStream.of(this.dice).filter(e -> e == 4).sum();
               break;
           case FIVES:
               score = (int)IntStream.of(this.dice).filter(e -> e == 5).sum();
               break;
           case SIXES:
               score = (int)IntStream.of(this.dice).filter(e -> e == 6).sum();
               break;
           case FULL_HOUSE:
               int[] fullHouseArray = IntStream
                       .rangeClosed(1, 6)
                       .map(e -> (int)IntStream.of(dice).filter(inner -> e == inner).count())
                       .filter(e -> e == 2 || e == 3).toArray();
               OptionalInt min = IntStream.of(fullHouseArray).min();
               OptionalInt max = IntStream.of(fullHouseArray).max();
               if (min.orElse(0) == 2 && max.orElse(0) == 3) {
                   score = IntStream.of(this.dice).sum();
               }
               break;
           case FOUR_OF_A_KIND:
               Map<Integer, Integer> fourOfAKindMap = IntStream
                       .rangeClosed(1, 6)
                       .collect(HashMap::new,
                                (h, e) -> h.put(e, (int)IntStream.of(dice).filter(inner -> e == inner).count()),
                                (h, e) -> {});

               score = fourOfAKindMap.entrySet().stream().filter(entry -> entry.getValue() >= 4).mapToInt(entry -> entry.getKey()).sum() * 4;
               break;
           default:
               throw new RuntimeException("Invalid game category.");
       }

       return score;
    }

}
