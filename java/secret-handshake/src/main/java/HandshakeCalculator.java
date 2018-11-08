import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class HandshakeCalculator {

    List<Signal> calculateHandshake(int number) {
       List<Signal> list = new ArrayList<>();

       if ((number & 1) != 0) {
           list.add(Signal.WINK);
       }

       if ((number & 2) != 0) {
           list.add(Signal.DOUBLE_BLINK);
       }

       if ((number & 4) != 0) {
           list.add(Signal.CLOSE_YOUR_EYES);
       }

       if ((number & 8) != 0) {
           list.add(Signal.JUMP);
       }

       final List<Signal> final_list = list;
        if ((number & 16) != 0) {
            list = IntStream.range(0, list.size())
                    .mapToObj(i -> final_list.get(final_list.size() - i + 0 - 1)).collect(Collectors.toList());
        }

        return list;
    }
}
