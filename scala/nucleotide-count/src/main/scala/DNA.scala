import scala.collection.mutable._

class DNA(dna: String) {

  def count(char: Char): Either[String, Int] = {
    val nucleotides = Set('A', 'C', 'G', 'T')

    if (dna.isEmpty) {
      return Right(0)
    }

    if (!nucleotides.contains(char)) {
      return Left("invalid nucleotide 'X'")
    }

    if (!(dna.toSet &~ nucleotides).isEmpty) {
      return Left("invalid nucleotide 'Y'")
    }

    Right(dna.filter(_ == char).size)
  }

  def nucleotideCounts: Either[String, Map[Char, Int]] = {
    val m: HashMap[Char, Int] = HashMap('A' -> 0, 'C' -> 0, 'G' -> 0, 'T' -> 0)

    dna.foreach(ch => {
      count(ch) match {
        case Left(err) => return Left(err)
        case Right(cnt) => m(ch) = cnt
      }
    })

    Right(m)
  }
}
