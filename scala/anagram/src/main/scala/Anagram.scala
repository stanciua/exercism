case class Anagram(val word: String) {
  def matches(words: Seq[String]): Seq[String] = {
    val wordLen = word.length
    val wordLowerCase = word.toLowerCase
    val charSeqSorted = wordLowerCase.toList.sorted
    (words
      .filter(w => wordLen == w.length && wordLowerCase != w.toLowerCase))
      .filter(w => w.toLowerCase.toList.sorted == charSeqSorted)
      .toSeq
  }
}
