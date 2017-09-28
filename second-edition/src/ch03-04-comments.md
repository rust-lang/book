## 주석

모든 프로그래머들은 되도록 이해하기 쉽게 이해되는 코드를 작성하기 위해 노력하지만, 자주 부연 설명이
필요합니다. 이런 경우, 프로그래머들은 메모를 남기거나 소스코드에 컴파일러는 무시하도록 되어 있는 *주석*
을 남겨 소스코드를 읽는 사람이 혜택을 받을 수 있게 합니다.  

여기에 간단한 주석이 있습니다:

```rust
// Hello, world.
```

Rust에서 주석은 두개의 슬래쉬로 시작해야 하고 해당 줄의 끝까지 계속됩니다. 한 줄을 넘는 주석을 작성할 경우, 
`//`를 각 줄에 포함시켜 사용하면 됩니다, 이런 식으로요:

```rust
// 우리는 여기에 뭔가 복잡한 것을 적어놓고자 하는데, 그를 위해 충분히 긴 여러 줄의 주석이 필요합니다. 
// 휴! 다행입니다.
// 이 주석은 그에 대해 설명할테니까요.
```

주석은 코드의 뒷 부분에 위치할 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

하지만 주석을 코드와 나눠 앞 줄에 기재되는 형식을 더 자주 보게 될 겁니다. 

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

이게 전부입니다. 특별히 복잡하지 않죠.
