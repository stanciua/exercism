object Sublist {
  private def checkForSuperOrSubLists[T](list1: List[T],
                                         list2: List[T],
                                         subOrSuperList: Int): Int = {
    var first = list1
    var second = list2
    while (!second.isEmpty && second.length >= first.length) {
      if (first == second.take(first.length)) {
        return subOrSuperList
      }
      second = second.tail
    }

    return Unequal
  }
  def sublist[T](list1: List[T], list2: List[T]): Int = {
    list1.length compare list2.length match {
      case x if x == 0 =>
        if (list1 == list2) return Equal else return Unequal
      case x if x < 0 => return checkForSuperOrSubLists(list1, list2, Sublist)
      case _ => {
        return checkForSuperOrSubLists(list2, list1, Superlist)
      }
    }
    Unequal
  }

  final val Equal = 0
  final val Unequal = 1
  final val Superlist = 2
  final val Sublist = 3
}
