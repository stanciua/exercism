import java.util.{GregorianCalendar, Calendar}
import java.util.TimeZone

case class Meetup(month: Int, year: Int) {
  private def advanceCalendarToDay(calendar: GregorianCalendar,
                                   toDay: Int,
                                   direction: Int): GregorianCalendar = {
    var dayOfWeek = calendar.get(Calendar.DAY_OF_WEEK)
    while (dayOfWeek != toDay) {
      calendar.add(Calendar.DAY_OF_MONTH, 1 * direction)
      dayOfWeek = calendar.get(Calendar.DAY_OF_WEEK)
    }

    calendar

  }
  def teenth(day: Int): GregorianCalendar = {
    // teenth days are from 13 till 19 of each month
    val calendar = new GregorianCalendar(year, month - 1, 13)
    advanceCalendarToDay(calendar, day, 1)
  }

  def first(day: Int): GregorianCalendar = {
    val calendar = new GregorianCalendar(year, month - 1, 1)
    advanceCalendarToDay(calendar, day, 1)
  }

  def second(day: Int): GregorianCalendar = {
    val calendar = first(day)
    calendar.add(Calendar.DAY_OF_MONTH, 1)
    advanceCalendarToDay(calendar, day, 1)
  }
  def third(day: Int): GregorianCalendar = {
    val calendar = second(day)
    calendar.add(Calendar.DAY_OF_MONTH, 1)
    advanceCalendarToDay(calendar, day, 1)
  }
  def fourth(day: Int): GregorianCalendar = {
    val calendar = third(day)
    calendar.add(Calendar.DAY_OF_MONTH, 1)
    advanceCalendarToDay(calendar, day, 1)
  }

  def last(day: Int): GregorianCalendar = {
    val calendar = new GregorianCalendar(year, month, 1)
    calendar.add(Calendar.DAY_OF_MONTH, -1)
    advanceCalendarToDay(calendar, day, -1)
  }
}

object Meetup {
  val Sun = 1
  val Mon = 2
  val Tue = 3
  val Wed = 4
  val Thu = 5
  val Fri = 6
  val Sat = 7
}
