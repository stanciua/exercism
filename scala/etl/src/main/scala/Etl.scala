import scala.collection.immutable.Map
import scala.collection.Seq
object ETL {
  def transform(data: Map[Int, Seq[String]]): Map[String, Int] = {
    Map(
      data
        .map { case (k, v) => v.zip(List.fill(v.length)(k)) }
        .toSeq
        .flatMap(x => x)
        .map(t => (t._1.toLowerCase, t._2)): _*)

  }
}
