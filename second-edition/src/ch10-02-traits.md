## 트레잇: 공유 동작을 정의하기

트레잇은 다른 종류의 추상화를 사용할 수 있도록 해줍니다: 이는 타입들이 공통적으로 갖는 동작에 대하여
추상화하도록 해줍니다. *트레잇(trait)* 이란 러스트 컴파일러에게 특정한 타입이 갖고 다른 타입들과
함께 공유할 수도 있는 기능에 대해 말해줍니다. 우리가 제네릭 타입 파라미터를 사용하는 상황에서는,
컴파일 타임에 해당 제네릭 타입이 어떤 트레잇을 구현한 타입이어야 함을 명시하여, 그러한 상황에서
우리가 사용하길 원하는 동작을 갖도록 하기 위해 *트레잇 바운드(trait bounds)* 를 사용할 수
있습니다.

> 노트: *트레잇*은 다른 언어들에서 '인터페이스(interface)'라고 부르는 기능과 유사하지만,
> 몇 가지 다른 점이 있습니다.

### 트레잇 정의하기

어떤 타입의 동작은 우리가 해당 타입 상에서 호출할 수 있는 메소드들로 구성되어 있습니다. 만일 우리가
서로 다른 타입에 대해 모두 동일한 메소드를 호출할 수 있다면 이 타입들은 동일한 동작을 공유하는 것입니다.
트레잇의 정의는 어떠한 목적을 달성하기 위해 필요한 동작의 집합을 정의하기 위해 메소드 시그니처들을 함께
묶는 방법입니다.

예를 들면, 다양한 종류와 양의 텍스트를 갖는 여러 가지의 구조체를 가지고 있다고 칩시다:
`NewsArticle` 구조체는 세계의 특정한 곳에서 줄지어 들어오는 뉴스 이야기를 들고 있고, `Tweet`은
최대 140글자의 콘텐츠와 함께 해당 트윗이 리트윗인지 혹은 다른 트윗에 대한 답변인지와 같은 메타데이터를
가지고 있습니다.

우리는 `NewsArticle` 혹은 `Tweet` 인스턴스에 저장되어 있을 데이터에 대한 종합 정리를 보여줄 수
있는 미디어 종합기 라이브러리를 만들고 싶어 합니다. 각각의 구조체들이 가질 필요가 있는 동작은 정리해주기가
되어야 하며, 그래서 각 인스턴스 상에서 `summary` 메소드를 호출함으로써 해당 정리를 얻어낼 수 있어야
한다는 것입니다. Listing 10-11은 이러한 개념을 표현한 `Summarizable` 트레잇의 정의를 나타냅니다:


<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}
```

<span class="caption">Listing 10-11: `summary` 메소드에 의해 제공되는 동작으로 구성된
`Summarizable` 트레잇의 정의</span>

`trait` 키워드 다음 트레잇의 이름, 위의 경우 `Summarizable`을 써서 트레잇을 선언했습니다.
중괄호 내에서는 이 트레잇을 구현하는 타입들이 가질 필요가 있는 동작들을 묘사한 메소드 시그니처들을
정의했는데, 위의  경우에는 `fn summary(&self) -> String`입니다. 메소드 시그니처 뒤에,
중괄호 내의 정의부를 제공하는 대신, 세미콜론을 집어넣었습니다. 그러면 이 트레잇을 구현하는 각
타입은 이 메소드의 본체에 대한 해당 타입 고유의 커스텀 동작을 제공해야 하는데, 컴파일러는
`Summarizable` 트레잇을 갖는 어떠한 타입이든 그에 대한 메소드 `summary`를 정확히 동일한
시그니처로 정의되도록 강제할 것입니다.

트레잇은 한 줄 당 하나의 메소드 시그니처와 각 줄의 끝에 세미콜론을 갖도록 함으로써, 본체 내에 여러
개의 메소드를 가질 수 있습니다.

### 특정 타입에 대한 트레잇 구현하기

`Summarizable` 트레잇을 정의하였으니, 이제 우리의 미디어 종합기 내에서 이 동작을 갖길 원했던 타입들
상에 이 트레잇을 구현할 수 있습니다. Listing 10-12는 `summary`의 반환 값을 만들기 위해 헤드라인,
저자, 위치를 사용하는 `NewsArticle` 구조체 상의 `Summarizable` 트레잇 구현을 보여줍니다.
`Tweet` 구조체에 대해서는, 트윗 내용이 이미 140자로 제한되어 있음을 가정하고, `summary`를 정의하는
데 있어 사용자 이름과 해당 트윗의 전체 텍스트를 가지고 오는 선택을 했습니다.

<span class="filename">Filename: lib.rs</span>

```rust
# pub trait Summarizable {
#     fn summary(&self) -> String;
# }
#
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<span class="caption">Listing 10-12: `NewsArticle`과 `Tweet` 타입 상에서의
`Summarizable` 트레잇 구현</span>

