# Hello, Cargo

## Cargo

Cargo 는 Rust 의 패키지 매니저이다.

### 프로젝트 생성

다음과 같은 명령어로 새로운 프로젝트를 생성할 수 있다.

```shell
cargo new [project name]
```

프로젝트가 생성되면, `Cargo.toml`에서 프로젝트의 정보와 Depencency 를 확인할 수 있다.

### 프로젝트 빌드

해당하는 프로젝트로 이동한 후에, 프로젝트를 빌드할 수 있다.

```shell
> cargo build
```

cargo 는 src 프로젝트 안의 파일들을 자동으로 빌드한다. 빌드한 파일들은 `./target`디렉토리로 나온다. 이때, `Cargo.lock`파일 도 같이 생성되는데, 패키지들의 의존성을 관리해주는 파일이다.(`package-lock.json`과 동일한듯.)

### 프로젝트 실행

프로젝트가 빌드가 되면, 다음 명령어로 프로젝트를 실행할 수 있다.

```shell
> cargo run
```

### 프로젝트 체크

프로젝트를 빌드하지 않고, 빌드가 되는지 여부만 확인할 수 있다.

```shell
> cargo check
```

### 프로젝트 릴리즈

```shell
> cargo build --release
```

## 외부 프로젝트 실행

```shell
> git clone [source]
> cd ./target-proj
> cargo build
```
