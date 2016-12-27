object SumOfMultiples {
  def sumOfMultiples(factors: Set[Int], limit: Int): Int = {
    if (factors.isEmpty) return 0
    val rangeOfNumbers = (factors min) until limit
    if (rangeOfNumbers.isEmpty) return 0

    (for (n <- rangeOfNumbers if factors exists (n % _ == 0))
      yield n) reduceLeft (_ + _)
  }
}
