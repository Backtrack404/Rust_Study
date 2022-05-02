# Guessing Game

## 새로운 프로젝트 준비하기 

1. 프로젝트 생성 

``` terminal
❯ cargo new GuessingGame --bin
     Created binary (application) `GuessingGame` package
```

첫 명령문인 cargo new 는 프로젝트 이름 (GuessingGame)을 첫번째 인자로 받으며 --bin 플래그를 통해 바이너리용 프로젝트를 생성하도록 함


</br>
</br>
</br>

## 추리값 처리하기

사용자가 추리한 값을 입력받아 그대로 출력하는 코드

``` rs
use std::io;

fn main() {
    println!("숫자를 추리해 보세요!");

    println!("추리한 숫자를 입력해 주세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("읽기 실패");

    println!("추리한 값: {}", guess);
}

```

</br>

코드 분석 

``` rs
use std::io;
```

사용자가 입력받고 결과값을 표시하기 위해서는 io 라이브러리를 스코프로 가져와야 함.

io 라이브러리는 std라고 불리는 표준 라이브러리에 있음

러스트는 모든 프로그램의 스코드에 prelude 내의 타입들을 가져옴.

만약 원하는 타입이 prelude에 없다면 use문을 사용해 명시적으로 그 타입을 가져와야함.

std::io는 사용자의 입력을 받는것을 포함해 io와 관련된 기능들을 제공.

</br>
</br>
</br>

``` rs
fn main(){
```

main함수는 프로그램의 진입점이고 fn문법은 새로운 함수를 선언

</br>
</br>
</br>

```rs
println!{"숫자를 추리해 보세요!"};

println!{"추리한 숫자를 입력해 주세요."};
```

println!는 string을 화면에 표시하는 매크로 

</br>
</br>
</br>

## 값을 변수에 저장하기 

다음코드로 사용자의 입력값을 저장할 공간을 생성 

``` rs
let mut guess = String::new();
```

let의 예제

```rs
let foo = bar;
```

위 코드는 foo라는 변수를 선언하고 bar라는 값과 묶음.

러스트에서 변수는 기본적으로 불변.

아래 예시는 mut 을 이용하여 가변변수를 만드는 예제

```rs
let foo = 5; // 불변 

let mut bar = 5; // 가변
```

</br>

따라서 let mut guess가 가변변수임을 알 수 있음.

= 의 반대면의 값은 guess와 묶이게 되는데 위 예시에서는 함수 String::new의 결과값인 새로운 String인스턴스가 묶이는 대상

String은 표준 라이브러리에서 제공하는 확장 가능한 UTF-8 인코딩의 문자열 타입 

</br>

::new 에 있는 ::는 new가 String 타입의 연관함수 임을 나타냄. 연관함수는 하나의 타입을 위한 함수이며, 이 경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수. 즉 정적 메소드 

</br>

new 함수는 새로운 빈 String을 생성. 

new 함수는 새로운 값을 생성하기 위한 일반적인 이름 

</br>

요약

let mut guess = String::new(); 

는 새로운 빈 String인스턴스와 연결된 가변변수를 생성

</br>
</br>
</br>

프로그램에 첫번째 라인에 use std::io; 를 이용하여 표준 라이브러리의 input/output을 포함. 

따라서 io의 연관함수인 stdin을 호출.

``` rs
io::stdin().read_line(&mut guess).expect("읽기 실패")
```

``` rs
io::stdin()
```

만약 프로그램 시작점에 use std::io가 없다면 함수 호출시 

std::io::stdin 처럼 작성해야 함

</br>


``` rs
.read_line(&mut guess)
```

.read_line(&mut guess)는 사용자로 부터 입력받기 위해 표준 입력 핸들에서 .read_line(&mut guess) 메소드를 호출. 

또한 read_line에 &mut guess를 인자로 하나 넘김.

</br>

read_line은 사용자가 표준 입력에 입력할 때 마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요. 

그 문자열 인자는 사용자 입력을 추가하면서 변경되므로 가변적이여야 함

</br>

&는 코드의 여러 부분에서 데이털르 여러번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자.

