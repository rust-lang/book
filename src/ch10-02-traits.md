## 트레잇으로 공통된 동작을 정의하기

*트레잇(trait)* 은 특정 타입이 가지고 있고 다른 타입과 공유할 수 있는
**기능**에 대한 정보를 러스트 컴파일러에게 전달합니다.
트레잇을 사용하면 공통된 기능을 추상적으로 정의할 수 있습니다.
트레잇 바운드(trait bound)를 이용하면 '특정 동작이 가능한 어떤 타입'을 제네릭으로 명시할 수도 있습니다.

> Note: 트레잇은 다른 언어에서 *인터페이스(interface)* 라고 부르는 기능과 유사하지만,
> 약간 차이가 있습니다.

### 트레잇 정의하기

어떤 타입의 동작은 우리가 해당 타입에서 호출할 수 있는 메소드로 구성됩니다.
만약 다양한 타입에서 공통된 메소드를 호출할 수 있다면, 우린 이 타입들이
동일한 동작을 공유한다고 표현할 수 있을 겁니다. 트레잇 정의는 메소드 시그니처를
그룹화하여 특정 목적을 달성하는 데 필요한 일련의 동작을 정의하는 것입니다.

예를 들어 봅시다.
다양한 종류 및 분량의 텍스트를 갖는 여러 가지 구조체가 있다고 치죠.
`NewsArticle` 구조체는 특정 지역에서 등록된 뉴스 기사를 저장하고,
`Tweet` 구조체는 최대 280자의 콘텐츠와 메타데이터(해당 트윗이 새 트윗인지,
리트윗인지, 다른 트윗의 대답인지)를 저장합니다.

`NewsArticle` 이나 `Tweet` 인스턴스에 저장된 데이터를 종합해 보여주는
종합 미디어 라이브러리를 만든다고 가정합시다.
이를 위해서는 각 타입의 요약 정보를 얻어와야 하는데,
이는 인스턴스에 `summarize` 메소드를 호출하여 가져오고자 합니다.
Listing 10-12는 이 동작을 `Summary` 트레잇 정의로 표현합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

<span class="caption">Listing 10-12: `summarize` 메소드가 제공하는
'요약' 동작으로 구성된 `Summary` 트레잇</span>

`trait` 키워드 다음 트레잇의 이름
`Summary` 를 작성해 트레잇을 선언했습니다.
중괄호 안에는 이 트레잇을 구현할 타입의 동작을 묘사하는
메소드 시그니처(`fn summarize(&self) -> String`)를 선언했습니다.

메소드 시그니처 뒤에는 중괄호로 시작하여
메소드를 구현하는 대신 세미콜론을 집어넣었습니다.
이 트레잇을 구현할 각각의 타입이 직접 메소드에 맞는 동작을 제공하도록 하기 위함입니다.
이제 `Summary` 트레잇을 갖는 모든 타입은 정확히 이와 같은 시그니처의
`summarize` 메소드를 가지고 있음을 러스트 컴파일러가 보장해줄 겁니다.

트레잇은 본문에 여러 메소드를 가질 수 있습니다.
메소드 서명은 한 줄에 하나씩 나열되며, 각 줄은 세미콜론으로 끝납니다.

### 특정 타입에 트레잇 구현하기

