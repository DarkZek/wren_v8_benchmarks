function math(a, b) { 
    if (a <= 0 || b <= 0) return 1
    return (a + b + math(a - 1, b - 1))
} math;
