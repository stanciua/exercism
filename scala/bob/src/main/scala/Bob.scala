class Bob {
  private def isAlpha(phrase: String) : Boolean = {
    val alphaChars = ('a' to 'z').toSet | ('A' to 'Z').toSet
    !(phrase.toSet & alphaChars).isEmpty
  }
  
  def hey(phrase: String) : String = {
    phrase match {
      case ph if ph == "" || (ph map (_ isWhitespace) forall (_ == true)) => "Fine. Be that way!"
      case ph if isAlpha(phrase) && ph.toUpperCase == ph => "Whoa, chill out!"
      case ph if ph.endsWith("?") => "Sure."
      case _ => "Whatever."
    }
  }
}
