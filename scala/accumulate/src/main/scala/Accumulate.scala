case class Accumulate() {
  def accumulate[S, D](f: S => D, coll: List[S]): List[D] = {
    for (e <- coll) yield f(e)
  }
}
