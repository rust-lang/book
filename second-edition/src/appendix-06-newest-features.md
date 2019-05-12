# 부록 F: 새로운 기능

이번 부록에선 본 책의 주요 내용이 작성되고 난 이후에
러스트 stable 에 추가된 몇 가지 기능을 다룹니다.


## 더 짧은 필드 초기화

자료 구조(구조체, 열거형, union)에서 필드명을 갖는 필드를 초기화할 때
`fieldname: fieldname` 을 `fieldname` 으로 줄여서 쓸 수 있습니다.
이 기능은 초기화 구문을 간결하게 만들어 코드 중복을 줄여줍니다.

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // 전체 구문
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // 단축된 필드 초기화 구문
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```


## loop 에서 반환하기

`loop` 는 특정 스레드가 작업을 완료했는지 알아보는 등, 어떤 연산이
실패할 수도 있다는 것을 알고 있을 때, 해당 연산을 재시도 하는 데
사용 가능합니다. 이 연산 결과를 다른 코드로 넘겨 주어야 한다면,
`break` 를 이용해 반복을 멈추고 결과를 반환할 수 있습니다:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

## 중복된 `use` 선언 합치기

모듈을 여러 하위 모듈이 이루고 있어
구조가 복잡한 모듈에서 몇 개의 모듈만 가져와야 할 때,
선언 속 중복되는 부분을 합칠 수 있다면
코드를 깔끔하게 만들 수 있을겁니다.

`use` 선언은 간결한 임포트 및 글롭에서 합칠 수 있습니다.
다음은 `bar` 과 `Foo`, 그리고 `baz`와 `Bar` 내
모든 항목을 가져오는 예시입니다:

```rust
# #![allow(unused_imports, dead_code)]
#
# mod foo {
#     pub mod bar {
#         pub type Foo = ();
#     }
#     pub mod baz {
#         pub mod quux {
#             pub type Bar = ();
#         }
#     }
# }
#
use foo::{
    bar::{self, Foo},
    baz::{*, quux::Bar},
};
#
# fn main() {}
```

## 포괄적인 범위 표현

앞서 범위 문법(`..` 와 `...` 를 말합니다)를 사용할 때,
표현식에선 상한을 포함하지 않는 `..` 를 사용하고, 패턴에선
상한을 포함하는 `...` 를 사용했습니다. 하지만 이제 `..=`
하나로 표현식과 패턴 모두에서 사용할 수 있습니다.

```rust
fn main() {
    for i in 0 ..= 10 {
        match i {
            0 ..= 5 => println!("{}: low", i),
            6 ..= 10 => println!("{}: high", i),
            _ => println!("{}: out of range", i),
        }
    }
}
```

match 내에선 `...` 를 사용해도 문제는 없지만 표현식에선
사용할 수 없으니 `..=` 를 권장합니다.

## 128 비트 정수

128 비트 정수가 러스트 1.26.0 에 추가됐습니다:

- `u128`: 부호가 없으며 [0, 2^128 - 1] 범위를 갖는 128 비트 정수
- `i128`: 부호가 있으며 [-(2^127), 2^127 - 1] 범위를 갖는 128 비트 정수

이들은 LLVM 을 통해 효율적으로 구현됐기 때문에,
128 비트 정수를 지원하지 않는 플랫폼에서도
다른 정수 타입들과 마찬가지 방식으로 사용 가능합니다.

이 기능은 암호화 알고리즘 등, 아주 큰 정수를 효율적으로
다뤄야 하는 알고리즘에서 유용할 겁니다.
