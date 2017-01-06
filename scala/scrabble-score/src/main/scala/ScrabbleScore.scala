import scala.collection.immutable._
// Letter                           Value
// A, E, I, O, U, L, N, R, S, T       1
// D, G                               2
// B, C, M, P                         3
// F, H, V, W, Y                      4
// K                                  5
// J, X                               8
// Q, Z                               10
case class Scrabble() {
  private val letterMap = HashMap((List('A', 'E', 'I', 'O', 'U', 'L', 'N', 'R',
      'S', 'T').zip(List.fill(10)(1))): _*) ++
      HashMap((List('D', 'G').zip(List.fill(2)(2))): _*) ++
      HashMap((List('B', 'C', 'M', 'P').zip(List.fill(4)(3))): _*) ++
      HashMap((List('F', 'H', 'V', 'W', 'Y').zip(List.fill(5)(4))): _*) ++
      HashMap((List('K').zip(List.fill(1)(5))): _*) ++
      HashMap((List('J', 'X').zip(List.fill(2)(8))): _*) ++
      HashMap((List('Q', 'Z').zip(List.fill(2)(10))): _*)

  def scoreLetter(ch: Char): Int = letterMap.get(ch.toUpper).getOrElse(0)

  def scoreWord(word: String): Int = word.map(ch => scoreLetter(ch)).sum
}
