import java.time.LocalDate
import java.time.LocalTime
import java.time.LocalDateTime

object Gigasecond {
  private final val GIGASECOND = 1000000000
  def addGigaseconds(input: LocalDate): LocalDateTime = {
    LocalDateTime.of(input, LocalTime.of(0, 0)).plusSeconds(GIGASECOND)
  }
  def addGigaseconds(input: LocalDateTime): LocalDateTime = {
    input.plusSeconds(GIGASECOND)
  }

}
