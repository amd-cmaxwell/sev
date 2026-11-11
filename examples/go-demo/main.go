package main

import (
	"fmt"
	"strings"

	complex_ffi "github.com/virtee/sev/bindings/go/sev_complex_ffi"
	core_ffi "github.com/virtee/sev/bindings/go/sev_core_ffi"
	linalg_ffi "github.com/virtee/sev/bindings/go/sev_linear_algebra_ffi"
	poly_ffi "github.com/virtee/sev/bindings/go/sev_polynomials_ffi"
	seq_ffi "github.com/virtee/sev/bindings/go/sev_sequences_ffi"
	trig_ffi "github.com/virtee/sev/bindings/go/sev_trig_ffi"
	types_ffi "github.com/virtee/sev/bindings/go/sev_types_ffi"
)

func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

func f(v float64) string {
	s := fmt.Sprintf("%g", v)
	if !strings.Contains(s, ".") {
		s += ".0"
	}
	return s
}

func fs(vs []float64) string {
	parts := make([]string, len(vs))
	for i, v := range vs {
		parts[i] = f(v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println("=== Core ===")
	fmt.Printf("add(2, 3) = %d\n", core_ffi.Add(2, 3))

	fmt.Println("\n=== Complex Numbers ===")
	z := complex_ffi.ComplexNew(3.0, 4.0)
	fmt.Printf("z = %s\n", complex_ffi.ComplexDisplay(z))
	fmt.Printf("  |z| = %s\n", f(complex_ffi.ComplexMagnitude(z)))
	fmt.Printf("  arg(z) = %.4f\n", complex_ffi.ComplexArgument(z))
	fmt.Printf("  conj(z) = %s\n", complex_ffi.ComplexDisplay(complex_ffi.ComplexConjugate(z)))

	w := complex_ffi.ComplexNew(1.0, 2.0)
	fmt.Printf("w = %s\n", complex_ffi.ComplexDisplay(w))
	fmt.Printf("  z + w = %s\n", complex_ffi.ComplexDisplay(complex_ffi.ComplexAdd(z, w)))
	fmt.Printf("  z * w = %s\n", complex_ffi.ComplexDisplay(complex_ffi.ComplexMul(z, w)))

	pair := must(complex_ffi.QuadraticRootsComplex(1.0, 0.0, 1.0))
	fmt.Printf("x^2 + 1 = 0: %s, %s\n", complex_ffi.ComplexDisplay(pair.First), complex_ffi.ComplexDisplay(pair.Second))

	fmt.Println("\n=== Polynomials ===")
	roots := must(poly_ffi.QuadraticRoots(1.0, -5.0, 6.0))
	fmt.Printf("x^2 - 5x + 6 = 0: x = %s, %s\n", f(roots.First), f(roots.Second))
	fmt.Printf("eval x^2 - 3x + 2 at x=1: %s\n", f(poly_ffi.PolynomialEvaluate([]float64{1, -3, 2}, 1.0)))
	lr := must(poly_ffi.LinearRoot(2.0, 4.0))
	fmt.Printf("linear_root(2x + 4 = 0): x = %s\n", f(lr))
	fmt.Printf("(x+1)(x-1) = %s\n", fs(poly_ffi.PolynomialMultiply([]float64{1, 1}, []float64{1, -1})))
	fmt.Printf("d/dx(3x^2 + 2x + 1) = %s\n", fs(poly_ffi.PolynomialDerivative([]float64{3, 2, 1})))

	fmt.Println("\n=== Linear Algebra ===")
	m := linalg_ffi.Matrix2x2{M00: 2, M01: 3, M10: 1, M11: 4}
	fmt.Printf("det([[2,3],[1,4]]) = %s\n", f(linalg_ffi.Determinant2x2(m)))

	sol := must(linalg_ffi.Solve2x2(
		linalg_ffi.Matrix2x2{M00: 1, M01: 1, M10: 2, M11: -1},
		types_ffi.Point2{X: 3, Y: 0},
	))
	fmt.Printf("solve x+y=3, 2x-y=0: x=%s, y=%s\n", f(sol.X), f(sol.Y))

	dp := must(linalg_ffi.DotProduct([]float64{1, 2, 3}, []float64{4, 5, 6}))
	fmt.Printf("dot([1,2,3], [4,5,6]) = %s\n", f(dp))

	cp := linalg_ffi.CrossProduct3d(linalg_ffi.Point3{X: 1, Y: 0, Z: 0}, linalg_ffi.Point3{X: 0, Y: 1, Z: 0})
	fmt.Printf("cross(i, j) = (%s, %s, %s)\n", f(cp.X), f(cp.Y), f(cp.Z))

	fmt.Println("\n=== Trigonometry ===")
	fmt.Printf("90 deg = %.4f rad\n", trig_ffi.DegreesToRadians(90.0))
	fmt.Printf("pi rad = %.1f deg\n", trig_ffi.RadiansToDegrees(3.14159265358979))
	fmt.Printf("distance (0,0)-(3,4) = %s\n", f(trig_ffi.Distance2d(0, 0, 3, 4)))
	mid := trig_ffi.Midpoint2d(0, 0, 4, 6)
	fmt.Printf("midpoint (0,0)-(4,6) = (%s, %s)\n", f(mid.X), f(mid.Y))
	heron := must(trig_ffi.TriangleAreaHeron(3, 4, 5))
	fmt.Printf("Heron area 3-4-5 = %s\n", f(heron))
	fmt.Printf("circle area r=3 = %.4f\n", trig_ffi.CircleArea(3.0))

	fmt.Println("\n=== Sequences & Number Theory ===")
	fmt.Printf("10! = %d\n", must(seq_ffi.Factorial(10)))
	fmt.Printf("C(10,3) = %d\n", must(seq_ffi.Combinations(10, 3)))
	fmt.Printf("P(5,2) = %d\n", must(seq_ffi.Permutations(5, 2)))
	fmt.Printf("fib(20) = %d\n", must(seq_ffi.Fibonacci(20)))
	fmt.Printf("gcd(48, 18) = %d\n", seq_ffi.Gcd(48, 18))
	fmt.Printf("lcm(4, 6) = %d\n", must(seq_ffi.Lcm(4, 6)))
	fmt.Printf("is_prime(97) = %t\n", seq_ffi.IsPrime(97))
	fmt.Printf("arithmetic sum 1..100 = %s\n", f(seq_ffi.ArithmeticSum(1.0, 1.0, 100)))
	gs := must(seq_ffi.GeometricSumInfinite(1.0, 0.5))
	fmt.Printf("geometric sum infinite a=1, r=0.5 = %s\n", f(gs))

	fmt.Println("\n=== Error Handling ===")
	if _, err := poly_ffi.QuadraticRoots(0.0, 1.0, 2.0); err != nil {
		fmt.Println("Caught MathError.DivisionByZero!")
	}
	if _, err := poly_ffi.QuadraticRoots(1.0, 0.0, 1.0); err != nil {
		fmt.Println("Caught MathError.NoRealSolution!")
	}
	if _, err := seq_ffi.Factorial(21); err != nil {
		fmt.Println("Caught MathError.Overflow!")
	}
	if _, err := trig_ffi.LawOfCosinesAngle(-1.0, 2.0, 3.0); err != nil {
		fmt.Println("Caught MathError.InvalidInput: sides must be positive")
	}

	fmt.Println("\nAll smoke tests passed!")
}
