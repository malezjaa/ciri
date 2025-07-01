<h1 align="center" id="title">Ciri Engine</h1>

<p align="center"><img src="https://socialify.git.ci/malezjaa/ciri/image?font=Inter&amp;issues=1&amp;language=1&amp;name=1&amp;owner=1&amp;pattern=Transparent&amp;pulls=1&amp;stargazers=1&amp;theme=Auto" alt="project-image"></p>

Simple game engine using [three-d](https://github.com/asny/three-d) for rendering.

<p align="center"><img src="https://img.shields.io/github/sponsors/malezjaa" alt="shields"><img src="https://img.shields.io/github/issues/malezjaa/ciri" alt="shields"><img src="https://img.shields.io/github/issues-pr/malezjaa/ciri" alt="shields"></p>

<h2>🧐 Features</h2>

Here're some of the project's best features:

* Multiplatform
* 2D and 3D
* Easy usage

<h2>🛠️ Installation Steps:</h2>

<p>1. Add crate</p>

```
cargo add ciri_core
```

<p>2. Add code</p>

```rust
fn main() {
    let engine = Engine::new(EngineOptions::builder().with_name("Builder Example").build())
        .with_orbit_camera();

    engine.render_loop_with_camera(move |mut frame, camera| {
        ...
    })
}
```

<p>3. Run</p>

```
cargo run
```

<h2>🛡️ License:</h2>

This project is licensed under the MIT

<h2>💖Like my work?</h2>

<p>Support me at https://github.com/sponsors/malezjaa</p>
