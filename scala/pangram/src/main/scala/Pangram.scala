object Pangrams {
  def isPangram(text: String): Boolean = {
    val asciiSet = ('a' to 'z').toSet
    (asciiSet & (((text toLowerCase) filter (ch => asciiSet contains ch)).toSet)) == asciiSet
  }
}
