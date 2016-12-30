import scala.collection.SortedMap
import scala.collection.mutable._

class School {
  var db: HashMap[Int, Seq[String]] = HashMap.empty

  def add(name: String, grade: Int): Unit = {
    db.get(grade) match {
      case Some(seq) => db(grade) = seq ++ Seq(name)
      case None => db(grade) = Seq(name)
    }
  }

  def grade(grd: Int): Seq[String] = {
    db.get(grd) match {
      case Some(seq) => seq
      case None => Seq.empty
    }
  }

  def sorted(): SortedMap[Int, Seq[String]] = {
    var sortedMap: SortedMap[Int, Seq[String]] = SortedMap.empty
    db.foreach {
      case (k, v) =>
        sortedMap = sortedMap + (k -> v.sorted)
    }
    sortedMap
  }
}
