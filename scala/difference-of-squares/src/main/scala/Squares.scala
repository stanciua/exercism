object Squares {
  def squareOfSums(number: Int): Int = {
    math.pow((1 to number).sum, 2).toInt
  }

  def sumOfSquares(number: Int): Int = {
    (1 to number).map(n => math.pow(n, 2).toInt).sum
  }

  def difference(number: Int): Int =
    squareOfSums(number) - sumOfSquares(number)
}
