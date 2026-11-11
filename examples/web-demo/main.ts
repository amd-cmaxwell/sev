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

// ── Wasm initialization ─────────────────────────────────────────────────────

const PACKAGES = [
  { init: initTypes, pkg: "sev-types-ffi", mod: "sev_types_ffi" },
  { init: initCore, pkg: "sev-core-ffi", mod: "sev_core_ffi" },
  { init: initComplex, pkg: "sev-complex-ffi", mod: "sev_complex_ffi" },
  { init: initPolynomials, pkg: "sev-polynomials-ffi", mod: "sev_polynomials_ffi" },
  { init: initLinearAlgebra, pkg: "sev-linear-algebra-ffi", mod: "sev_linear_algebra_ffi" },
  { init: initTrig, pkg: "sev-trig-ffi", mod: "sev_trig_ffi" },
  { init: initSequences, pkg: "sev-sequences-ffi", mod: "sev_sequences_ffi" },
];

async function initAll() {
  await Promise.all(
    PACKAGES.map(({ init, pkg, mod }) =>
      init(fetch(new URL(`./node_modules/@virtee/${pkg}/src/${mod}.wasm`, import.meta.url)))
    )
  );
}

// ── DOM helpers ──────────────────────────────────────────────────────────────

function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  attrs?: Record<string, string>,
  ...children: (string | Node)[]
): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (attrs) for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
  for (const c of children) e.append(typeof c === "string" ? document.createTextNode(c) : c);
  return e;
}

function numInput(id: string, value: string, width = "5rem") {
  return el("input", { type: "number", id, value, style: `width:${width}` });
}

function resultSpan(id: string) {
  return el("span", { class: "result", id });
}

function calcRow(label: string, ...children: (string | Node)[]) {
  const row = el("div", { class: "calc-row" });
  row.append(el("label", {}, label));
  for (const c of children) row.append(typeof c === "string" ? document.createTextNode(c) : c);
  return row;
}

function val(id: string): number {
  return parseFloat((document.getElementById(id) as HTMLInputElement).value);
}

function bigVal(id: string): bigint {
  return BigInt((document.getElementById(id) as HTMLInputElement).value);
}

function setResult(id: string, text: string, isError = false) {
  const r = document.getElementById(id)!;
  r.textContent = text;
  r.className = isError ? "result error" : "result";
}

function btn(label: string, onClick: () => void) {
  const b = el("button", {}, label);
  b.addEventListener("click", () => {
    try { onClick(); } catch (e: any) {
      const nearestResult = b.parentElement?.querySelector(".result");
      if (nearestResult) {
        nearestResult.textContent = `Error: ${e.message || e}`;
        nearestResult.className = "result error";
      }
    }
  });
  return b;
}

// ── Interactive calculator sections ──────────────────────────────────────────

function coreSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Core Arithmetic"));
  s.append(calcRow("add(a, b)",
    numInput("core-a", "2"), " + ", numInput("core-b", "3"), " ",
    btn("Compute", () => setResult("core-r", `= ${add(val("core-a"), val("core-b"))}`)),
    " ", resultSpan("core-r"),
  ));
  return s;
}

function complexSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Complex Numbers"));

  s.append(calcRow("z = a + bi",
    numInput("cx-a", "3"), " + ", numInput("cx-b", "4"), "i ",
    btn("Analyze", () => {
      const z = complexNew(val("cx-a"), val("cx-b"));
      setResult("cx-r",
        `|z| = ${complexMagnitude(z)}, arg(z) = ${complexArgument(z).toFixed(4)}, ` +
        `conj(z) = ${complexDisplay(complexConjugate(z))}`);
    }),
    " ", resultSpan("cx-r"),
  ));

  s.append(calcRow("z + w",
    "w = ", numInput("cx-wa", "1"), " + ", numInput("cx-wb", "2"), "i ",
    btn("Add", () => {
      const z = complexNew(val("cx-a"), val("cx-b"));
      const w = complexNew(val("cx-wa"), val("cx-wb"));
      setResult("cx-add-r", `= ${complexDisplay(complexAdd(z, w))}`);
    }),
    " ",
    btn("Multiply", () => {
      const z = complexNew(val("cx-a"), val("cx-b"));
      const w = complexNew(val("cx-wa"), val("cx-wb"));
      setResult("cx-add-r", `= ${complexDisplay(complexMul(z, w))}`);
    }),
    " ", resultSpan("cx-add-r"),
  ));

  return s;
}

function trigSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Trigonometry"));

  s.append(calcRow("Degrees to radians",
    numInput("trig-deg", "90"), "° ",
    btn("Convert", () => setResult("trig-dr", `= ${degreesToRadians(val("trig-deg")).toFixed(6)} rad`)),
    " ", resultSpan("trig-dr"),
  ));

  s.append(calcRow("Radians to degrees",
    numInput("trig-rad", "3.14159", "6rem"), " rad ",
    btn("Convert", () => setResult("trig-rd", `= ${radiansToDegrees(val("trig-rad")).toFixed(4)}°`)),
    " ", resultSpan("trig-rd"),
  ));

  s.append(calcRow("Distance 2D",
    "(", numInput("dist-x1", "0"), ",", numInput("dist-y1", "0"), ") to (",
    numInput("dist-x2", "3"), ",", numInput("dist-y2", "4"), ") ",
    btn("Compute", () =>
      setResult("dist-r", `= ${distance2d(val("dist-x1"), val("dist-y1"), val("dist-x2"), val("dist-y2"))}`)
    ),
    " ", resultSpan("dist-r"),
  ));

  s.append(calcRow("Triangle area (Heron)",
    "sides: ", numInput("heron-a", "3"), numInput("heron-b", "4"), numInput("heron-c", "5"), " ",
    btn("Compute", () =>
      setResult("heron-r", `= ${triangleAreaHeron(val("heron-a"), val("heron-b"), val("heron-c"))}`)
    ),
    " ", resultSpan("heron-r"),
  ));

  s.append(calcRow("Circle area",
    "r = ", numInput("circle-r", "3"), " ",
    btn("Compute", () => setResult("circle-res", `= ${circleArea(val("circle-r")).toFixed(4)}`)),
    " ", resultSpan("circle-res"),
  ));

  return s;
}

function polynomialSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Polynomials"));

  s.append(calcRow("Quadratic roots",
    numInput("qa", "1"), "x² + ", numInput("qb", "-5"), "x + ", numInput("qc", "6"), " = 0 ",
    btn("Solve", () => {
      const r = quadraticRoots(val("qa"), val("qb"), val("qc"));
      setResult("q-r", `x = ${r.first}, ${r.second}`);
    }),
    " ", resultSpan("q-r"),
  ));

  s.append(calcRow("Linear root",
    numInput("la", "2"), "x + ", numInput("lb", "4"), " = 0 ",
    btn("Solve", () => setResult("l-r", `x = ${linearRoot(val("la"), val("lb"))}`)),
    " ", resultSpan("l-r"),
  ));

  s.append(calcRow("Evaluate poly",
    "coeffs: ", el("input", { type: "text", id: "pe-c", value: "1, -3, 2" }),
    " at x = ", numInput("pe-x", "1"), " ",
    btn("Evaluate", () => {
      const coeffs = (document.getElementById("pe-c") as HTMLInputElement).value.split(",").map(Number);
      setResult("pe-r", `= ${polynomialEvaluate(coeffs, val("pe-x"))}`);
    }),
    " ", resultSpan("pe-r"),
  ));

  s.append(calcRow("Multiply polys",
    el("input", { type: "text", id: "pm-a", value: "1, 1" }),
    " × ",
    el("input", { type: "text", id: "pm-b", value: "1, -1" }), " ",
    btn("Multiply", () => {
      const a = (document.getElementById("pm-a") as HTMLInputElement).value.split(",").map(Number);
      const b = (document.getElementById("pm-b") as HTMLInputElement).value.split(",").map(Number);
      setResult("pm-r", `= [${[...polynomialMultiply(a, b)].join(", ")}]`);
    }),
    " ", resultSpan("pm-r"),
  ));

  s.append(calcRow("Derivative",
    "coeffs: ", el("input", { type: "text", id: "pd-c", value: "3, 2, 1" }), " ",
    btn("Differentiate", () => {
      const c = (document.getElementById("pd-c") as HTMLInputElement).value.split(",").map(Number);
      setResult("pd-r", `= [${[...polynomialDerivative(c)].join(", ")}]`);
    }),
    " ", resultSpan("pd-r"),
  ));

  return s;
}

function linearAlgebraSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Linear Algebra"));

  s.append(calcRow("Determinant 2×2",
    "[[", numInput("det-00", "2"), numInput("det-01", "3"),
    "], [", numInput("det-10", "1"), numInput("det-11", "4"), "]] ",
    btn("Compute", () => {
      const m = Matrix2x2.new({ m00: val("det-00"), m01: val("det-01"), m10: val("det-10"), m11: val("det-11") });
      setResult("det-r", `= ${determinant2x2(m)}`);
    }),
    " ", resultSpan("det-r"),
  ));

  s.append(calcRow("Solve Ax = b",
    "A = above, b = (", numInput("sol-bx", "3"), ",", numInput("sol-by", "0"), ") ",
    btn("Solve", () => {
      const m = Matrix2x2.new({ m00: val("det-00"), m01: val("det-01"), m10: val("det-10"), m11: val("det-11") });
      const b = Point2.new({ x: val("sol-bx"), y: val("sol-by") });
      const r = solve2x2(m, b);
      setResult("sol-r", `x = ${r.x}, y = ${r.y}`);
    }),
    " ", resultSpan("sol-r"),
  ));

  s.append(calcRow("Dot product",
    el("input", { type: "text", id: "dp-a", value: "1, 2, 3" }),
    " · ",
    el("input", { type: "text", id: "dp-b", value: "4, 5, 6" }), " ",
    btn("Compute", () => {
      const a = (document.getElementById("dp-a") as HTMLInputElement).value.split(",").map(Number);
      const b = (document.getElementById("dp-b") as HTMLInputElement).value.split(",").map(Number);
      setResult("dp-r", `= ${dotProduct(a, b)}`);
    }),
    " ", resultSpan("dp-r"),
  ));

  s.append(calcRow("Cross product",
    "(", numInput("cp-ax", "1"), numInput("cp-ay", "0"), numInput("cp-az", "0"),
    ") × (", numInput("cp-bx", "0"), numInput("cp-by", "1"), numInput("cp-bz", "0"), ") ",
    btn("Compute", () => {
      const a = Point3.new({ x: val("cp-ax"), y: val("cp-ay"), z: val("cp-az") });
      const b = Point3.new({ x: val("cp-bx"), y: val("cp-by"), z: val("cp-bz") });
      const r = crossProduct3d(a, b);
      setResult("cp-r", `= (${r.x}, ${r.y}, ${r.z})`);
    }),
    " ", resultSpan("cp-r"),
  ));

  return s;
}

