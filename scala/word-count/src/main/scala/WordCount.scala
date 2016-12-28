import scala.collection.mutable._

case class Phrase(val phrase: String) {
  def wordCount(): Map[String, Int] = {
    val allowedChars = ('0' to '9').toSet | ('a' to 'z').toSet + '\'' + ' ' + ','
    val counts: HashMap[String, Int] = HashMap.empty
    ((((phrase toLowerCase) filter (ch => allowedChars contains ch) toString) split ("""\s+|,""")) filter (!_.isEmpty))
      .foreach { word =>
        counts.get(word) match {
          case Some(v) => counts(word) = v + 1
          case None => counts(word) = 1
        }
      }
    counts
  }
}
