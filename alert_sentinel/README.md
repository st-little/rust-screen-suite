# Alert Sentinel

A Rust + Dioxus-based Windows application for alert monitoring and management.

## Project Structure

```
alert_sentinel/
├─ assets/ # Static assets (images, styles, etc.)
├─ src/ # Rust source code
│  ├─ main.rs # Application entry point
├─ Cargo.toml # Rust package manifest
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.86.0 recommended)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/installation/) (`cargo install dioxus-cli --version=0.6.3`)
- [OpenCV](https://opencv.org/) (version 4.11.0, install via [Chocolatey](https://chocolatey.org/): `choco install -y opencv --version=4.11.0`)
- [Node.js & npm](https://nodejs.org/) for Tailwind CSS

## Development

### 1. Install Dependencies

- Install Rust, Dioxus CLI, npm, and OpenCV as described above.

### 2. Tailwind CSS

```bash
npm install -g tailwindcss
npx tailwindcss -i ./tailwind.css -o ./assets/styling/tailwind.css --watch
```

### 3. Run the App (Desktop)

```bash
dx serve --platform desktop
```

### 4. Generate Cargo license file (HTML, using cargo-about)

First, install [cargo-about](https://github.com/EmbarkStudios/cargo-about):

```bash
cargo install cargo-about
```

Then generate the license file in HTML format:

```bash
cargo about generate about.hbs --output-file alert_sentinel/public/licenses/rust-licenses.html
```

### 5. Generate Node module license file (if using npm modules)

Then generate the license file in Markdown format:

```bash
npx license-checker-rseidelsohn --markdown > public/licenses/node-licenses.md
```

### 6. Build Release Binary

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx build --release --platform desktop --package alert_sentinel --features bundle
```

The binary will be located in `target\dx\alert_sentinel\release\windows\app\alert_sentinel.exe.`

## CI/CD

- GitHub Actions workflows are provided for automated build and release.
- See `.github/workflows/ci_alert_sentinel.yml` for details.