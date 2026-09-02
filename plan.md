# Ray Tracer — Build Plan

A from-scratch path tracer in Rust, built for fun and learning. No AI-generated/LLM code, except for this plan.

## Goals

- Understand how ray/path tracing actually works by building one.
- Produce nice images: spheres, materials, soft shadows, depth of field.
- Keep it idiomatic Rust and reasonably fast (rayon for parallelism).
- Have fun; correctness over cleverness; optimize only once it works.

## Non-goals (for v1)

- Real-time / GPU. This is an offline CPU renderer.
- A full scene-description language or GUI. Scenes are built in code.
- Physically-accurate spectral rendering. RGB is fine.

## Reference material

- *Ray Tracing in One Weekend* (Shirley) — the spine of phases 1–7.
- *Ray Tracing: The Next Week* — motion blur, BVH, textures, volumes.
- *Physically Based Rendering* (PBRT) — deeper theory when curious.

Read these for concepts, but write the Rust yourself.

## Output format

Start with PPM (P3 ASCII, then P6 binary) — trivial to write, no dependencies.
Add PNG output later via a crate if wanted.

---

## Phases

Each phase ends with a committable milestone and an image to look at.

### Phase −1 — Learn enough Rust (first-time Rust)

Don't try to learn all of Rust. A ray tracer only needs a slice of it, and
it's a friendly project: mostly value types and math, barely any borrow-checker
fights. Target ~a few evenings before Phase 0.

- [ ] Install & tooling: `rustup`, then `cargo build` / `cargo run` /
      `cargo test`. Know that `cargo run --release` is *much* faster (use it
      for real renders).
- [ ] Read *The Rust Book* (doc.rust-lang.org/book) chapters 1–6 and 10:
      variables, functions, ownership, structs, enums/`match`, `Option`,
      generics/traits. Skim the rest.
- [ ] Do `rustlings` (a few sets) for muscle memory — optional but great.
      Sets that match what this project actually leans on:
      - Do: `intro`, `variables`, `functions`, `if`, `primitive_types`,
            `move_semantics` (this is what makes Copy-vs-move click),
            `structs`, `enums`, `option`, `error_handling` (just enough for
            `?`), `traits`, `generics`, `vecs`, `quiz1`/`quiz2`.
      - Optional: `iterators` (pays off once writing the pixel loop and later
            rayon's `par_iter`), `tests`.
      - Skip: `lifetimes`, `smart_pointers` (Rc/RefCell), `threads`, `macros`,
            `conversions`, `clippy` — matches the skip list below.
- [ ] The specific bits this project leans on:
  - **Structs + `impl`** — this is 90% of the code (`Vec3`, `Ray`, `Sphere`).
  - **Traits** — `Hittable` and `Material` are traits; this is Rust's version
        of an interface. `Box<dyn Trait>` to store "any material."
  - **`Option<T>`** — `hit()` returns `Option<HitRecord>`; `scatter()` returns
        `Option<(Ray, Color)>`. Learn `match`, `if let`, `?`.
  - **`Copy` vs move** — make `Vec3` derive `Copy`; then it behaves like a
        number and ownership mostly disappears. This is why the project is easy.
  - **Operator overloading** — `impl std::ops::Add for Vec3`, etc., so you can
        write `a + b`. Nice early win.
  - **`f64` methods** — `.sqrt()`, `.abs()`, `.min()`, `.powi()`.
- [ ] Skip for now: lifetimes, `Rc`/`RefCell`, async, macros. You won't need
      them for v1 (rayon in Phase 7 is a couple of lines, no theory required).
- **Milestone:** you can write a `Vec3` struct with an `add` method and a test
      that passes. That's Phase 0 already starting.

### Phase 0 — Scaffolding
- [done] `Vec3` type (x, y, z): add, sub, scalar mul/div, dot, cross, length,
      normalize, elementwise mul. Alias as `Point3` and `Color`.
- [ ] Write a hardcoded gradient image to PPM; view it.
- [ ] Unit tests for the vector math.
- **Milestone:** a color gradient PPM opens correctly.

### Phase 1 — Rays & camera
- [ ] `Ray { origin, direction }` with `at(t)`.
- [ ] Simple camera: viewport, focal length, generate rays through pixels.
- [ ] Background gradient based on ray direction (sky).
- **Milestone:** blue-white vertical gradient rendered through rays.

### Phase 2 — Spheres & hits
- [ ] Ray/sphere intersection (solve the quadratic).
- [ ] `HitRecord { point, normal, t, front_face }`; correct outward normals.
- [ ] `Hittable` trait + `HittableList`; nearest-hit over a list.
- [ ] Shade by surface normal (map normal to RGB) to sanity-check.
- **Milestone:** a shaded sphere on a ground sphere.

### Phase 3 — Antialiasing
- [ ] Multiple jittered samples per pixel (use `rand`), average the color.
- [ ] Gamma correction (sqrt) on output.
- **Milestone:** smooth edges, no jaggies.

### Phase 4 — Diffuse materials
- [ ] Random vectors in/on unit sphere; Lambertian scatter.
- [ ] Recursive `ray_color` with a depth cap.
- **Milestone:** matte spheres with soft indirect shading.

### Phase 5 — Metal & dielectrics
- [ ] `Material` trait: `scatter -> Option<(Ray, attenuation)>`.
- [ ] Metal: reflection + fuzz.
- [ ] Dielectric: refraction (Snell), total internal reflection, Schlick.
- **Milestone:** the classic three-ball scene (matte / metal / glass).

### Phase 6 — Positionable camera
- [ ] `look_from` / `look_at` / `vup`, vertical FOV.
- [ ] Defocus blur (aperture / lens radius) for depth of field.
- **Milestone:** an angled shot with depth of field.

### Phase 7 — The cover scene + performance
- [ ] Procedurally scatter many random spheres (the RTIOW cover image).
- [ ] Parallelize the pixel loop with `rayon` (`par_iter` over rows/pixels);
      make RNG per-thread.
- [ ] Time a render; note samples/sec before and after rayon.
- **Milestone:** the cover render at higher sample count, multithreaded.

---

## Stretch goals (pick what's fun)

- **BVH** (bounding volume hierarchy) — the big speedup for many objects.
- **Motion blur** — time-parameterized rays.
- **Textures** — solid, checker, image textures; Perlin noise.
- **Quads / triangles** — then load a simple OBJ mesh.
- **Lights & emissive materials** — a Cornell box.
- **Volumes** — participating media (fog/smoke).
- **PNG output**, and a tiny scene file format (RON/JSON).
- **Importance sampling** — lower noise per sample.

## Engineering notes

- Module layout to grow into: `vec3`, `ray`, `camera`, `hittable`, `sphere`,
  `material`, `color`, `scene`, `main`.
- Keep a `f64` vs `f32` decision explicit; RTIOW uses `f64`.
- Add `criterion` benchmarks once perf matters; don't guess, measure.
- Commit per milestone with the resulting image checked in (or in a `renders/`
  dir that's gitignored, with one or two hero images kept).

## Definition of done (v1)

The cover scene renders correctly with antialiasing, all three material types,
depth of field, and rayon multithreading — matching the RTIOW final image.