참조자는 복잡한 특성으로서 러스트의 큰 이점 중 하나가 참조자를 사용함으로써 얻는 안전성과 용이성.

참조자는 변수처럼 기본적으로 불변.

따라서 가변으로 바꾸기 위해 &guess가 아닌 

&mut guess로 작성 

</br>
</br>
</br>

## Result 타입으로 잠재된 실패 다루기.

``` rs
.expect("읽기 실패");
```

read_line은 인자로 넘긴 문자열에 사용자가 입력을 저장할 뿐 아니라 하나의 값을 돌려준다. 

여기서 돌려준 값은 io::Result 이다.

러스트는 표준 라이브러리에 여러 종류의 Result타입을 가지고 있다. 제네릭 Result나 io:Result가 예.

Result 타입은 열거형(enums)으로 열거형은 정해진 값들만 가질 수 있으며 이러한 값들은 열거형의 variant라고 부른다. 

Result의 variants는 Ok와 Err 이며 
Ok는 처리 성공을 의미고 내부적으로 성공적으로 생성된 결과를 가지고 있으며 Err는 처리 실패를 의미하고 내부적으로 그 이유에 대한 정보를 가지고 있다.

이러한 Result는 에러 처리를 위한 정보를 표현하기 위해 사용되며 Result타입의 값들은 다른 타입들 처럼 메소드들을 가지고 있다.

io:Result 인스턴스는 expect 메소드를 가지고 있고 만약 io:Result 인스턴스가 Err일 경우 expect 메소드는 프로그램이 작동을 멈추게 하고 expect에 인자로 넘겼던 메세지를 출력 하도록 한다.

만약 read_line 메소드가 Ok값이라면 ecpect는 Ok가 가지고 있는 결과값을 돌려주어 사용할 수 있도록 한다.

만약 expect를 호줄하지 않는다면 컴파일은 되지만 경고 나타난다.

``` rs
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

러스트는 read_line이 돌려주는 Result값을 사용하지 않았음을 경고하며 일어날 수 있는 에러를 처리하지 않았음을 알려줌. 이 경고를 없애는 옳은 방법은 에러를 처리하는 코드를 작성하는 것이지만 만약 문제가 발생했을 때 프로그램이 멈추길 발한다면 expect를 사용할 수 있다.

</br>
</br>
</br>

## println! 변경자(placeholder)를 이용한 값 출력

``` rs
println!("추리한 값: {}", guess);
```

이 라인은 사용자가 입력한 값을 저장한 문자열을 출력한다.

{}는 변경자로써 값이 표시되는 위치를 나타낸다. 

{}를 이용하여 하나 이상의 값을 표시할 수 도 있다. 아래는 그 예시 이다.

```rs
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

위 코드는 x = 5 and y = 10 을 출력한다. 

</br>


첫번째 부분 실행 

``` terminal
❯ cargo run
   Compiling GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 4.21s
     Running `target/debug/GuessingGame`
숫자를 추리해 보세요!
추리한 숫자를 입력해 주세요.
10
추리한 값: 10
```

</br>
</br>
</br>
</br>
</br>
</br>

## 비밀번호 생성하기 

추리를 위한 비밀번호를 생성하기 위해 러스트 팀에서 제공하는 rand 크레이트를 상요한다.

</br>

## 크레이트(Crate)를 사용하여 더 많은 기능 가져오기

크레이트는 러스트 코드의 묶음(package)이다.

현재 프로젝트는 실행이 가능한 binary crate이다. 

rand crate는 다른 프로그램에서 사용되기 위한 library crate 이다.

</br>
</br>

rand 를 사용하기 위해 Cargo.toml을 수정해 rand 크레이트를 의존 리스트에 추가한다. 

[dependencies] 절의 시작 바로 아래에 추가한다.

Filename : Cargo.toml

```toml
[dependencies]

rand = "0.3.14"
```

rand 크레이트 버전을 명시.
Cargo는 버전을 명시하는 표준에 해당하는 Semantic Versioning(semver)을 이용 0.3.14는 ^0.3.14의 축약형이 되며 이는 버전 0.3.14와 호환되는 API를 제공하는 모든 버전임을 의미 

