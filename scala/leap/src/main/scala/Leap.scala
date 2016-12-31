class Year private (val year: Int) {
  def isLeap(): Boolean = {
    year match {
      case _ if year % 400 == 0 => true
      case _ if year % 100 == 0 => false
      case _ if year % 4 == 0 => true
      case _ => false
    }
  }
}

object Year {
  def apply(year: Int): Year = {
    new Year(year)
  }

}
