class Bob {
  private def isAlpha(phrase: String) : Boolean = {
    val alphaChars = ('a' to 'z').toSet | ('A' to 'Z').toSet
    !(phrase.toSet & alphaChars).isEmpty
  }

  private def isYelling(phrase: String) : Boolean = phrase == "" || (phrase map (_ isWhitespace) forall (_ == true))  
  
  def hey(phrase: String) : String = {
    phrase match {
      case ph if isYelling(phrase) => "Fine. Be that way!"
      case ph if isAlpha(phrase) && ph.toUpperCase == ph => "Whoa, chill out!"
      case ph if ph.endsWith("?") => "Sure."
      case _ => "Whatever."
    }
  }
}
