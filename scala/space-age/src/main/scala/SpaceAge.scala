case class SpaceAge(seconds: Long) {
  def onEarth(): Double = ((seconds + 0.005) / 31557600.0 * 100 round) / 100.0
  def onMercury(): Double =
    ((seconds + 0.005) / 31557600.0 / 0.2408467 * 100 round) / 100.0
  def onVenus(): Double =
    ((seconds + 0.005) / 31557600.0 / 0.61519726 * 100 round) / 100.0
  def onMars(): Double =
    ((seconds + 0.005) / 31557600.0 / 1.8808158 * 100 round) / 100.0
  def onJupiter(): Double =
    ((seconds + 0.005) / 31557600 / 11.862615 * 100 round) / 100.0
  def onSaturn(): Double =
    ((seconds + 0.005) / 31557600 / 29.447498 * 100 round) / 100.0
  def onUranus(): Double =
    ((seconds + 0.005) / 31557600 / 84.016846 * 100 round) / 100.0
  def onNeptune(): Double =
    ((seconds + 0.005) / 31557600 / 164.79132 * 100 round) / 100.0
}
