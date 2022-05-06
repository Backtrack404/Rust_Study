# 1_Hello Cargo 

## Cargo를 이용하여 프로젝트 생성 

``` terminal
❯ cargo new [Project Name] --bin
```

cargo new 옵션 

``` terminal
❯ cargo new --help
```

</br>
</br>
</br>

Filename : Cargo.toml

```toml
[package]
name = "HelloCargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Cargo의 환경설정 포멧 

첫번째 라인 [package]은 이후의 문장들이 패키지 환경설정이라는 것을 나타내는 섹션의 시작점 

</br>

그 다음 세 라인들은 Cargo가 프로그램을 컴파일 하기 위해 필요로 하는 정보에 대한 설정.

이름 , 버전, 누가 작성했는가.

</br>

마지막 라인 [dpendencies]은 프로젝트의 의존성 리스트.

러스트에선 코드의 패키지를 크레이트 라고 부름 

</br>
</br>
</br>

Filename : src/main.rs 

``` rs
fn main() {
    println!("Hello, world!");
}
```

Cargo가 코드를 src 디렉터리 안에 위치시고, 최상위 디렉터리에 Cargo.toml환경 파일을 가지게 해줌.

Cargo를 사용하지 않고 프로젝트를 시작했다면 Cargo를 사용한 프로젝트로 바꿀 수 있다. 

프로젝트 코드를 src 디렉터리로 옮기고 적합한 Cargo.toml파일을 생성. 

</br>
</br>
</br>

## Cargo 프로젝트를 빌드하고 실행하기 

프로젝트 빌드

``` terminal
❯ cargo build
   Compiling HelloCargo v0.1.0 (/home/PromotionPawn/Project/Rust/HelloCargo)
warning: crate `HelloCargo` should have a snake case name
```

커맨드를 실행하면 ./target/debug/[Project Name] 에 실행파일 생성

</br>

실행파일 실행

```cargo
❯ ./target/build/[Project Name]
```

cargo build를 실행하게 되면 최상위 디렉터리에 Cargo.lock이라는 새로운 파일 생성.

이 파일은 어떠한 의존성도 가지고 있지 않으며 Cargo가 파일의 내용 관리 

</br>

컴파일 후 바로 프로그램 실행

```terminal
❯ cargo run
```

두번째 컴파일시 내용이 변경되지 않아 컴파일 하지 않고 바로 실행.

수정했다면 자동으로 컴파일하고 실행

</br>
</br>
</br>

## 에러 체크 

Cargo는 cargo check를 통해 코드가 컴파일 되는지 체크. 

별도의 생행파일 생성은 하지 않음.

```terminal
❯ cargo check
```

cargo check는 cargo build와 달리 실행파일을 생성하는 단계를 건너뛰기 때문에 훨씬 빠름.

코드 작성하는 동안 계속적으로 작업물을 감시하는 중이라면 cargo check를 이용하는것이 좋음 

</br>
</br>
</br>

## 릴리즈 빌드 

프로젝트가 배포를 위한 준비가 되었다면 

``` terminal
❯ cargo build --release
```

를 사용하여 최적화와 함께 컴파일 가능.

이 커맨드는 ./target/debug 대신 ./target/release에 실행파일 생성.

