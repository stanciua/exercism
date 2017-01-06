import scala.collection.immutable._

case class Scrabble() {
  private val letterMap = HashMap((List('A', 'E', 'I', 'O', 'U', 'L', 'N', 'R',
    'S', 'T').zip(List.fill(10)(1)) ++
  List('D', 'G').zip(List.fill(2)(2)) ++
  List('B', 'C', 'M', 'P').zip(List.fill(4)(3)) ++
  List('F', 'H', 'V', 'W', 'Y').zip(List.fill(5)(4)) ++
  List('K').zip(List.fill(1)(5)) ++
  List('J', 'X').zip(List.fill(2)(8)) ++
  List('Q', 'Z').zip(List.fill(2)(10)): _*))

  def scoreLetter(ch: Char): Int = letterMap.get(ch.toUpper).getOrElse(0)

  def scoreWord(word: String): Int = word.map(ch => scoreLetter(ch)).sum
}
