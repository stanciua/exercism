class Darts {
    double x;
    double y;
    final static double OUTER_RADIUS = 10;
    final static double MIDDLE_RADIUS = 5;
    final static double INNER_RADIUS = 1;

    Darts(double x, double y) {
        this.x = x;
        this.y = y;
    }

    static double distanceBetween2Points(double x1, double y1, double x2, double y2) {
        return Math.sqrt(Math.pow(Math.abs(x2 - x1), 2) + Math.pow(Math.abs(y2 -y1), 2));
    }

    int score() {
    double radius = Darts.distanceBetween2Points(0, 0, this.x, this.y);
    if (radius <= Darts.OUTER_RADIUS && radius > Darts.MIDDLE_RADIUS) {
        return 1;
    } else if (radius <= Darts.MIDDLE_RADIUS && radius > Darts.INNER_RADIUS) {
        return 5;
    } else if (radius <= INNER_RADIUS) {
        return 10;
    }

    return 0;
    }
}
