class PhoneNumber(val num: String) {
  def number(): Option[String] = {
    val numParsed = num.filter(_.isDigit).toString
    numParsed.length match {
      case l if l < 10 || l > 11 => return None
      case l if l == 10 => return Some(numParsed)
      case l if l == 11 && numParsed.take(1) == "1" =>
        return Some(numParsed.drop(1))
      case _ => None

    }
  }

  def areaCode(): Option[String] = {
    this.number match {
      case Some(n) => Some(n.take(3))
      case None => None
    }
  }

  def prettyPrint(): Option[String] = {
    this.number match {
      case Some(n) =>
        Some(s"(${n.take(3)}) ${n.drop(3).take(3)}-${n.drop(6).take(4)}")
      case None => None
    }
  }

}
