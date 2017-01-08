import scala.annotation._
object Sublist {
  @tailrec
  private def checkForSuperOrSubLists[T](list1: List[T],
                                         list2: List[T],
                                         subOrSuperList: Int): Int = {
    if (!list2.isEmpty && list2.length >= list1.length) {
      if (list1 == list2.take(list1.length)) {
        return subOrSuperList
      }
    } else {
      return Unequal
    }

    checkForSuperOrSubLists(list1, list2.tail, subOrSuperList)

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