</br>

코드 수정없이 빌드

``` terminal
❯ cargo build
    Updating crates.io index
  Downloaded rand v0.3.23
  Downloaded libc v0.2.125
  Downloaded rand v0.4.6
  Downloaded 3 crates (677.0 KB) in 0.66s
   Compiling libc v0.2.125
   Compiling rand v0.4.6
   Compiling rand v0.3.23
   Compiling GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1m 19s
```

이로 인해 외부 의존성을 가지고 Cargo는 Crates.io데이터의 복사본인 레지스트리에서 모든것을 가져온다.

Crates.io는 러스트의 생태계의 개발자들이 다른 사람들도 이용할 수 있도록 러스트 오픈소스를 공개하는 곳이다.

레지스트리를 업데이트하면 Cargo는 [dependencies] 절을 확인하고 아직 가지고 있지 않은 것들을 다운로드 한다.

이 경우 rand만 의존한다고 명시했으나 rand는 libc에 의존하기 때문에 libc도 다운받고 다운 받은 후 컴파일 하여 의존성이 해결된 프로젝트를 컴파일한다.

</br>
</br>
</br>

## 재현 가능한 빌드를 보장하는 Cargo.lock

</br>

Cargo는 다른 누구라도 같은 코드를 빌드할 경우 같은 산출물이 나오도록 보장하는 방법을 가지고 있다.

Cargo는 다른 의존성을 추가하기 전까지 명시한 의존 패키지만을 사용한다.

예로 rand크레이트의 다음 버전인 v0.3.15에서 중요한 결함이 고쳐졌지만 코드를 방치는 변경점이 있다면 Cargo.lock 때문에 명시적으로 업그레이드 하지 않는 이상 0.3.14를 사용 


</br>
</br>
</br>

## 크레이트를 새로운 버전으로 업그레이드 하기 

크레이트를 업데이트 하고 싶다면 cargo update 명령어를 통해 업데이트 가능.

명령어는 Cargo.lock파일을 무시하고 Cargo.toml에 명시한 요구사항이 맞는 최신버전을 확인. 

만약 이 버전들로 문제가 없다면 Cargo는 해당 버전을 Cargo.lock에 기록한다.

하지만 Cargo는 기본적으로 0.3.0보다 크고 0.4.0보다 작은 버전을 찾는다. 

만약 rand크레이트가 새로운 두 개의 버전인 0.3.15와 0.4.0을 릴리즈 했다면 cargo update를 실행했을 때 다음의 메시지는 내보낸다.

``` terminal
❯ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.1
```

이 시점에 Cargo.lock파일에서 변경이 일어난 것과 앞으로 사용될 rand 크레이트의 버전이 0.3.15임을 확인 할 수 있다.

</br>

만약 0.4.0이나 0.4.x에 해당하는 모든 버전을 받고 싶다면 Cargo.toml을 다음과 같이 업데이트 해야 한다.

```toml
[dependencies]
rand = "0.4.0"
```

</br>
</br>
</br>
</br>

## 임의의 숫자 생성하기 

Filename: src/main.rs

```rs
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("비밀번호: {}", secret_number);

    println!("추리한 숫자를 입력해 주세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("읽기 실패");

    println!("추리한 값: {}", guess);
}
```

extern crate rand;를 추가해 러스트에게 외부에 존재하는 크레이트가 있음을 알린다.

이 라인은 use rand로도 표기할 수 있으며 rand::를 앞에 붙여 rand내의 모든 것을 호출 할 수 있다.

use rand::Rng의 Rng는 정수 생성기가 구현한 메소드들을 정의한 trait이며 해당 메소드들을 이용하기 위해서는 반드시 스코프 내에 있어야 한다.

</br>

rand::thread_rng함수는 OS가 시드(seed)를 정의하고 현재 스레드에서만 사용되는 특별한 정수 생성기를 돌려준다. 

