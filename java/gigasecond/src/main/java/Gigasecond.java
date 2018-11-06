import java.time.LocalDate;
import java.time.LocalDateTime;

class Gigasecond {
    public final static long GIGASECOND = 1_000_000_000;
    LocalDateTime birthDateTime;
    Gigasecond(LocalDate birthDate) {
        this.birthDateTime = LocalDateTime.of(birthDate.getYear(), birthDate.getMonth(), birthDate.getDayOfMonth(), 0, 0);
    }

    Gigasecond(LocalDateTime birthDateTime) {
        this.birthDateTime = birthDateTime;
    }

    LocalDateTime getDateTime() {
        return this.birthDateTime.plusSeconds(GIGASECOND);
    }


}
