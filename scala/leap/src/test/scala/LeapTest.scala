import org.scalatest._

class LeapTest extends FunSuite {
  test("vanilla leap year") {
    assert(Year(1996).isLeap)
  }

  test("any old year") {
    assert(!Year(1997).isLeap)
  }

  test("an even year") {
    assert(!Year(1986).isLeap)
  }

  test("century") {
    assert(!Year(1900).isLeap)
  }

  test("exceptional century") {
    assert(Year(2000).isLeap)
  }

  test("exceptional century that is no millenium") {
    assert(Year(1600).isLeap)
  }
}