원하던 동작을 `Summary` 트레잇으로 정의했으니,
우리의 종합 미디어 내 각 타입에 `Summary` 트레잇을 구현해봅시다.
Listing 10-13은 헤드라인, 저자, 지역 정보를 사용하여 `summarize`의 반환 값을
만드는 `Summary` 트레잇을 `NewsArticle` 구조체에 구현한 모습입니다.
`Tweet` 구조체에는 트윗 내용이 이미 280자로 제한되어있음을 가정하고,
사용자명과 해당 트윗의 전체 텍스트를 가져오도록 `summarize` 를
정의했습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Summary {
#     fn summarize(&self) -> String;
# }
#
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<span class="caption">Listing 10-13: `NewsArticle`과 `Tweet` 타입에
`Summary` 트레잇 구현</span>

어떤 타입에 대한 트레잇을 구현하는 것은
평범한 메소드를 구현하는 것과 유사합니다.
다른 점은 `impl` 뒤에 구현하고자 하는 트레잇 이름을 적고,
그다음 `for` 키워드와 트레잇을 구현할
타입명을 명시한다는 점입니다.
`impl` 블록 내에는 트레잇 정의에서 정의된 메소드 시그니처를 집어넣되,
세미콜론 대신 중괄호를 사용하여 메소드 본문에
우리가 원하는 특정한 동작을 채워 넣습니다.

트레잇을 구현하고 나면, 다음과 같이 평범한 메소드를 호출하던 것과
동일한 방식으로 `NewsArticle`과 `Tweet` 인스턴스에서 메소드를 호출할 수 있습니다:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

이 코드는
`1 new tweet: horse_ebooks: of course, as you probably already know, people`를 출력합니다.

Listing 10-13에서 `Summary` 트레잇과 `NewsArticle`, `Tweet` 타입을 동일한
*lib.rs* 파일에 정의했기 때문에 모두 같은 스코프 내에 있다는 점을 알아두세요.
만약 이 *lib.rs* 가 `aggregator` 크레이트에 있고,
누군가가 자신의 라이브러리 스코프에 정의된 구조체에
우리 크레이트의 `Summary` 트레잇을 구현하고자 한다면
먼저 `use aggregator::Summary;` 를 명시하여
트레잇을 자신의 스코프에 가져와야합니다.
물론 다른 크레이트에서 `Summary` 트레잇을 구현할 수 있도록 하려면
Listing 10-12처럼 `trait` 앞에 `pub` 키워드를 붙여
공개 트레잇으로 만들어야 합니다.

트레잇 구현에는 한 가지 제약사항이 있습니다. 타입에 트레잇을 구현할 때,
해당 트레잇이나 트레잇을 구현할 타입 둘 중 하나는 반드시 우리 크레이트 내의 것이어야 합니다.
예를 들어, 우린 우리가 만든 `aggregator` 크레이트 기능의 일부로서
`Tweet` 타입에 표준 라이브러리 트레잇인 `Display` 등을 구현할 수 있습니다.
`Tweet` 타입이 우리가 만든 `aggregator` 크레이트의 타입이기 때문입니다.
또한 `aggregator` 크레이트 내에서 `Vec<T>` 타입에 `Summary` 트레잇을 구현할 수도 있습니다.
마찬가지로 `Summary` 트레잇이 우리가 만든 `aggregator` 크레이트의 트레잇이기 때문입니다.

하지만 외부 타입에 외부 트레잇을 구현할 수는 없습니다.
예를 들어, 우린 우리가 만든 `aggregator` 크레이트에서 `Vec<T>` 에 `Display` 트레잇을 구현할 수 없습니다.
`Vec<T>`, `Display` 모두 우리가 만든 크레이트가 아닌 표준 라이브러리에 정의되어 있기 때문입니다.
이 제약은 프로그램의 특성 중 하나인 *일관성(coherence)*,
보다 자세히는 *고아 규칙(orphan rule)* 에서 나옵니다
(부모 타입이 존재하지 않기 때문에 고아 규칙이라고 부릅니다).
이 규칙으로 인해 다른 사람의 코드는 여러분의 코드를 망가뜨릴 수 없으며 반대의 경우도 마찬가지입니다.
이 규칙이 없다면 두 크레이트가 동일한 타입에 동일한 트레잇을 구현할 수 있게 되고,
러스트는 어떤 구현체를 이용해야 할지 알 수 없습니다.

### 기본 구현

타입에 트레잇을 구현할 때마다 모든 메소드를 구현할 필요는 없도록
트레잇의 메소드에 기본 동작을 제공할 수도 있습니다.
이러면 특정한 타입에 트레잇을 구현할 때 기본 동작을 유지할지
오버라이드(override)할지 선택할 수 있습니다.

Listing 10-14는 Listing 10-12에서 `Summary` 트레잇에
메소드 시그니처만 정의했던 것과는 달리 `summarize` 메소드에
기본 문자열을 명시하는 방법을 나타냅니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

<span class="caption">Listing 10-14: `summarize` 메소드의 기본 구현을
포함하는 `Summary` 트레잇 정의</span>

`NewsArticle` 인스턴스에 별도의 구현을 정의하지 않고
기본 구현을 사용하려면 `impl Summary for NewsArticle {}` 처럼
`impl` 블록을 비워두면 됩니다.

`impl` 블록을 비워도 `NewsArticle` 인스턴스에서 `summarize` 메소드를 호출할 수 있습니다.
`NewsArticle` 에 `summarize` 메소드를 직접적으로 정의하지는 않았지만,
`NewsArticle` 은 `Summary` 트레잇을 구현하도록 지정되어 있으며,
`Summary` 트레잇은 `summarize` 메소드의 기본 구현을 제공하기 때문입니다:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summarize());
```

이 코드는 `New article available! (Read more...)`를 출력합니다.

`summarize` 기본 구현을 생성한다고 해서
Listing 10-13 코드의 `Tweet` 의 `Summary` 구현을 변경할 필요는 없습니다.
기본 구현을 오버라이딩하는 문법과,
기본 구현이 없는 트레잇 메소드를
구현하는 문법은 동일하기 때문입니다.

기본 구현 내에서 트레잇 내 다른 메소드를 호출할 수도 있습니다.
호출할 다른 메소드가 기본 구현을 제공하지 않는 메소드여도 상관없습니다.
이 점을 이용해, 트레잇의 일부 구현만 명시하는 것만으로도 트레잇의 다양한 기능을 제공받을 수 있습니다.
예시로 알아봅시다.
`Summary` 트레잇에 `summarize_author` 메소드를 추가하고,
`summarize` 메소드의 기본 구현 내에서 `summarize_author` 메소드를 호출하도록
만들어보았습니다:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

이 `Summary`를 어떤 타입에 구현할 때는 `summarize_author` 만
정의하면 됩니다:

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

`summarize_author` 를 정의하고 나면 `Tweet` 인스턴스에서 `summarize` 를 호출할 수 있습니다.
이러면 `summarize` 기본 구현이 우리가 정의한 `summarize_author` 메소드를 호출할 겁니다.
우린 `summarize_author` 만 구현하고 추가적인 코드를 전혀 작성하지 않았지만,
`Summary` 트레잇은 우리에게 `summarize` 메소드의 기능도 제공해주는 것을
알 수 있습니다.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

이 코드는 `1 new tweet: (Read more from @horse_ebooks...)` 를 출력합니다.

단, 어떤 메소드를 오버라이딩하는 구현을 하면 해당 메소드의 기본 구현을
호출할 수는 없다는 점을 기억해두세요.

### 매개변수로서의 트레잇

트레잇을 정의하고 구현하는 방법을 알아보았으니, 트레잇을 이용하여
어떤 함수가 다양한 타입으로 작동하게 만드는 법을 알아봅시다.

Listing 10-13에서 `Summary` 트레잇을
`NewsArticle`, `Tweet` 타입에 구현해보았습니다.
이번에는 `item` 매개변수의 `summarize` 메소드를 호출하는 `notify` 함수를 정의해보죠.
이때 `item` 매개변수의 타입은 `Summary` 트레잇을 구현하는 어떤 타입입니다.
이럴 때는 `impl Trait` 문법을 사용합니다:

```rust,ignore
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`item` 매개변수의 구체적 타입을 명시하는 대신 `impl` 키워드와 트레잇 이름을 명시했습니다.
이 매개변수는 명시된 트레잇을 구현하는 타입이라면 어떤 타입이건 전달받을 수 있습니다.
`notify` 본문 내에서는 `item` 에 `summarize` 등
`Summary` 트레잇의 모든 메소드를 호출할 수 있습니다.
`notify` 는 `NewsArticle` 인스턴스로도, `Tweet` 인스턴스로도 호출할 수 있습니다.
만약 `notify` 함수를 `Summary` 트레잇을 구현하지 않는
`String`, `i32` 등의 타입으로 호출하는 코드를 작성한다면 컴파일 에러가 발생합니다.

