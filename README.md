# egui_1

Rust로 작성된 크로스 플랫폼 GUI 애플리케이션입니다. **egui**와 **eframe**을 사용하여 네이티브 데스크톱과 웹 브라우저(WASM) 모두에서 실행 가능합니다.

---

## 프로젝트 구조

```
egui_1/
├── Cargo.toml          # 프로젝트 설정 및 의존성 정의
├── index.html          # WASM 빌드용 HTML 엔트리포인트 (Trunk가 사용)
├── src/
│   ├── main.rs         # 네이티브 바이너리 진입점 (egui_native)
│   ├── lib.rs          # WASM cdylib 진입점 (wasm_bindgen 호출)
│   └── shared.rs       # 공유 애플리케이션 로직 (WhiteFCApp)
└── target/             # 빌드 산출물 (네이티브 및 WASM)
```

### 주요 파일 설명
- **`src/shared.rs`**: `WhiteFCApp` 구조체와 UI 로직을 정의. `main.rs`와 `lib.rs`가 `include!` 매크로로 이 파일을 포함하여 중복을 방지.
- **`src/main.rs`**: 네이티브 플랫폼용 진입점. `eframe::run_native()`로 애플리케이션을 실행.
- **`src/lib.rs`**: WASM 타겟용 진입점. `#[wasm_bindgen(start)]`로 브라우저에서 자동 실행되며, `eframe::WebRunner`로 캔버스에 앱을 마운트.
- **`index.html`**: Trunk가 읽는 HTML 템플릿. `<link data-trunk rel="rust" data-target-name="egui_1" />`로 빌드할 Rust 아티팩트를 지정.

---

## 빌드 및 실행 방법

### 사전 요구사항
- **Rust** (1.70 이상 권장)
- **Trunk** (WASM 빌드 도구):
  ```bash
  cargo install --locked trunk
  ```
- **wasm32 타겟** (WASM 빌드용):
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

---

## 네이티브 빌드 (Desktop)

### 개발 빌드 (디버그)
```bash
cargo build --bin egui_native
```
- 산출물: `target/debug/egui_native` (macOS/Linux) 또는 `egui_native.exe` (Windows)

### 실행
```bash
cargo run --bin egui_native
```

### 릴리즈 빌드 (최적화)
```bash
cargo build --bin egui_native --release
```
- 산출물: `target/release/egui_native`
- 실행 파일 크기가 작아지고 성능이 향상됩니다.

### 테스트 (네이티브)
```bash
cargo test
```
> 참고: 현재 프로젝트에는 테스트 코드가 없습니다. 필요 시 `src/` 아래에 `#[cfg(test)]` 모듈을 추가하세요.

---

## WASM 빌드 (Web)

### 개발 서버 실행 (라이브 리로드)
```bash
trunk serve --open
```
- 자동으로 브라우저가 열리며, 코드 변경 시 자동 재빌드·새로고침됩니다.
- 기본 주소: `http://127.0.0.1:8080`

### 개발 빌드 (디버그)
```bash
trunk build
```
- 산출물: `dist/` 폴더에 `index.html`, `egui_1_bg.wasm`, JS glue 코드 등이 생성됩니다.

### 릴리즈 빌드 (최적화)
```bash
trunk build --release
```
- WASM 바이너리 크기가 줄어들고 최적화됩니다.
- 산출물: `dist/` 폴더 (정적 파일 호스팅 가능)

### 테스트 (WASM)
WASM 타겟에서의 유닛 테스트는 [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)과 `wasm-bindgen-test`를 사용할 수 있습니다:
```bash
# wasm-pack 설치
cargo install wasm-pack

# WASM 테스트 실행 (브라우저 headless 모드)
wasm-pack test --headless --chrome
```
> 현재 프로젝트에는 WASM 테스트 설정이 포함되어 있지 않습니다. 필요 시 `Cargo.toml`에 `wasm-bindgen-test` 의존성을 추가하세요.

---

## 배포 (WASM)

릴리즈 빌드 후 `dist/` 폴더의 내용을 정적 파일 호스팅 서비스(GitHub Pages, Netlify, Vercel 등)에 업로드하면 됩니다.

