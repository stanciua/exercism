class SpaceAge {
    double seconds;

    SpaceAge(double seconds) {
        this.seconds = seconds;
    }

    double getSeconds() {
        return this.seconds;
    }

    enum Planet {
        Earth(365.25),
        Mercury(0.2408467),
        Venus(0.61519726),
        Mars(1.8808158),
        Jupiter(11.862615),
        Saturn(29.447498),
        Uranus(84.016846),
        Neptune(164.79132);

        private final double orbitalPeriod;

        Planet(double orbitalPeriod) {
            this.orbitalPeriod = orbitalPeriod;
        }

        public double getOrbitalPeriod() {
            return this.orbitalPeriod;
        }
    }

    double onEarth() {
        return this.seconds / (1 * 60 * 60 * 24 * Planet.Earth.getOrbitalPeriod());
    }

    double onMercury() {
        return onEarth() / Planet.Mercury.getOrbitalPeriod();
    }

    double onVenus() {
        return onEarth() / Planet.Venus.getOrbitalPeriod();
    }

    double onMars() {
        return onEarth() / Planet.Mars.getOrbitalPeriod();
    }

    double onJupiter() {
        return onEarth() / Planet.Jupiter.getOrbitalPeriod();
    }

    double onSaturn() {
        return onEarth() / Planet.Saturn.getOrbitalPeriod();
    }

    double onUranus() {
        return onEarth() / Planet.Uranus.getOrbitalPeriod();
    }

    double onNeptune() {
        return onEarth() / Planet.Neptune.getOrbitalPeriod();
    }

}
