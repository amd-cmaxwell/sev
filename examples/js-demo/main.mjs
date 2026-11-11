import { describe, it, before } from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

import {
  uniffiInitAsync as initCore,
  add,
} from "@virtee/sev-core-ffi";
import {
  uniffiInitAsync as initComplex,
  complexNew,
  complexMagnitude,
  complexArgument,
  complexConjugate,
  complexAdd,
  complexMul,
  complexDisplay,
  quadraticRootsComplex,
} from "@virtee/sev-complex-ffi";
import {
  uniffiInitAsync as initPolynomials,
  polynomialEvaluate,
  quadraticRoots,
  linearRoot,
  polynomialMultiply,
  polynomialDerivative,
} from "@virtee/sev-polynomials-ffi";
import {
  uniffiInitAsync as initLinearAlgebra,
  Matrix2x2,
  Point3,
  determinant2x2,
  solve2x2,
  dotProduct,
  crossProduct3d,
} from "@virtee/sev-linear-algebra-ffi";
import {
  uniffiInitAsync as initTrig,
  degreesToRadians,
  radiansToDegrees,
  distance2d,
  midpoint2d,
  triangleAreaHeron,
  circleArea,
  lawOfCosinesAngle,
} from "@virtee/sev-trig-ffi";
import {
  uniffiInitAsync as initSequences,
  arithmeticSum,
  geometricSumInfinite,
  factorial,
  permutations,
  combinations,
  gcd,
  lcm,
  fibonacci,
  isPrime,
} from "@virtee/sev-sequences-ffi";
import {
  uniffiInitAsync as initTypes,
  MathError_Tags,
  Point2,
} from "@virtee/sev-types-ffi";

const require = createRequire(import.meta.url);
const wasmUrl = (pkg, name) =>
  pathToFileURL(require.resolve(`${pkg}/src/${name}.wasm`));

before(async () => {
  await Promise.all([
    initTypes(wasmUrl("@virtee/sev-types-ffi", "sev_types_ffi")),
    initCore(wasmUrl("@virtee/sev-core-ffi", "sev_core_ffi")),
    initComplex(wasmUrl("@virtee/sev-complex-ffi", "sev_complex_ffi")),
    initPolynomials(wasmUrl("@virtee/sev-polynomials-ffi", "sev_polynomials_ffi")),
    initLinearAlgebra(wasmUrl("@virtee/sev-linear-algebra-ffi", "sev_linear_algebra_ffi")),
    initTrig(wasmUrl("@virtee/sev-trig-ffi", "sev_trig_ffi")),
    initSequences(wasmUrl("@virtee/sev-sequences-ffi", "sev_sequences_ffi")),
  ]);
});

describe("core", () => {
  it("adds two numbers", () => {
    assert.equal(add(2, 3), 5);
  });
});

describe("complex numbers", () => {
  it("computes magnitude and argument", () => {
    const z = complexNew(3.0, 4.0);
    assert.equal(complexMagnitude(z), 5.0);
    assert.equal(complexArgument(z).toFixed(4), "0.9273");
  });

  it("conjugates", () => {
    const z = complexNew(3.0, 4.0);
    assert.equal(complexDisplay(complexConjugate(z)), complexDisplay(complexNew(3.0, -4.0)));
  });

  it("adds and multiplies", () => {
    const z = complexNew(3.0, 4.0);
    const w = complexNew(1.0, 2.0);
    assert.equal(complexDisplay(complexAdd(z, w)), complexDisplay(complexNew(4.0, 6.0)));
    assert.equal(complexDisplay(complexMul(z, w)), complexDisplay(complexNew(-5.0, 10.0)));
  });

  it("solves x^2 + 1 = 0 over C", () => {
    const { first, second } = quadraticRootsComplex(1.0, 0.0, 1.0);
    assert.equal(complexMagnitude(first), 1.0);
    assert.equal(complexMagnitude(second), 1.0);
    assert.equal(complexArgument(first).toFixed(4), (Math.PI / 2).toFixed(4));
    assert.equal(complexArgument(second).toFixed(4), (-Math.PI / 2).toFixed(4));
  });
});

