object Grains {
  def square(number: Int): Option[BigInt] = {
    number match {
      case n if n < 1 || n > 64 => None
      case n => Some(BigInt(2).pow(number - 1))
    }
  }

  def total(): BigInt = {
    (1 to 64)
      .map(
        square(_)
          .getOrElse[BigInt](0))
      .sum
  }

}