다음으로 gen_range 메소드를 호출하는데 이 메소드는 Rng trait에 정의되어 있으므로 use rand:Rng문을 통해 스코프로 가져올 수 있다.

gen_range 메소드는 두 개의 숫자를 인자로 받고 두 숫자 사이에 있는 임의의 숫자를 생성한다. 

하한선은 포함되지만 상한선은 제외되므로 1부터 100사이의 숫자를 생성하려면 1과 101을 넘겨야 한다.

</br>

cargo doc --open명령어로 로컬에서 모든 의존 패키지들이 제공하는 문서들을 빌드해서 브라우저에 표시해준다. 

만약 rand 크레이트의 다른 기능들에 흥미가 있다면 cargo doc --open을 실행하고 왼쪽 사이드바에 rand를 클릭하면 된다.

``` terminal
❯ cargo doc --open
    Checking libc v0.2.125
    Checking rand v0.4.6
    Checking rand v0.3.23
 Documenting GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
    Finished dev [unoptimized + debuginfo] target(s) in 5.39s
     Opening /home/PromotionPawn/Project/Rust/GuessingGame/target/doc/GuessingGame/index.html
```

</br>
</br>
</br>

코드에 추가한 두 번째 라인은 비밀번호를 표시한다.

``` terminal
❯ cargo run
   Compiling GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.87s
     Running `target/debug/GuessingGame`
숫자를 추리해 보세요!
비밀번호: 13
추리한 숫자를 입력해 주세요.
13
추리한 값: 13
```

</br>
</br>
</br>
</br>

## 비밀번호와 추리값 비교

Filename : src/main.rs

``` rs
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("비밀번호: {}", secret_number);

    println!("추리한 숫자를 입력해 주세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("읽기 실패");

    println!("추리한 값: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
```

표준 라이브러리로부터 std::cmp::Ordering을 스코프로 가져오는 또다른 use이다.

Ordering은 Result와 같은 열거형이지만 Ordering의 값은 Less, Greater, Equal 이다.

```rs
match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
```

cmp 메소도는 두 값을 비교하며 비교 가능한 모든 것들에 대해 호출할 수 있다.

이 메소드는 비교하고 싶은 모든 것들의 참조자를 받는다.

여기서는 guess와 secret_number를 비교하고 있다.

cmp는 Ordering 열거형을 돌려준다. 

match 표현문을 이용하여 cmp가 guess와 secret_number를 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정할 수 있다.

match 표현식은 arm으로 이루어져 있으며 arm은 하나의 패턴과 match 표현식에서 주어진 값이 패턴과 맞는다면 실행할 코드로 이루어져 있다.

러스트는 match에게 주어진 값을 arm의 패턴에 맞는지 순서대로 확인한다.

match 생성자와 패턴들은 코드가 마주칠 다양한 상황을 표현할 수 있도록하고 모든 경우의 수를 처리했음을 확신할 수 있도록 도와주는 강력한 특성들이다.

</br>

하지만 위 코드는 컴파일되지 않는다

``` terminal
❯ cargo run
   Compiling GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number){
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

error[E0283]: type annotations needed for `{integer}`
   --> src/main.rs:10:44
    |
10  |     let secret_number = rand::thread_rng().gen_range(1,101);
    |         -------------                      ^^^^^^^^^ cannot infer type for type `{integer}`
    |         |
    |         consider giving `secret_number` a type
    |
    = note: multiple `impl`s satisfying `{integer}: SampleRange` found in the `rand` crate:
            - impl SampleRange for i16;
            - impl SampleRange for i32;
            - impl SampleRange for i64;
            - impl SampleRange for i8;
            and 6 more
note: required by a bound in `gen_range`
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/lib.rs:524:34
    |
524 |     fn gen_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T where Self: Sized {
    |                                  ^^^^^^^^^^^ required by this bound in `gen_range`
help: consider specifying the type argument in the function call
    |
10  |     let secret_number = rand::thread_rng().gen_range::<T>(1,101);
    |                                                     +++++

Some errors have detailed explanations: E0283, E0308.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `GuessingGame` due to 2 previous errors
```