어떤 타입 상에서의 트레잇 구현은 트레잇과 관련이 없는 메소드를 구현하는 것과 유사합니다. 다른 점은
`impl` 뒤에 우리가 구현하고자 하는 트레잇 이름을 넣고, 그다음 `for`와 우리가 트레잇을 구현하고자
하는 타입의 이름을 쓴다는 것입니다. `impl` 블록 내에서는 트레잇 정의부가 정의한 바 있는 메소드
시그니처를 집어넣지만, 각 시그니처의 끝에 세미콜론을 집어넣는 대신 중괄호를 넣고 우리가 트레잇의
메소드가 특정한 타입에 대해서 갖기를 원하는 특정한 동작으로 메소드의 본체를 채웁니다.


트레잇을 한번 구현했다면, 트레잇의 일부가 아닌 메소드들을 호출했던 것과 동일한 방식으로
`NewsArticle`과 `Tweet`의 인스턴스 상에서 해당 메소드들을 호출할 수 있습니다:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

이 코드는 `1 new tweet: horse_ebooks: of course, as you probably already
know, people`를 출력할 것입니다.

Listing 10-12에서 `Summarizable` 트레잇과 `NewsArticle` 및 `Tweet` 타입을 동일한
`lib.rs` 내에 정의했기 때문에, 이들이 모두 동일한 스코프 내에 있다는 점을 주목하세요. 만일
이 `lib.rs`가 `aggregator`라고 불리는 크레이트에 대한 것이고 누군가가 우리의 크레이트 기능에
더해 그들의 `WeatherForecast` 구조체에 대하여 `Summarizable`을 구현하기를 원한다면, 그들의
코드는 Listing 10-13과 같이 이를 구현하기 전에 먼저 `Summarizable` 트레잇을 그들의 스코프로
가져올 필요가 있습니다:

<span class="filename">Filename: lib.rs</span>

```rust,ignore
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```

<span class="caption">Listing 10-13: 우리의 `aggregator` 크레이트로부터
다른 크레이트 내의 스코프로 `Summarizable` 트레잇을 가져오기</span>

이 코드는 또한 `Summarizable`이 공개 트레잇임을 가정하는데, 이는 Listing 10-11에서 `trait`
전에 `pub` 키워드를 집어넣었기 때문입니다.

트레잇 구현과 함께 기억할 한 가지 제한사항이 있습니다: 트레잇 혹은 타입이 우리의 크레이트 내의 것일
경우에만 해당 타입에서의 트레잇을 정의할 수 있습니다. 바꿔 말하면, 외부의 타입에 대한 외부 트레잇을
구현하는 것은 허용되지 않습니다. 예를 들어, `Vec`에 대한 `Display` 트레잇은 구현이 불가능한데,
`Display`와 `Vec` 모두 표준 라이브러리 내에 정의되어 있기 때문입니다. 우리의 `aggregator`
크레이트 기능의 일부로서 `Tweet`과 같은 커스텀 타입에 대한 `Display`와 같은 표준 라이브러리
트레잇을 구현하는 것은 허용됩니다. 또한 우리의 `aggregator` 크레이트 내에서 `Vec`에 대한
`Summarizable`을 구현하는 것도 가능한데, 이는 우리 크레이트 내에 `Summarizable`이 정의되어
있기 때문입니다. 이러한 제한은 *고아 규칙(orphan rule)* 이라고 불리는 것의 일부인데, 이는
타입 이론에 흥미가 있다면 찾아볼 수 있습니다. 간단하게 말하면, 부모 타입이 존재하지 않기 때문에
고아 규칙이라고 부릅니다. 이 규칙이 없다면, 두 크레이트는 동일한 타입에 대해 동일한 트레잇을
구현할 수 있게 되고, 이 두 구현체가 충돌을 일으킬 것입니다: 러스트는 어떤 구현을 이용할 것인지
알지 못할 것입니다. 러스트가 고아 규칙을 강제하기 때문에, 다른 사람의 코드는 여러분의 코드를
망가뜨리지 못하고 반대의 경우도 마찬가지입니다.

