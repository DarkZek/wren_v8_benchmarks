class Math {
    static math(a, b) {
        if (a <= 0 || b <= 0) return 1
        return a + b + this.math(a - 1, b - 1)
    }
}
