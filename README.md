<h1 align="center" id="title">Ciri Engine</h1>

<p align="center"><img src="https://socialify.git.ci/malezjaa/ciri/image?custom_language=Rust&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Auto" alt="ciri" width="640" height="320" /></p>
<h3 id="description" align="center">Simple game engine using <a href="https://github.com/asny/three-d">three-d</a> for rendering.</h3>

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
