object Hamming {
  def compute(strand1: String, strand2: String): Option[Int] = {
    if (strand1.length != strand2.length) return None
    Some(strand1 zip strand2 filter ({ case (a, b) => a != b }) size)
  }
}
