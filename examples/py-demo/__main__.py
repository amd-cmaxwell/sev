from sev_types_ffi import MathError, Point2
from sev_core_ffi import add
from sev_complex_ffi import (
    complex_new, complex_magnitude, complex_argument, complex_conjugate,
    complex_add, complex_mul, complex_display, quadratic_roots_complex,
)
from sev_polynomials_ffi import (
    polynomial_evaluate, quadratic_roots, linear_root,
    polynomial_multiply, polynomial_derivative,
)
from sev_linear_algebra_ffi import (
    Matrix2x2, Point3,
    determinant_2x2, solve_2x2, dot_product, cross_product_3d,
)
from sev_trig_ffi import (
    degrees_to_radians, radians_to_degrees,
    distance_2d, midpoint_2d, triangle_area_heron, circle_area,
    law_of_cosines_angle,
)
from sev_sequences_ffi import (
    arithmetic_sum, geometric_sum_infinite,
    factorial, permutations, combinations,
    gcd, lcm, fibonacci, is_prime,
)


def main():
    print("=== Core ===")
    print(f"add(2, 3) = {add(2, 3)}")

    print("\n=== Complex Numbers ===")
    z = complex_new(3.0, 4.0)
    print(f"z = {complex_display(z)}")
    print(f"  |z| = {complex_magnitude(z)}")
    print(f"  arg(z) = {complex_argument(z):.4f}")
    print(f"  conj(z) = {complex_display(complex_conjugate(z))}")

    w = complex_new(1.0, 2.0)
    print(f"w = {complex_display(w)}")
    print(f"  z + w = {complex_display(complex_add(z, w))}")
    print(f"  z * w = {complex_display(complex_mul(z, w))}")

    pair = quadratic_roots_complex(1.0, 0.0, 1.0)
    print(f"x^2 + 1 = 0: {complex_display(pair.first)}, {complex_display(pair.second)}")

    print("\n=== Polynomials ===")
    roots = quadratic_roots(1.0, -5.0, 6.0)
    print(f"x^2 - 5x + 6 = 0: x = {roots.first}, {roots.second}")
    print(f"eval x^2 - 3x + 2 at x=1: {polynomial_evaluate([1.0, -3.0, 2.0], 1.0)}")
    print(f"linear_root(2x + 4 = 0): x = {linear_root(2.0, 4.0)}")
    print(f"(x+1)(x-1) = {polynomial_multiply([1.0, 1.0], [1.0, -1.0])}")
    print(f"d/dx(3x^2 + 2x + 1) = {polynomial_derivative([3.0, 2.0, 1.0])}")

    print("\n=== Linear Algebra ===")
    m = Matrix2x2(m00=2.0, m01=3.0, m10=1.0, m11=4.0)
    print(f"det([[2,3],[1,4]]) = {determinant_2x2(m)}")

    sol = solve_2x2(
        Matrix2x2(m00=1.0, m01=1.0, m10=2.0, m11=-1.0),
        Point2(x=3.0, y=0.0),
    )
    print(f"solve x+y=3, 2x-y=0: x={sol.x}, y={sol.y}")

    dp = dot_product([1.0, 2.0, 3.0], [4.0, 5.0, 6.0])
    print(f"dot([1,2,3], [4,5,6]) = {dp}")

    cp = cross_product_3d(Point3(x=1.0, y=0.0, z=0.0), Point3(x=0.0, y=1.0, z=0.0))
    print(f"cross(i, j) = ({cp.x}, {cp.y}, {cp.z})")

    print("\n=== Trigonometry ===")
    print(f"90 deg = {degrees_to_radians(90.0):.4f} rad")
    print(f"pi rad = {radians_to_degrees(3.14159265358979):.1f} deg")
    print(f"distance (0,0)-(3,4) = {distance_2d(0.0, 0.0, 3.0, 4.0)}")
    mid = midpoint_2d(0.0, 0.0, 4.0, 6.0)
    print(f"midpoint (0,0)-(4,6) = ({mid.x}, {mid.y})")
    print(f"Heron area 3-4-5 = {triangle_area_heron(3.0, 4.0, 5.0)}")
    print(f"circle area r=3 = {circle_area(3.0):.4f}")

    print("\n=== Sequences & Number Theory ===")
    print(f"10! = {factorial(10)}")
    print(f"C(10,3) = {combinations(10, 3)}")
    print(f"P(5,2) = {permutations(5, 2)}")
    print(f"fib(20) = {fibonacci(20)}")
    print(f"gcd(48, 18) = {gcd(48, 18)}")
    print(f"lcm(4, 6) = {lcm(4, 6)}")
    print(f"is_prime(97) = {is_prime(97)}")
    print(f"arithmetic sum 1..100 = {arithmetic_sum(1.0, 1.0, 100)}")
    print(f"geometric sum infinite a=1, r=0.5 = {geometric_sum_infinite(1.0, 0.5)}")

    print("\n=== Error Handling ===")
    try:
        quadratic_roots(0.0, 1.0, 2.0)
    except MathError.DivisionByZero:
        print("Caught MathError.DivisionByZero!")

    try:
        quadratic_roots(1.0, 0.0, 1.0)
    except MathError.NoRealSolution:
        print("Caught MathError.NoRealSolution!")

    try:
        factorial(21)
    except MathError.Overflow:
        print("Caught MathError.Overflow!")

    try:
        law_of_cosines_angle(-1.0, 2.0, 3.0)
    except MathError.InvalidInput as e:
        print(f"Caught MathError.InvalidInput: {e}")

    print("\nAll demos passed!")


if __name__ == "__main__":
    main()