describe("polynomials", () => {
  it("finds quadratic roots", () => {
    const { first, second } = quadraticRoots(1.0, -5.0, 6.0);
    assert.deepEqual([first, second].sort(), [2, 3]);
  });

  it("evaluates a polynomial", () => {
    assert.equal(polynomialEvaluate([1.0, -3.0, 2.0], 1.0), 0.0);
  });

  it("finds linear root", () => {
    assert.equal(linearRoot(2.0, 4.0), -2.0);
  });

  it("multiplies polynomials", () => {
    assert.deepEqual([...polynomialMultiply([1.0, 1.0], [1.0, -1.0])], [1.0, 0.0, -1.0]);
  });

  it("differentiates", () => {
    assert.deepEqual([...polynomialDerivative([3.0, 2.0, 1.0])], [6.0, 2.0]);
  });
});

describe("linear algebra", () => {
  it("computes determinant", () => {
    const m = Matrix2x2.new({ m00: 2.0, m01: 3.0, m10: 1.0, m11: 4.0 });
    assert.equal(determinant2x2(m), 5.0);
  });

  it("solves 2x2 system", () => {
    const sol = solve2x2(
      Matrix2x2.new({ m00: 1.0, m01: 1.0, m10: 2.0, m11: -1.0 }),
      Point2.new({ x: 3.0, y: 0.0 }),
    );
    assert.equal(sol.x, 1.0);
    assert.equal(sol.y, 2.0);
  });

  it("computes dot product", () => {
    assert.equal(dotProduct([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), 32.0);
  });

  it("computes cross product", () => {
    const cp = crossProduct3d(
      Point3.new({ x: 1.0, y: 0.0, z: 0.0 }),
      Point3.new({ x: 0.0, y: 1.0, z: 0.0 }),
    );
    assert.deepEqual({ x: cp.x, y: cp.y, z: cp.z }, { x: 0.0, y: 0.0, z: 1.0 });
  });
});

describe("trigonometry", () => {
  it("converts degrees and radians", () => {
    assert.equal(degreesToRadians(90.0).toFixed(4), "1.5708");
    assert.equal(radiansToDegrees(Math.PI).toFixed(1), "180.0");
  });

  it("computes 2d distance", () => {
    assert.equal(distance2d(0.0, 0.0, 3.0, 4.0), 5.0);
  });

  it("computes midpoint", () => {
    const mid = midpoint2d(0.0, 0.0, 4.0, 6.0);
    assert.equal(mid.x, 2.0);
    assert.equal(mid.y, 3.0);
  });

  it("computes triangle area via Heron's formula", () => {
    assert.equal(triangleAreaHeron(3.0, 4.0, 5.0), 6.0);
  });

  it("computes circle area", () => {
    assert.equal(circleArea(3.0).toFixed(4), "28.2743");
  });
});

describe("sequences & number theory", () => {
  it("computes factorial", () => {
    assert.equal(factorial(10n), 3628800n);
  });

  it("computes combinations and permutations", () => {
    assert.equal(combinations(10n, 3n), 120n);
    assert.equal(permutations(5n, 2n), 20n);
  });

  it("computes fibonacci", () => {
    assert.equal(fibonacci(20n), 6765n);
  });

  it("computes gcd and lcm", () => {
    assert.equal(gcd(48n, 18n), 6n);
    assert.equal(lcm(4n, 6n), 12n);
  });

  it("tests primality", () => {
    assert.equal(isPrime(97n), true);
    assert.equal(isPrime(100n), false);
  });

  it("computes arithmetic and geometric sums", () => {
    assert.equal(arithmeticSum(1.0, 1.0, 100n), 5050.0);
    assert.equal(geometricSumInfinite(1.0, 0.5), 2.0);
  });
});

describe("error handling", () => {
  it("throws DivisionByZero for degenerate quadratic", () => {
    assert.throws(
      () => quadraticRoots(0.0, 1.0, 2.0),
      (err) => err.tag === MathError_Tags.DivisionByZero,
    );
  });

  it("throws NoRealSolution for complex-only roots", () => {
    assert.throws(
      () => quadraticRoots(1.0, 0.0, 1.0),
      (err) => err.tag === MathError_Tags.NoRealSolution,
    );
  });

  it("throws Overflow for large factorial", () => {
    assert.throws(
      () => factorial(21n),
      (err) => err.tag === MathError_Tags.Overflow,
    );
  });

  it("throws InvalidInput with message", () => {
    assert.throws(
      () => lawOfCosinesAngle(-1.0, 2.0, 3.0),
      (err) => {
        assert.equal(err.tag, MathError_Tags.InvalidInput);
        assert.ok(err.inner.message.length > 0);
        return true;
      },
    );
  });
});