### 예시: GitHub Pages
```bash
# 릴리즈 빌드
trunk build --release

# dist/ 폴더를 gh-pages 브랜치로 푸시 (예시)
# (실제로는 GitHub Actions를 사용하거나 수동으로 푸시)
```

### 예시: 로컬 HTTP 서버로 테스트
```bash
# Python 간이 서버
python3 -m http.server -d dist 8000

# 브라우저에서 http://localhost:8000 접속
```

---

## 사용 라이브러리

### 핵심 의존성
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)** `0.27.2`  
  egui 애플리케이션을 네이티브 및 웹에서 실행하기 위한 프레임워크. 윈도우 생성, 이벤트 루프, 렌더링을 추상화합니다.
  
- **[egui](https://github.com/emilk/egui)** (eframe 내부 의존)  
  즉시 모드(immediate mode) GUI 라이브러리. 간단한 API로 UI를 빠르게 구성할 수 있습니다.

- **[glow](https://github.com/grovesNL/glow)**  
  OpenGL/WebGL 렌더링 백엔드. eframe이 그래픽을 렌더링할 때 사용합니다.

### 네이티브 전용 (조건부 컴파일)
- **[tracing-subscriber](https://github.com/tokio-rs/tracing)** `0.3`  
  네이티브 환경에서 로깅 및 진단 메시지를 출력하는 데 사용됩니다.

### WASM 전용 (조건부 컴파일)
- **[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)** `0.2`  
  Rust와 JavaScript 간의 바인딩을 생성합니다. WASM 모듈을 브라우저에서 호출 가능하게 만듭니다.

- **[wasm-bindgen-futures](https://github.com/rustwasm/wasm-bindgen)** `0.4`  
  WASM 환경에서 Rust의 `async`/`await`를 JavaScript Promise와 연결합니다.

- **[console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook)** `0.1.7`  
  Rust 패닉 메시지를 브라우저 콘솔에 표시하여 디버깅을 쉽게 합니다.

- **[tracing-wasm](https://github.com/stoically/tracing-wasm)** `0.2`  
  WASM 환경에서 `tracing` 로그를 브라우저 콘솔로 전달합니다.

### 빌드 도구
- **[Trunk](https://trunkrs.dev/)**  
  Rust WASM 애플리케이션을 빌드·번들·서빙하는 도구. HTML 자산 처리, wasm-bindgen 호출, 라이브 리로드를 자동화합니다.

---

## 주요 기능

- **크로스 플랫폼**: 단일 코드베이스로 macOS, Linux, Windows, 웹 브라우저를 모두 지원합니다.
- **즉시 모드 UI**: egui의 간결한 API로 상태 관리와 렌더링을 동시에 처리합니다.
- **라이트 테마**: 기본적으로 밝은 배경(`egui::Visuals::light()`)을 사용합니다.
- **코드 공유**: `src/shared.rs`를 통해 네이티브와 WASM 빌드가 동일한 앱 로직을 사용합니다.

---

## 문제 해결

### 네이티브 빌드 시 OpenGL 오류
일부 Linux 배포판에서는 OpenGL 개발 라이브러리가 필요합니다:
```bash
# Ubuntu/Debian
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev

# Fedora
sudo dnf install clang libxkbcommon-devel pkg-config openssl-devel
```

### WASM 빌드 시 wasm-bindgen 버전 불일치
Trunk가 자동으로 설치한 `wasm-bindgen-cli`와 `Cargo.toml`의 `wasm-bindgen` 버전이 다를 경우 오류가 발생할 수 있습니다. 두 버전을 일치시켜 주세요:
```bash
cargo update -p wasm-bindgen
```

### 브라우저에서 빈 화면
- 브라우저 개발자 도구(F12)의 콘솔을 확인하세요.
- `index.html`의 `<canvas id="the_canvas_id">`가 `src/lib.rs`의 `.start("the_canvas_id", ...)`와 일치하는지 확인하세요.

---

## 라이선스

이 프로젝트는 교육 목적으로 작성되었습니다. 라이선스는 별도로 명시하지 않았습니다.

---

## 참고 자료

- [egui 공식 문서](https://docs.rs/egui/)
- [eframe 공식 문서](https://docs.rs/eframe/)
- [Trunk 공식 문서](https://trunkrs.dev/)
- [Rust WASM 공식 가이드](https://rustwasm.github.io/docs/book/)