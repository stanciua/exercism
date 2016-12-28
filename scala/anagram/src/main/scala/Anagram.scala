case class Anagram(val word: String) {
  def matches(words: Seq[String]): Seq[String] = {
    val wordLen = word.length
    val wordLowerCase = word.toLowerCase
    val charSeqSorted = wordLowerCase.toList.sorted
    (words
      .filter(w => {
        val wLowerCase = w.toLowerCase
        wordLen == w.length && wordLowerCase != wLowerCase && wLowerCase.toList.sorted == charSeqSorted
      }))
      .toSeq
  }
}