위 에러의 핵심은 일치하지 않는 타입이 있다고 알려준다.

러스트는 강한 정적 타입 시스템을 가지고 있지만 타입 추론도 수행한다. 

만약 let guess = String::new()를 작성한다면 러스트는 guess가 String 타입이여야 함을 추론할 수 있으므로 타입을 적으라고 하지 않는다.

반대로 secret_numbe()는 정수형 이다. 

i32 는 32비트 정수, 

u32 는 32비트 부호없는 정수,

i64 는 64비트 정수이며 그 외에 도 비슷하다.

러스트는 기본적으로 우리가 다른 정수형임을 추론할 수 있는 다른 타입 정보를 제공하지 않는다면 숫자들을 i32로 생각한다. 이 에러의 원인은 러스트가 문자열과 정수형을 비교하지 않았기 때문이다.

최종적으로 추리값을 정수형으로 비교하기 위해 입력받은 String을 정수로 바꿔야 한다.

</br>
</br>
</br>

Filename : src/main.rs

```rs
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("비밀번호: {}", secret_number);

    println!("추리한 숫자를 입력해 주세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("읽기 실패");

    let guess: u32 = guess.trim().parse()
    .expect("정수를 입력해 주세요!");

    println!("추리한 값: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
```

</br>

두 라인은 다음과 같다.

```rs
let guess: u32 = guess.trim().parse()
    .expect("정수를 입력해 주세요!");
```

위에서 guess 변수를 이미 생성했지만 러스트는 이전에 있던 guess 값을 가리는(shadow)것을 허락한다.

이 특징은 종종 하나의 값을 현재 타입에서 다른 타입으로 변환하고 싶을 경우에 사용한다.

Shadowing은 우리들이 guess_str과 guess처럼 고유의 변수명을 만들도록 강요하는 대신 guess를 재사용 가능하도록 한다.

</br>

여기서 guess를 guess.trim().parse() 표현식과 묶는다. 

표현식 내의 guess는 입력값을 가지고 있던 String을 참조한다.

String 인스턴스의 trim 메소드는 처음과 끝 부분의 빈칸을 제거한다. 

u32는 정수형 글자만을 가져야 하지만 사용자들은 read_line을 끝내기 위해 ender키를 반드시 눌러야 하는데 이때 개행문자(\n)가 추가된다. 

trim 메소드는 \n을 제거하고 숫자만 남도록 처리한다.

문자열의 parse 메소드는 문자열을 숫자형으로 파싱한다. 

이 메소드는 다양한 종류의 정수형을 변환하므로 let guess: u32 처럼 정확한 타입을 명시해야 한다.

guess 뒤의 콜론(:)은 변수의 타입을 명시했음을 의미한다. 

</br>

parse메소드의 호출은 에러가 발생하기 쉬우므로 Result타입으로 잠재된 실패 다루기에서 read_line과 비슷하게 parse 메소드는 실패할 경우를 위해 Result 타입을 결과로 돌려준다. 만약 parse 메소드가 문자열에서 정수로 파싱을 실패하여 Err Result variant를 돌려준다면 expect호출은 게임을 멈추고 우리가 명시한 메시지를 출력한다. 

</br>

프로그램 실행 