function sequencesSection() {
  const s = el("section", {});
  s.append(el("h2", {}, "Sequences & Number Theory"));

  s.append(calcRow("Factorial",
    numInput("fact-n", "10"), "! ",
    btn("Compute", () => setResult("fact-r", `= ${factorial(bigVal("fact-n"))}`)),
    " ", resultSpan("fact-r"),
  ));

  s.append(calcRow("Fibonacci",
    "fib(", numInput("fib-n", "20"), ") ",
    btn("Compute", () => setResult("fib-r", `= ${fibonacci(bigVal("fib-n"))}`)),
    " ", resultSpan("fib-r"),
  ));

  s.append(calcRow("Combinations",
    "C(", numInput("comb-n", "10"), ",", numInput("comb-k", "3"), ") ",
    btn("Compute", () => setResult("comb-r", `= ${combinations(bigVal("comb-n"), bigVal("comb-k"))}`)),
    " ", resultSpan("comb-r"),
  ));

  s.append(calcRow("Permutations",
    "P(", numInput("perm-n", "5"), ",", numInput("perm-k", "2"), ") ",
    btn("Compute", () => setResult("perm-r", `= ${permutations(bigVal("perm-n"), bigVal("perm-k"))}`)),
    " ", resultSpan("perm-r"),
  ));

  s.append(calcRow("GCD / LCM",
    numInput("gcd-a", "48"), numInput("gcd-b", "18"), " ",
    btn("GCD", () => setResult("gcd-r", `gcd = ${gcd(bigVal("gcd-a"), bigVal("gcd-b"))}`)),
    " ",
    btn("LCM", () => setResult("gcd-r", `lcm = ${lcm(bigVal("gcd-a"), bigVal("gcd-b"))}`)),
    " ", resultSpan("gcd-r"),
  ));

  s.append(calcRow("Is prime?",
    numInput("prime-n", "97"), " ",
    btn("Check", () => {
      const result = isPrime(bigVal("prime-n"));
      setResult("prime-r", result ? "Yes, it's prime!" : "No, not prime.");
    }),
    " ", resultSpan("prime-r"),
  ));

  s.append(calcRow("Arithmetic sum",
    "a₁=", numInput("asum-a", "1"), " d=", numInput("asum-d", "1"), " n=", numInput("asum-n", "100"), " ",
    btn("Compute", () =>
      setResult("asum-r", `= ${arithmeticSum(val("asum-a"), val("asum-d"), bigVal("asum-n"))}`)
    ),
    " ", resultSpan("asum-r"),
  ));

  s.append(calcRow("Geometric sum (∞)",
    "a=", numInput("gsum-a", "1"), " r=", numInput("gsum-r", "0.5"), " ",
    btn("Compute", () =>
      setResult("gsum-res", `= ${geometricSumInfinite(val("gsum-a"), val("gsum-r"))}`)
    ),
    " ", resultSpan("gsum-res"),
  ));

  return s;
}

// ── Smoke tests ──────────────────────────────────────────────────────────────

interface TestResult { name: string; passed: boolean; error?: string }
interface TestGroup { name: string; results: TestResult[] }

function assert(condition: boolean, msg: string) {
  if (!condition) throw new Error(msg);
}

function assertEq(a: unknown, b: unknown) {
  if (a !== b) throw new Error(`expected ${b}, got ${a}`);
}

function assertDeep(a: unknown, b: unknown) {
  if (JSON.stringify(a) !== JSON.stringify(b))
    throw new Error(`expected ${JSON.stringify(b)}, got ${JSON.stringify(a)}`);
}

function assertThrowsTag(fn: () => void, tag: string): void {
  try { fn(); throw new Error("expected error"); }
  catch (e: any) {
    if (e.message === "expected error") throw e;
    if (e.tag !== tag) throw new Error(`expected tag ${tag}, got ${e.tag}`);
  }
}