### 기본 구현

종종 모든 타입 상에서의 모든 구현체가 커스텀 동작을 정의하도록 하는 대신, 트레잇의 몇몇 혹은 모든
메소드들에 대한 기본 동작을 갖추는 것이 유용할 수 있습니다. 특정한 타입에 대한 트레잇을 구현할 때,
각 메소드의 기본 동작을 유지하거나 오버라이드(override)하도록 선택할 수 있습니다.

Listing 10-14는 우리가 Listing 10-11에서 한 것과 같이 메소드 시그니처를 정의만 하는 선택
대신 `Summarizable` 트레잇의 `summary` 메소드에 대한 기본 스트링을 명시하는 선택을 하는 방법을
보여줍니다:

<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

<span class="caption">Listing 10-14: `summary` 메소드의 기본 구현을 포함한
`Summarizable` 트레잇의 정의</span>

만일 우리가 Listing 10-12에서 한 것과 같은 커스텀 구현을 정의하는 대신 `NewsArticle`의
인스턴스를 정리하기 위해 이 기본 구현을 사용하고자 한다면, 빈 `impl` 블록을 명시하면 됩니다:

```rust,ignore
impl Summarizable for NewsArticle {}
```

비록 `NewsArticle`에 대한 `summary` 메소드를 직접 정의하는 선택을 더 이상 하지 않았더라도,
`summary` 메소드가 기본 구현을 갖고 있고 `NewsArticle`이 `Summarizable` 트레잇을 구현하도록
명시했기 때문에, 우리는 여전히 `newsArticle`의 인스턴스 상에서 `summary` 메소드를 호출할 수
있습니다:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summary());
```

위의 코드는 `New article available! (Read more...)`를 출력합니다.

`Summarizable` 트레잇이 `summary` 에대한 기본 구현을 갖도록 변경하는 것은 Listing 10-12의
`Tweet`이나 Listing 10-13의 `WeatherForecast` 상에서의 `Summarizable` 구현에 대한
어떤 것도 바꾸도록 요구하지 않습니다: 기본 구현을 오버라이딩 하기 위한 문법은 기본 구현이 없는 트레잇
메소드를 구현하기 위한 문법과 정확히 동일합니다. 

기본 구현은 동일한 트레잇 내의 다른 메소드들을 호출하는 것이 허용되어 있는데, 심지어 그 다른 메소드들이
기본 구현을 갖고 있지 않아도 됩니다. 이러한 방식으로, 트레잇은 수많은 유용한 기능을 제공하면서도 다른
구현자들이 해당 트레잇의 작은 일부분만 구현하도록 요구할 수 있습니다. 우리는 `Summarizable` 트레잇이
구현이 필요한 `author_summary` 메소드도 갖도록 하여, `summary` 메소드가 `author_summary`
메소드를 호출하는 기본 구현을 갖는 형태를 선택할 수도 있습니다:

```rust
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```

이 버전의 `Summarizable`을 사용하기 위해서는, 어떤 타입에 대한 이 트레잇을 구현할 때
`author_summary`만 정의하면 됩니다:

```rust,ignore
impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
```

일단 `author_summary`를 정의하면, `Tweet` 구조체의 인스턴스 상에서 `summary`를 호출할
수 있으며, `summary`의 기본 구현이 우리가 제공한 `author_summary`의 정의부를 호출할
것입니다.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

위의 코드는 `1 new tweet: (Read more from @horse_ebooks...)`를 출력할 것입니다.

오버라이딩된 구현으로부터 기본 구현을 호출하는 것은 불가능하다는 점을 기억해주세요.

### 트레잇 바운드

이제 트레잇을 정의하고 어떤 타입들에 대해 이 트레잇을 구현해봤으니, 제네릭 타입 파라미터를 이용하는
트레잇을 사용할 수 있습니다. 우리는 제네릭 타입에 제약을 가하여 이 제네릭 타입이 어떠한 타입이든 되기
보다는, 이 제네릭 타입이 특정한 트레잇을 구현하여 이 타입들이 가지고 있을 필요가 있는 동작을 갖고
있도록 타입들로 제한함을 컴파일러가 확신하도록 할 수 있습니다.

예를 들면, Listing 10-12에서는 `NewsArticle`과 `Tweet` 타입에 대하여 `Summarizable`
트레잇을 구현했습니다. 우리는 파라미터 `item` 상에서 `summary` 메소드를 호출하는 함수 `notify`를
정의할 수 있는데, 이 `item`은 제네릭 타입 `T`의 값입니다. 에러없이 `item` 상에서 `summary`를
호출하기 위해서는, `T`에 대한 트레잇 바운드를 사용하여 `item`이 `Summarizable` 트레잇을
반드시 구현한 타입이어야 함을 특정할 수 있습니다:

```rust,ignore
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```

트레잇 바운드는 제네릭 타입 파라미터의 선언부와 함께, 꺾쇠 괄호 내에 콜론 뒤에 옵니다. `T` 상에서의
트레잇 바운드이므로, 우리는 `notify`를 호출하여 `NewsArticle`이나 `Tweet`의 어떠한 인스턴스라도
넘길 수 있습니다. 우리의 `aggregator` 크레이트를 사용하는 Listing 10-13의 외부 코드도 우리의
`notify` 함수를 호출하여 `WeatherForecast`의 인스턴스를 넘길 수 있는데, 이는
`WeatherForecast` 또한 `Summarizable`을 구현하였기 때문입니다. `String`이나 `i32` 같은
어떠한 다른 타입을 가지고 `notify`를 호출하는 코드는 컴파일되지 않을 것인데, 그 이유는 그러한
타입들이 `Summarizable`을 구현하지 않았기 때문입니다.

`+`를 이용하면 하나의 제네릭 타입에 대해 여러 개의 트레잇 바운드를 특정할 수 있습니다. 만일 함수
내에서 타입 `T`에 대해 `summary` 메소드 뿐만 아니라 형식화된 출력을 사용하길 원한다면,
트레잇 바운드 `T: Summarizable + Display`를 이용할 수 있습니다. 이는 `T`가 `Summarizable`과
`Display` 둘다 구현한 어떤 타입이어야 함을 의미합니다.

여러 개의 제네릭 타입 파라미터를 가진 함수들에 대하여, 각 제네릭은 고유의 트레잇 바운드를 가집니다.
함수 이름과 파라미터 리스트 사이의 꺾쇠 괄호 내에 많은 수의 트레잇 바운드 정보를 특정하는 것은
코드를 읽기 힘들게 만들 수 있으므로, 함수 시그니처 뒤에 `where` 절 뒤로 트레잇 바운드를 옮겨서
특정하도록 해주는 대안 문법이 있습니다. 따라서 아래와 같은 코드 대신:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

`where` 절을 이용하여 아래와 같이 작성할 수 있습니다:

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

함수 이름, 파라미터 리스트, 그리고 반환 타입이 서로 가까이 있도록 하여, 이쪽이 덜 어수선하고 이 함수의
시그니처를 많은 트레잇 바운드를 가지고 있지 않은 함수처럼 보이도록 만들어 줍니다.

### 트레잇 바운드를 사용하여 `largest` 함수 고치기

따라서 여러분이 어떤 제네릭 상에서 어떤 트레잇으로 정의된 동작을 이용하기를 원하는 어떤 경우이든,
여러분은 해당 제네릭 타입 파라미터의 타입내에 트레잇 바운드를 명시할 필요가 있습니다. 이제 우리는
Listing 10-5에서 제네릭 타입 파라미터를 사용하는 `largest` 함수의 정의를 고칠 수 있습니다!
우리가 그 코드를 치워뒀을 때, 아래와 같은 에러를 봤었지요:


```text
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