``` terminal
❯ cargo run
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/GuessingGame`
숫자를 추리해 보세요!
비밀번호: 55
추리한 숫자를 입력해 주세요.
55
추리한 값: 55
You win!
```

</br>
</br>
</br>
</br>

## 반복문을 이용하여 여러번의 추리 허용 

loop 키워드는 무한루프를 제공한다. 

Filename: src/main.rs

```rs
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("비밀번호: {}", secret_number);

    loop {
        println!("추리한 숫자를 입력해 주세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("읽기 실패");

        let guess: u32 = guess.trim().parse()
        .expect("정수를 입력해 주세요!");

        println!("추리한 값: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!")
        }
    }
}
```

</br>

위 코드는 무한루프에 빠져 답을 맞춰도 프로그램이 종료되지 않는다. 

```terminal
❯ cargo run
   Compiling GuessingGame v0.1.0 (/home/PromotionPawn/Project/Rust/GuessingGame)
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/GuessingGame`
숫자를 추리해 보세요!
비밀번호: 17
추리한 숫자를 입력해 주세요.
1
추리한 값: 1
Too small!
추리한 숫자를 입력해 주세요.
17
추리한 값: 17
You win!
추리한 숫자를 입력해 주세요.
0
추리한 값: 0
Too small!
추리한 숫자를 입력해 주세요.
quit
thread 'main' panicked at '정수를 입력해 주세요!: ParseIntError { kind: InvalidDigit }', src/main.rs:22:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

문자를 입력하게 되면 .expect 에 걸려 프로그램이 종료된다.


</br>
</br>
</br>
</br>

## 정답 이후에 종료하기 

사용자가 정답을 맞췄을 때 게임이 종료되도록 break문을 추가한다. 

Filename: src/main.rs

```rs
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("비밀번호: {}", secret_number);

    loop {
        println!("추리한 숫자를 입력해 주세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("읽기 실패");

        let guess: u32 = guess.trim().parse()
        .expect("정수를 입력해 주세요!");

        println!("추리한 값: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

break문을 You win! 이후에 추가하여 사용자가 비밀번호를 맞췄을 때 프로그램이 반복문을 끝내도록 한다. 

반복문이 main의 마지막 부분이르모 반복문의 종료는 프로그램의 종료를 의미한다. 

</br>
</br>
</br>
</br>

## 잘못된 입력값 처리하기

사용자가 숫자가 아닌 값을 입력했을 때 프로그램이 종료되는 동작을 더 다음어 숫자가 아닌 입력은 무시하여 사용자가 계속 입력할수 있도록 하기위해 guess가 String에서 u32로 변환되는 라인을 수정하면 된다.

```rs
let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
```

expect 메소드 호출을 match표현식으로 바꾸는 것은 에러 발생 시 종료에서 처리로 바꾸는 일반적인 방법이다. 

parse메소드가 Result 타입을 돌려주는 것과 Result는 Ok나 Err variants를 가진 열거형임을 떠올리면 알 수 있다. 

만약 parse가 성공적으로 문자열에서 정수로 변환했다면 결과값을 가진 Ok를 돌려준다. Ok는 첫번째 arm의 패턴과 매칭하게 되고 match 표현식은 parse가 생성한 num 값을 돌려준다. 그 값은 위에 생성한 새로운 guess값고 묶이게 된다.

만약 parse가 문자열을 정수로 바꾸지 못했다면 에러 정보를 가진 Err를 돌려준다. Err는 첫번째 arm의 패턴인 Ok(num)과 매칭하지 않지만 두 번째 arm의 Err(_)와 매칭한다.

_는 모든 값과 매칭될 수 있다.

위 코드에서는 Err내에 무슨 값이 있던지 간에 관계없이 모든 Err에 매칭되도록 했다. 따라서 프로그램은 두 번째 arm의 코드인 continue를 실행하며, 이는 loop의 다음 반복으로 가서 또 다른 추리값을 요청하도록 한다. 효율적으로 프로그램은 parse에서 가능한 모든 에러를 무시한다. 

</br>
</br>
</br>

프로그램 실행 

```terminal
❯ cargo run
warning: crate `GuessingGame` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `guessing_game`

warning: `GuessingGame` (bin "GuessingGame") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/GuessingGame`
숫자를 추리해 보세요!
비밀번호: 61
추리한 숫자를 입력해 주세요.
d
추리한 숫자를 입력해 주세요.
cargo
추리한 숫자를 입력해 주세요.
1
추리한 값: 1
Too small!
추리한 숫자를 입력해 주세요.
100
추리한 값: 100
Too big!
추리한 숫자를 입력해 주세요.
61
추리한 값: 61
You win!

❯ 
```

</br>
</br>
</br>

아직 비밀번호가 출력되므로 이를 수정하면 최종 코드가 완성된다.

```rs
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("추리한 숫자를 입력해 주세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("읽기 실패");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("추리한 값: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

</br>
</br>
</br>
</br>
