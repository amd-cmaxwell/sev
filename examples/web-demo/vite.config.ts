import { defineConfig } from "vite";

export default defineConfig({
  assetsInclude: ["**/*.wasm"],
  optimizeDeps: {
    exclude: [
      "@virtee/sev-core-ffi",
      "@virtee/sev-complex-ffi",
      "@virtee/sev-polynomials-ffi",
      "@virtee/sev-linear-algebra-ffi",
      "@virtee/sev-trig-ffi",
      "@virtee/sev-sequences-ffi",
      "@virtee/sev-types-ffi",
    ],
  },
});