`largest`의 본체 내에서 큰 부등호 연산자를 사용하여 타입 `T`의 두 값을 비교할 수 있길 원했습니다.
이 연산자는 표준 라이브러리 트레잇인 `std::cmp::PartialOrd` 상에서 기본 메소드로 정의되어 있습니다.
따라서 큰 부등호 연산자를 사용할 수 있도록 하기 위해서는, `T`에 대한 트레잇 바운드 내에
`PartialOrd`를 특정하여 `largest` 함수가 비교 가능한 어떤 타입의 슬라이스에 대해 작동하도록
할 필요가 있습니다. `PartialOrd`는 프렐루드(prelude)에 포함되어 있기 때문에 따로 스코프 내로
가져올 필요는 없습니다.

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

이 코드를 컴파일하면, 다른 에러를 얻게 됩니다:

```text
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

이 에러에 대한 열쇠는 `cannot move out of type [T], a non-copy array`에 있습니다.
`largest` 함수의 제네릭 없는 버전에서, 우리는 고작 가장 큰 `i32` 혹은 `char`를 찾는 시도만
했습니다. 4장에서 논의한 바와 같이, 고정된 크기를 갖는 `i32`와 `char`와 같은 타입들은
스택에 저장될 수 있으며, 따라서 이 타입들은 `Copy` 트레잇을 구현하고 있습니다. 우리가 `largest`
함수를 제네릭으로 바꿨을 때, 이제는 `list` 파라미터가 `Copy` 트레잇을 구현하지 않은 타입을 가질
가능성도 생기는데, 이는 곧 `list[0]`의 값을 `largest` 변수로 소유권을 옮기지 못할 것이라는
의미입니다.

만약 이 코드를 오직 `Copy`가 구현된 타입들을 가지고 호출하도록 하는 것만 원한다면, `T`의 트레잇
바운드에 `Copy`를 추가할 수 있습니다! Listing 10-15는 `largest`로 넘겨지는 슬라이스 내의 값의
타입이 `i32`와 `char`처럼 `PartialOrd` 및 `Copy` 트레잇 모두를 구현했을 때에 한하여 컴파일되는
제네릭 `largest` 함수의 완전체 코드를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-15: `PartialOrd`와 `Copy` 트레잇을 구현한 어떠한
제네릭 타입 상에서 동작하는 `largest` 함수의 동작 가능한 정의</span>

만일 우리의 `largest` 함수를 `Copy` 트레잇을 구현한 타입에 대한 것으로만 제한하길 원치 않는다면,
`T`가 `Copy` 대신 `Clone` 트레잇 바운드를 갖도록 명시하여 `largest` 함수가 소유권을 갖길
원하는 경우 슬라이스의 각 값이 복제되도록 할 수도 있습니다. 그러나 `clone` 함수를 이용한다는 것은
더 많은 힙 할당을 할 수 있다는 것이고, 힙 할당은 많은 양의 데이터에 대해서 동작할 경우 느릴 수
있습니다. `largest`를 구현하는 또다는 방법은 함수가 슬라이스 내의 `T` 값에 대한 참조자를
반환하도록 하는 것입니다. 만약 반환 타입을 `T` 대신 `&T`로 바꾸고 함수의 본체가 참조자를
반환하도록 바꾼다면, `Clone`이나 `Copy` 트레잇 바운드도 필요치 않으며 어떠한 힙 할당도
하지 않게 될 것입니다. 여러분이 직접 이 대안 해결책을 구현해보세요!

트레잇과 트레잇 바운드는 중복을 제거하기 위하여 제네릭 타입 파라미터를 사용하는 코드를 작성할 수 있도록
해주지만, 여전히 컴파일러에게 해당 제네릭 타입이 어떤 동작을 할 필요가 있는지를 정확히 명시하도록
해줍니다. 컴파일러에게 트레잇 바운드를 제공하기 때문에, 우리 코드와 함께 이용되는 모든 구체적인 타입들이
정확한 동작을 제공하는지를 확인할 수 있습니다. 동적 타입 언어에서는, 어떤 타입에 대해 어떤 메소드를
호출하는 시도를 했는데 해당 타입이 그 메소드를 구현하지 않았다면, 런타임에 에러를 얻게 됩니다.
러스트는 이러한 에러들을 컴파일 타임으로 옮겨서 우리의 코드가 실행 가능하기 전에 그 문제들을 해결하도록
우리를 강제합니다. 이에 더해서, 우리는 런타임에 해당 동작에 대한 검사를 하는 코드를 작성할 필요가
없는데, 우리는 이미 컴파일 타임에 이를 확인했기 때문이며, 이는 제네릭의 유연성을 포기하지 않고도
다른 언어들에 비해 성능을 향상시킵니다.

우리가 심지어 아직 알아채지도 못한 *라이프타임(lifetime)* 이라 불리는 또다른 종류의 제네릭이
있습니다. 라이프타임은 어떤 타임이 우리가 원하는 동작을 갖도록 확신하는데 도움을 주기 보다는,
참조자들이 우리가 원하는 만큼 오랫동안 유효한지를 확신하도록 도와줍니다. 라이프타임이 어떤 식으로 그렇게
하는지를 배워봅시다.
