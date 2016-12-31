import scala.util._
import scala.collection.mutable._

class Robot {
  private var name_ =
    Robot.genRandName(2, 3)

  def name(): String = { name_ }

  def reset(): Unit = {
    name_ = Robot.genRandName(2, 3)
  }
}

object Robot {
  private val numericToAlpha: HashMap[Int, Char] = HashMap(
    (0 to 25).zip('A' to 'Z'): _*)
  private val r = new Random(System.currentTimeMillis)
  def genRandNameImpl(noAlphas: Int, noNumerics: Int): String = {
    val alphas = (for (n <- 0 until noAlphas)
      yield numericToAlpha(r.nextInt(26))).mkString
    val numerics = f"${r.nextInt(scala.math.pow(10, noNumerics).toInt)}%03d"
    alphas + numerics
  }

  def genRandName(noAlphas: Int, noNumerics: Int): String = {
    var randName = genRandNameImpl(noAlphas, noNumerics)
    while (cache contains randName) {
      randName = genRandNameImpl(noAlphas, noNumerics)
    }

    cache += randName
    randName
  }

  private val cache = HashSet.empty[String]

}