#### 트레잇 바운드 문법

`impl Trait` 문법은 간단하지만,
이는 *트레잇 바운드* 라는 긴 형식의 syntax sugar입니다.
트레잇 바운드는 다음과 같이 생겼습니다:

```rust,ignore
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

앞서 본 예시와 동일한 코드지만, 더 장황합니다.
트레잇 바운드는 꺾쇠괄호 안의 제네릭 타입 매개변수 선언에 붙은 콜론(`:`) 뒤에
위치합니다.

`impl Trait` 문법은 편리하고, 단순한 상황에는 코드를 더 간결하게 만듭니다.
하지만 트레잇 바운드 문법은 더 복잡한 상황을 표현할 수 있습니다.
예를 들어, `Summary` 를 구현하는 두 매개변수를 전달받는 함수를
`impl Trait` 문법으로 표현하면 다음과 같습니다:

```rust,ignore
pub fn notify(item1: impl Summary, item2: impl Summary) {
```

`item1` 과 `item2` 가 (둘 다 `Summary`를 구현하는 타입이되)
서로 다른 타입이어도 상관없다면 `impl Trait` 문법 사용도 적절합니다.
하지만 만약 두 매개변수가 같은 타입이어야만 한다는 요구사항이 추가되면,
이는 트레잇 바운드 문법으로만 표현 가능합니다:

```rust,ignore
pub fn notify<T: Summary>(item1: T, item2: T) {
```

`item1` 및 `item2` 매개 변수의 타입으로 지정된 제네릭 타입 `T`는
함수를 호출할 때 `item1`, `item2` 인자 값의 구체적인 타입이
반드시 동일하도록 제한합니다.

#### `+` 구문으로 트레잇 바운드를 여럿 지정하기

트레잇 바운드는 여러 개 지정할 수도 있습니다.
`notify` 에서 `item` 의 `summarize` 메소드뿐만 아니라 출력 포매팅까지 사용한다고 가정해보죠.
`notify` 정의에서 `item` 이 `Display`, `Summary`를 모두 구현해야하도록 지정해야합니다.
`+` 구문을 사용하면 트레잇을 여러 개 지정할 수 있습니다:

```rust,ignore
pub fn notify(item: impl Summary + Display) {
```

`+` 구문은 제네릭 타입의 트레잇 바운드에도 사용할 수 있습니다:

```rust,ignore
pub fn notify<T: Summary + Display>(item: T) {
```

두 트레잇 바운드가 지정됐으니, `notify` 본문에서는 `item` 의 `summarize` 메소드를
호출할 수도 있고 `item` 을 `{}` 으로 포매팅할 수도 있습니다.

#### `where` 조항으로 트레잇 바운드 정리하기

트레잇 바운드가 너무 많아지면 문제가 생깁니다.
제네릭마다 트레잇 바운드를 갖게 되면, 여러 제네릭 타입 매개변수를 사용하는 함수는
함수명과 매개변수 사이에 너무 많은 트레잇 바운드 정보를 담게 될 수 있습니다.
이는 가독성을 해치기 때문에, 러스트는 트레잇 바운드를 함수 시그니처 뒤의
`where` 조항에 명시할 수 있도록 대안을 제공합니다.
즉, 다음과 같이 작성하는 대신:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

다음과 같이 `where` 조항을 사용할 수 있습니다:

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

트레잇 바운드로 도배되지 않은 평범한 함수처럼
함수명, 매개변수 목록, 반환형이 붙어 있으니, 함수 시그니처를
읽기 쉬워집니다.

### 트레잇을 구현하는 타입을 반환하기

`impl Trait` 문법을 트레잇을 구현하는 어떤 타입의 값을 반환하는 데
사용할 수도 있습니다:

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

반환 타입에 구체적인 타입명이 아닌 `impl Summary` 를 작성하여
`returns_summarizable` 함수는 `Summary` 트레잇을 구현하는 어떤 타입을 반환함을 명시했습니다.
이 경우 `returns_summarizable` 는 `Tweet` 을 반환하지만,
이 함수를 호출하는 쪽의 코드에서는 구체적인 타입을 알 수 없습니다.

구현하는 트레잇으로만 명시된 타입을 반환하는 기능은
13장에서 다룰 클로저 및 반복자 문법에서 굉장히 유용합니다.
클로저와 반복자는 컴파일만 아는 타입이나,
직접 명시하기에는 굉장히 긴 타입을 생성합니다.
`impl Trait` 문법을 사용하면 굉장히 긴 타입을 직접 작성할 필요 없이
`Iterator` 트레잇을 구현하는 어떤 타입이라고만 간결하게 명시할 수 있습니다.

하지만, `impl Trait` 문법을 쓴다고 해서 다양한 타입을 반환할 수는 없습니다.
다음은 반환형을 `impl Summary`로 명시하고 `NewsArticle`, `Tweet` 중 하나를 반환하는 코드 예시입니다.
이 코드는 컴파일할 수 없습니다:

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

`NewsArticle`, `Tweet` 중 하나를 반환하는 행위는 컴파일러 내에서
`impl Trait` 문법이 구현되는 방식으로 인해 제한됩니다.
함수가 이렇게 동작하도록 만드는 방법은 17장의
["트레잇 객체를 사용하여 다른 타입 간의 값 허용하기"][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore -->
절에서 알아볼
예정입니다.

### 트레잇 바운드로 `largest` 함수 고치기

제네릭 타입 매개변수에 원하는 동작을 지정하는 방법을 배웠으니,
Listing 10-5의 `largest` 함수 정의를 제네릭 타입 매개변수를 사용하도록 고쳐봅시다!
마지막으로 코드를 실행했을 때 나타났었던 에러는
다음과 같습니다:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

`largest` 본문에서 `T` 타입 값 두 개를 큰 부등호(`>`) 연산자로 비교할 수 있길 원했습니다.
해당 연산자는 표준 라이브러리 `std::cmp::PartialOrd` 트레잇의 기본 메소드로 정의되어있으니,
`T`의 트레잇 바운드로 `PartialOrd` 를 지정하면 `largest` 함수는
서로 비교 가능한 모든 타입의 슬라이스로 작동할 수 있습니다.
`PartialOrd`는 프렐루드(prelude)에 포함되어 있기 때문에
따로 스코프 내로 가져올 필요는 없습니다.
`largest` 함수의 시그니처를 다음과 같이 바꿔봅시다:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

하지만 이 코드를 컴파일하면 또 다른 에러가 나타납니다:

```text
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

에러의 핵심은 `cannot move out of type [T], a non-copy slice` 입니다.
우리가 제네릭을 사용하지 않고 만든 `largest` 함수는 `i32`, `char` 타입만 다루었습니다.
4장의 ["스택에만 저장된 데이터: 복사(copy)"][stack-only-data-copy]<!-- ignore --> 절에서 다루었듯
`i32`, `char` 타입은 고정된 크기를 갖고 스택에 저장되는 타입이며 `Copy` 트레잇을 구현합니다.
하지만 제네릭으로 만든 `largest` 함수는
`list` 매개변수로 `Copy` 트레잇을
구현하지 않는 타입이 전달될 수도 있습니다.
이 경우 `list[0]` 값을 `largest` 변수로 이동시킬 수 없으니,
에러가 발생합니다.

`T`의 트레잇 바운드에 `Copy`를 추가해서 이 코드는
`Copy` 트레잇을 구현하는 타입으로만 호출할 수 있도록 바꿔봅시다.
Listing 10-15는 `i32`, `char` 타입처럼 `PartialOrd`, `Copy` 트레잇을
*모두* 구현하는 타입의 값일 경우에만 컴파일되는 제네릭 `largest` 함수의
전체 코드입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
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
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-15: `PartialOrd`, `Copy` 트레잇을
모두 구현하는 제네릭 타입으로 작동하는
`largest` 함수 정의</span>

`Copy` 트레잇을 구현하는 타입으로만 호출 가능한 제약을 없애고 싶다면
`T`의 트레잇 바운드에 `Copy` 대신 `Clone` 을 지정한 후,
`largest` 함수에서 소유권이 필요할 때 슬라이스 내
각 값을 복제하도록 할 수도 있습니다.
단, `clone` 함수의 사용은 `String` 처럼 힙 영역 데이터를 갖는 타입의 경우
더 많은 힙 할당이 발생할 수 있으며 데이터양이 많아지면
힙 할당이 느려질 수도 있습니다.

아니면 `largest` 함수가 슬라이스 내 `T` 값의
참조자를 반환하도록 구현할 수도 있습니다.
반환형을 `T` 에서 `&T` 로 변경하고 함수 본문도 그것에 맞게 변경한다면,
`Clone` 이나 `Copy` 트레잇 바운드도 불필요하며 힙 할당도 피할 수 있죠.
이 방법은 여러분이 직접 구현해보세요!

### 트레잇 바운드를 사용해 조건부로 메소드 구현하기

제네릭 타입 매개변수를 사용하는 `impl` 블록에 트레잇 바운드를 이용하면,
명시된 트레잇을 구현하는 타입에만 메소드를 구현할 수도 있습니다.
예를 들어, Listing 10-16의 `Pair<T>` 타입은 항상 `new` 함수를 구현합니다.
하지만 만약 `T` 타입이 비교를 가능하게 만드는 `PartialOrd` 트레잇과
출력을 가능하게 만드는 `Display` 트레잇을 모두 구현하는 타입이라면,
그런 경우에만 `cmp_display` 메소드를 구현합니다.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

<span class="caption">Listing 10-16: 트레잇 바운드를 이용해 제네릭 타입에
조건부로 메소드 구현하기</span>

타입이 특정 트레잇을 구현하는 경우에만 해당 타입에 트레잇을 구현할 수도 있습니다.
트레잇 바운드에 맞는 모든 타입에 트레잇을 구현하는 것을
*포괄 구현(blanket implementations)* 이라 하며,
이는 러스트 표준 라이브러리 내에서 광범위하게 사용됩니다.
예를 들어, 표준 라이브러리는 `Display` 트레잇을 구현하는 모든 타입에 `ToString` 트레잇도 구현합니다.
표준 라이브러리 내 `impl` 블록은 다음과 유사합니다:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

우리가 `Display` 트레잇이 구현된 모든 타입에 `to_string()`
메소드(`ToString` 트레잇에 정의된)를 호출할 수 있는 건
표준 라이브러리의 이 포괄 구현 덕분입니다.
예를 들어, 정수는 `Display`를 구현하므로 `String` 값으로 변환할 수 있습니다.

```rust
let s = 3.to_string();
```

포괄 구현은 트레잇 문서 페이지의 "구현체(Implementors)" 절에
있습니다.

트레잇과 트레잇 바운드를 사용하면 제네릭 타입 매개변수로
코드 중복을 제거하면서 '특정 동작을 하는 제네릭 타입'
이 필요하다는 걸 컴파일러에게 전달할 수 있습니다.
컴파일러는 트레잇 바운드를 이용하여 우리가 코드에서
사용한 구체적인 타입들이 알맞은 동작을 제공하는지 검사합니다.
동적 타입 언어에선 해당 타입이 구현하지 않는 메소드를 호출하면
런타임에 에러가 발생합니다. 하지만 러스트는 컴파일타임에 에러를
제공하여 코드를 실행하기도 전에 문제를 해결하도록 강제합니다.
따라서 우린 런타임에 해당 동작을 구현하는지를 검사하는 코드를
작성할 필요가 없습니다. 컴파일 타임에 이미 다 확인했기 때문이죠.
러스트는 제네릭의 유연성과 성능 둘 다 놓치지 않습니다.

제네릭의 다른 종류로는 *라이프타임(lifetime)* 이 있습니다.
(사실 우리는 이미 라이프타임을 사용해봤습니다.)
라이프타임은 타입의 동작을 보장하는 것이 아닌, 참조자를 우리가 원하는 시점까지
유효하도록 보장하는 역할을 합니다. 라이프타임이 어떻게 동작하는지 살펴봅시다.

[stack-only-data-copy]:
ch04-01-what-is-ownership.html#%EC%8A%A4%ED%83%9D%EC%97%90%EB%A7%8C-%EC%A0%80%EC%9E%A5%EB%90%9C-%EB%8D%B0%EC%9D%B4%ED%84%B0-%EB%B3%B5%EC%82%ACcopy
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