function runTests(): TestGroup[] {
  const groups: TestGroup[] = [];

  function describe(name: string, fn: () => void) {
    const results: TestResult[] = [];
    const group: TestGroup = { name, results };
    groups.push(group);
    const prevIt = (globalThis as any).__currentResults;
    (globalThis as any).__currentResults = results;
    fn();
    (globalThis as any).__currentResults = prevIt;
  }

  function it(name: string, fn: () => void) {
    const results: TestResult[] = (globalThis as any).__currentResults;
    try { fn(); results.push({ name, passed: true }); }
    catch (e: any) { results.push({ name, passed: false, error: e.message || String(e) }); }
  }

  describe("Core", () => {
    it("adds two numbers", () => assertEq(add(2, 3), 5));
  });

  describe("Complex Numbers", () => {
    it("computes magnitude and argument", () => {
      const z = complexNew(3.0, 4.0);
      assertEq(complexMagnitude(z), 5.0);
      assertEq(complexArgument(z).toFixed(4), "0.9273");
    });
    it("conjugates", () => {
      const z = complexNew(3.0, 4.0);
      assertEq(complexDisplay(complexConjugate(z)), complexDisplay(complexNew(3.0, -4.0)));
    });
    it("adds and multiplies", () => {
      const z = complexNew(3.0, 4.0);
      const w = complexNew(1.0, 2.0);
      assertEq(complexDisplay(complexAdd(z, w)), complexDisplay(complexNew(4.0, 6.0)));
      assertEq(complexDisplay(complexMul(z, w)), complexDisplay(complexNew(-5.0, 10.0)));
    });
    it("solves x² + 1 = 0 over ℂ", () => {
      const { first, second } = quadraticRootsComplex(1.0, 0.0, 1.0);
      assertEq(complexMagnitude(first), 1.0);
      assertEq(complexMagnitude(second), 1.0);
      assertEq(complexArgument(first).toFixed(4), (Math.PI / 2).toFixed(4));
      assertEq(complexArgument(second).toFixed(4), (-Math.PI / 2).toFixed(4));
    });
  });

  describe("Polynomials", () => {
    it("finds quadratic roots", () => {
      const { first, second } = quadraticRoots(1.0, -5.0, 6.0);
      assertDeep([first, second].sort(), [2, 3]);
    });
    it("evaluates a polynomial", () => assertEq(polynomialEvaluate([1.0, -3.0, 2.0], 1.0), 0.0));
    it("finds linear root", () => assertEq(linearRoot(2.0, 4.0), -2.0));
    it("multiplies polynomials", () =>
      assertDeep([...polynomialMultiply([1.0, 1.0], [1.0, -1.0])], [1.0, 0.0, -1.0]));
    it("differentiates", () =>
      assertDeep([...polynomialDerivative([3.0, 2.0, 1.0])], [6.0, 2.0]));
  });

  describe("Linear Algebra", () => {
    it("computes determinant", () => {
      const m = Matrix2x2.new({ m00: 2.0, m01: 3.0, m10: 1.0, m11: 4.0 });
      assertEq(determinant2x2(m), 5.0);
    });
    it("solves 2×2 system", () => {
      const sol = solve2x2(
        Matrix2x2.new({ m00: 1.0, m01: 1.0, m10: 2.0, m11: -1.0 }),
        Point2.new({ x: 3.0, y: 0.0 }),
      );
      assertEq(sol.x, 1.0);
      assertEq(sol.y, 2.0);
    });
    it("computes dot product", () => assertEq(dotProduct([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), 32.0));
    it("computes cross product", () => {
      const cp = crossProduct3d(
        Point3.new({ x: 1.0, y: 0.0, z: 0.0 }),
        Point3.new({ x: 0.0, y: 1.0, z: 0.0 }),
      );
      assertDeep({ x: cp.x, y: cp.y, z: cp.z }, { x: 0.0, y: 0.0, z: 1.0 });
    });
  });

  describe("Trigonometry", () => {
    it("converts degrees ↔ radians", () => {
      assertEq(degreesToRadians(90.0).toFixed(4), "1.5708");
      assertEq(radiansToDegrees(Math.PI).toFixed(1), "180.0");
    });
    it("computes 2D distance", () => assertEq(distance2d(0.0, 0.0, 3.0, 4.0), 5.0));
    it("computes midpoint", () => {
      const mid = midpoint2d(0.0, 0.0, 4.0, 6.0);
      assertEq(mid.x, 2.0);
      assertEq(mid.y, 3.0);
    });
    it("triangle area (Heron)", () => assertEq(triangleAreaHeron(3.0, 4.0, 5.0), 6.0));
    it("circle area", () => assertEq(circleArea(3.0).toFixed(4), "28.2743"));
  });

  describe("Sequences & Number Theory", () => {
    it("computes factorial", () => assertEq(factorial(10n), 3628800n));
    it("computes combinations and permutations", () => {
      assertEq(combinations(10n, 3n), 120n);
      assertEq(permutations(5n, 2n), 20n);
    });
    it("computes fibonacci", () => assertEq(fibonacci(20n), 6765n));
    it("computes gcd and lcm", () => {
      assertEq(gcd(48n, 18n), 6n);
      assertEq(lcm(4n, 6n), 12n);
    });
    it("tests primality", () => {
      assertEq(isPrime(97n), true);
      assertEq(isPrime(100n), false);
    });
    it("computes arithmetic and geometric sums", () => {
      assertEq(arithmeticSum(1.0, 1.0, 100n), 5050.0);
      assertEq(geometricSumInfinite(1.0, 0.5), 2.0);
    });
  });

  describe("Error Handling", () => {
    it("throws DivisionByZero for degenerate quadratic", () =>
      assertThrowsTag(() => quadraticRoots(0.0, 1.0, 2.0), MathError_Tags.DivisionByZero));
    it("throws NoRealSolution for complex-only roots", () =>
      assertThrowsTag(() => quadraticRoots(1.0, 0.0, 1.0), MathError_Tags.NoRealSolution));
    it("throws Overflow for large factorial", () =>
      assertThrowsTag(() => factorial(21n), MathError_Tags.Overflow));
    it("throws InvalidInput with message", () => {
      try { lawOfCosinesAngle(-1.0, 2.0, 3.0); throw new Error("expected error"); }
      catch (e: any) {
        if (e.message === "expected error") throw e;
        assertEq(e.tag, MathError_Tags.InvalidInput);
        assert(e.inner.message.length > 0, "expected non-empty message");
      }
    });
  });

  return groups;
}

function renderTests(groups: TestGroup[]) {
  const panel = el("details", { id: "test-panel", open: "" });
  let total = 0, passed = 0;
  for (const g of groups) {
    for (const r of g.results) { total++; if (r.passed) passed++; }
  }

  panel.append(el("summary", {}, `Smoke Tests — ${passed}/${total} passed`));

  for (const g of groups) {
    const div = el("div", { class: "test-group" });
    div.append(el("div", { class: "test-group-name" }, g.name));
    for (const r of g.results) {
      const item = el("div", { class: `test-item ${r.passed ? "pass" : "fail"}` },
        r.passed ? r.name : `${r.name} — ${r.error}`);
      div.append(item);
    }
    panel.append(div);
  }

  const summary = el("div", { id: "test-summary", class: passed === total ? "all-pass" : "has-fail" },
    passed === total ? `All ${total} tests passed.` : `${total - passed} of ${total} tests failed.`);
  panel.append(summary);

  return panel;
}

// ── App entry ────────────────────────────────────────────────────────────────

async function main() {
  const loading = document.getElementById("loading")!;
  const appEl = document.getElementById("app")!;

  try {
    await initAll();
  } catch (e: any) {
    loading.textContent = `Failed to load WASM: ${e.message || e}`;
    loading.style.color = "var(--error)";
    return;
  }

  loading.style.display = "none";
  appEl.style.display = "block";

  appEl.append(coreSection());
  appEl.append(complexSection());
  appEl.append(trigSection());
  appEl.append(polynomialSection());
  appEl.append(linearAlgebraSection());
  appEl.append(sequencesSection());

  const groups = runTests();
  appEl.append(renderTests(groups));

  const total = groups.reduce((n, g) => n + g.results.length, 0);
  const passed = groups.reduce((n, g) => n + g.results.filter(r => r.passed).length, 0);
  document.title = `UniFFI Math Demo — ${passed}/${total} tests passed`;
}

main();
