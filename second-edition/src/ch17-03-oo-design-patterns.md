## 객체 지향 디자인 패턴 구현하기

*상태 패턴 (state pattern*은 객체 지향 디자인 패턴입니다. 이
패턴의 핵심은 어떤 값이 *상태 객체들*의 집합으로 표현되는 일종의
내부 상태를 가지며, 이 값의 동작은 내부 상태에 기반한여 바뀐다는
것입니다. 상태 객체들은 기능을 공유합니다: 당연히 러스트에서는
객체와 상속보다는 구조체와 트레잇을을 사용합니다. 각 상태 객체는
자신의 동작 및 다른 상태로 변경되어야 할 때의 제어에 대한 책임이
있습니다. 상태 객체를 보유한 값은 상태들의 서로 다른 행동 혹은
상태 간의 전이가 이뤄지는 때에 대해 아무것도 모릅니다.

상태 패턴을 사용한다는 것은 프로그램의 사업적 요구사항들이 변경될 때, 상태를
보유한 값의 코드 혹은 그 값을 사용하는 코드는 변경될 필요가 없음을 의미합니다.
단지 우리는 상태 객체 중에 하나의 내부 코드를 갱신하여 그 규칙을 바꾸거나
혹은 상태 객체를 더 추가할 필요가 있을 따름입니다. 상태 디자인 패턴 예제를
살펴보고 이를 러스트에서 사용하는 방법에 대해 알아봅시다.

우리는 점진적인 방식으로 블로그에 게시물을 올리는 작업 흐름을 구현하려고 합니다.
블로그의 최종적인 기능은 다음과 같을 것입니다:

1. 블로그 게시물은 빈 초안으로 시작합니다.
2. 초안이 완료되면 게시물의 검토가 요청됩니다.
3. 게시물이 승인되면 게시됩니다.
4. 오직 게시된 블로그 게시물만이 출력될 내용물을 반환하므로, 승인되지 않은 게시물은
   실수로라도 게시될 수 없습니다.

이 외에 게시물에 시도되는 어떠한 변경사항도 영향을 미쳐서는 안됩니다. 예를 들어,
만약 리뷰를 요청하기도 전에 블로그 게시물 초안을 승인하려는 시도를 했다면, 그
게시물은 비공개 초안인 상태로 유지되야 합니다.

Listing 17-11은 코드의 형태로 이 작업 흐름을 보여줍니다: 이는 우리가 구현할
라이브러리 크레이트 `blog`의 API를 사용하는 예제입니다. 아직 컴파일되지 않는 이유는
`blog` 크레이트를 아직 구현하지 않았기 때문이죠.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">Listing 17-11: `blog` 크레이트가 갖길 원하는
요구 동작들을 보여주는 코드</span>

우리는 사용자가 새로운 블로그 게시물의 초안을 `Post::new`를 통해 만들 수 있도록
허용하고 싶습니다. 이후에는 블로그 게시물에 초안인 상태로 글을 추가할 수 있도록 하고자
합니다. 만약 우리가 게시물의 내용을 승인 전에 즉시 얻어오는 시도를 하면, 해당 게시물이
아직 초안이기 때문에 아무 일도 일어나지 않아야 합니다. 이를 보여주는 용도로 코드 내에
`assert_eq!`를 추가했습니다. 이를 위한 훌륭한 단위 테스트는 블로그 게시물 초안이
`content` 메소드에 대해 빈 문자열을 반환하는 것이겠지만, 우리는 이 예제를 위한
테스트를 구현하진 않을 겁니다.

다음으로, 게시물의 리뷰를 요청하는 것을 허용하고자 하고, 리뷰를 기다리는
동안에는 `content`가 빈 문자열을 반환하도록 하고 싶습니다. 게시물이 허가를
받은 시점에는 게시가 되어야 하는데, 이는 `content`의 호출되었을 때 게시물의
글이 반환될 것임을 뜻합니다.

이 크레이트로부터 우리가 상호작용 하고 있는 유일한 타입이 `Post` 타입임을
주목하세요. 이 타입은 상태 패턴을 사용하며 게시물이 가질 수 있는 초안,
리뷰 대기중, 게시됨을 나타내는 세가지 상태 중 하나가 될 값을 보유할
것입니다. 어떤 상태에서 다른 상태로 변경되는 것은 `Post` 타입 내에서
내부적으로 관리됩니다. 이 상태들은 우리 라이브러리의 사용자가 `Post`
인스턴스 상에서 호출하는 메소드에 응답하여 변경되나, 상태 변화를 직접
관리할 필요는 없습니다. 또한, 사용자는 리뷰 전에 게시물이 게시되는 것
같은 상태와 관련된 실수를 할 수 없습니다.

### `Post`를 정의하고 초안 상태의 새 인스턴스 생성하기

라이브러리의 구현을 시작해보죠! 우리는 어떤 내용물을 담고 있는
공개된 `Post` 구조체가 필요하다는 것을 아니까, Listing 17-12에서
보시는 바와 같이 `Post`의 인스턴스를 만들기 위해서 구조체 및 관련된
공개 `new` 함수 대한 정의로 시작할 것입니다. 비공개 `State` 트레잇
또한 만들겁니다. 그 다음 `Post`는 비공개 필드 `state`에 `Option`으로
감싸진 `Box<State>` 형태의 트레잇 객체를 보유할 겁니다. 곧 `Option`이
왜 필요한지 보게 될 겁니다. 

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

<span class="caption">Listing 17-12: `Post` 구조체, `Post`
인스턴스를 만드는 `new` 함수, `State` 트레잇과 `Draft` 구조체의
정의</span>

`State` 트레잇은 게시물의 상태 변화에 따라 달라지는 동작을 정의하고, `Draft`,
`PendingReview`, 그리고 `Published` 상태는 모두 `State` 트레잇을
구현하게 됩니다. 지금은 트레잇이 아무런 메소드도 갖지 않고, 우리는 그저 `Draft`
상태의 구현부터 시작하려고 하는데, 왜냐면 그게 게시물이 최초로 갖는 상태이거든요.

우리가 새로운 `Post`를 생성할 때, 이것의 `state` 필드에 `Box`를 보유한
`Some` 값을 설정합니다. 이 `Box`는 `Draft` 구조체의 새 인스턴스를 가리킵니다.
이는 우리가 언제 `Post`의 새 인스턴스를 생성하든지, 초안으로 시작하는 것을
보장합니다. `Post`의 `state` 필드가 비공개이기 때문에, 다른 상태로 `Post`를
생성할 방법은 없습니다! `Post::new` 함수에서는 `content` 필드를 새로운, 빈
`String`로 설정합니다.

### 게시물 콘텐츠의 글을 저장하기

Listing 17-11은 우리가 `add_text`로 명명된 메소드를 호출하고 여기에
`&str`을 넘겨 블로그 게시물의 콘텐츠 글에 추가할 수 있도록 하길 원한다는
것을 보여줍니다. 우리는 `content` 필드를 `pub`으로 노출시키는 것보다는
메소드로서 이를 구현할 겁니다. 이는 우리가 `content` 필드의 데이터를
읽는 방식을 제어할 수 있는 메소드를 나중에 구현할 수 있음을 뜻합니다.
`add_text` 메소드는 매우 직관적이니까, Listing 17-13에서 `impl Post`
블록에 구현을 추가해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">Listing 17-13: `content`에 글을 추가하기
위한 `add_text` 메소드 구현하기 </span>

`add_text` 메소드는 가변 참조자 `self`를 취하는데, 그 이유는 우리가
`add_text`를 호출하고 있는 해당 `Post` 인스턴스를 변경하게 되기 때문입니다.
그런 다음 우리는 `content`의 `String` 상에서 `push_str`을 호출하고
`text`를 인자로 전달해 저장된 `content`에 추가합니다. 이 동작은 게시물의
상태와 무관하게 이뤄지므로, 상태 패턴의 일부가 아닙니다. `add_text` 메소드는
`state` 필드와 전혀 상호작용을 하지 않지만, 우리가 지원하고자 하는 동작
요소입니다.

### 초안 게시물의 내용이 비어있음을 보장하기

우리가 `add_text`를 호출하고 우리의 게시물에 어떤 콘텐츠를 추가한 이후일지라도,
여전히 `content` 메소드가 빈 스트링 슬라이스을 반환하길 원하는데, 그 이유는 
Listing 17-11의 8번째 줄처럼 게시물이 여전히 초안 상태이기 때문입니다. 당장은
이 요건을 만족할 가장 단순한 것으로 `content` 메소드를 구현해놓으려고 합니다:
언제나 빈 스트링 슬라이스를 반환하는 것으로요. 우리가 게시물의 상태를 변경하여
이것이 게시될 수 있도록 하는 기능을 구현하게 되면 그 후에 이 메소드를 변경하겠습니다.
그 때까지 게시물은 오직 초안 상태로만 존재하기에 게시물 컨텐츠는 항상 비어 있어야
합니다. Listing 17-14는 이 껍데기 구현을 보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

<span class="caption">Listing 17-14: 항상 비어있는 스트링 슬라이스를
반환하는 `Post`의 `content` 메소드에 대한 껍데기 구현 </span>

`content` 메소드를 추가함으로서, Listing 17-11의 8번째 줄까지는
의도한대로 작동됩니다.

### 게시물에 대한 리뷰 요청이 그의 상태를 변경합니다

다음으로, 우리는 게시물의 리뷰를 요청하는 기능을 만들어야 하는데, 이는 게시물의 상태를 `Draft`에서
`PendingReview`로 변경해야 합니다. Listing 17-15는 이에 관련된 코드 입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">Listing 17-15: `Post`와 `State` 트레잇에
`request_review` 메소드를 구현하기</span>

우리는 `Post`에게 `self`에 대한 가변 참조자를 취하려는 `request_review`란
이름의 공개 메소드를 주어줬습니다. 그 다음 우리가 `Post`의 현재 상태 상에서 내부
메소드 `request_review`를 호출하고, 이 두번째 `request_review` 메소드는
현재의 상태를 소비하고 새로운 상태를 반환합니다.

우리는 `State` 트레잇에 `request_review` 메소드를 추가했습니다; 트레잇을
구현하는 모든 타입은 이제 `request_review` 메소드를 구현할 필요가 있을 것입니다.
주목할 점은 메소드의 첫 인자를 `self`, `&self`, 나 `&mut self`를 취하기
보다 `self:Box<Self>`를 취한다는 겁니다. 이 문법은 메소드가 오직 그
타입을 보유한 `Box` 상에서 호출될 경우에만 유효함을 뜻합니다. 해당 문법은
`Box<Self>`의 소유권을 가져가는데, `Post`의 예전 상태를 무효화하여 새
상태로 변화하게 해줍니다.

이전 상태를 소비하기 위해서 `request_review` 메소드는 상태 값의 소유권을
취할 필요가 있습니다. 이것이 `Post`의 `state` 필드 내 `Option`이 들어온
까닭입니다: 우리는 `take` 메소드를 호출하여 `state` 필드 밖으로 `Some` 값을
빼내고 그 자리에 `None`을 남기는데, 왜냐면 러스트는 구조체 내에 값이 없는 필드를
허용하지 않기 때문입니다. 이는 우리가 `state` 값을 빌리기 보다는 게시물 밖으로
이동시키도록 만듭니다. 이후 우리는 게시물의 `state` 값을 이런 연산의 결과물로
설정하려고 합니다.

우리는 `state` 값의 소유권을 얻기 위해서
`self.state = self.state.request_review();`처럼 직접 설정하는 것 보다는
`state`를 임시로 `None`으로 설정할 필요가 있습니다. 이는 `Post`가 예전 `state`
값을 새 상태로 변경시킨 뒤에는 사용할 수 없음을 보장합니다.

`Draft`의 `request_review` 메소드는 새 박스로 포장된 `PendingReview`
구조체의 새 인스턴스를 반환해야 하며, 이는 게시물이 리뷰를 기다리고 있다는 상태를
표현합니다. `PendingReview` 구조체 또한 `request_review` 메소드를
구현하지만 어떤 변경도 하지 않습니다. 그보다 이 구조체는 자기 자신을 반환하는데,
그 이유는 이미 `PendingReview` 상태인 게시물에 대한 리뷰를 요청할 때는
`PendingReview` 상태를 그대로 유지해야 하기 때문입니다.

이제 우리는 상태 패턴의 장점을 알아보기 시작할 수 있습니다: `Post`의
`request_review` 메소드는 그것의 `state`가 무엇이든 상관없이 동일합니다.
각 상태는 그 자신의 규칙에 따라 맡은 책임을 다할 것 입니다.

우리는 `Post`의 `content` 메소드가 여전히 빈 스트링 슬라이스를 반환하도록 그대로
놔두려고 합니다. 현재 우리는 `Draft` 상태에 있는 `Post` 뿐만 아니라 `PendingReview`
상태에 있는 `Post`를 소유할 수 있습니다만, `PendingReview` 상태에서도 동일한 동작을
원합니다. Listing 17-11은 이제 11번째 줄까지 동작합니다!

### `content`의 동작을 변경하는 `approve` 메소드 추가하기

`approve` 메소드는 `request_review` 메소드와 유사할겁니다: 이것은
Listing 17-16과 같이 해당 상태가 허가되었을때 가져야 하는 값으로 `state`를
설정할 것입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         Box::new(PendingReview {})
#     }
#
    // --snip--
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         self
#     }
#
    // --snip--
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">Listing 17-16: `Post` 및 `State` 트레잇에
대한 `approve` 메소드 구현하기</span>

우리는 `approve` 메소드를 `State` 트레잇에 추가했고 `State`를 구현하는
새 구조체 `Published` 상태도 추가했습니다.

`request_review`와 유사하게, 우리가 `Draft`의 `approve` 메소드를 호출하면,
이는 별 효과가 없는데 이유는 이 때 반환되는 것이 `self`이기 때문이죠. 우리가
`PendingReview` 상에서 `approve`를 호출하면, 박스로 포장된 `Published`
구조체의 새 인스턴스가 반환됩니다. `Published` 구조체는 `State` 트레잇을 구현하고,
`request_review`와 `approve` 메소드 양 쪽 모두에서 자기 자신을 반환하는데,
이러한 경우에는 그 게시물이 `Published` 상태를 유지해야 하기 때문입니다.

이제 우리가 해야 할 일은 `Post`의 `content` 메소드를 갱신하는 겁니다: Listing
17-17처럼 만일 상태가 `Published`이면, 우리는 게시물의 `content` 필드의
값을 반환하길 원합니다; 그렇지 않다면, 우리는 빈 스트링 슬라이스를 반환하고자 합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
# trait State {
#     fn content<'a>(&self, post: &'a Post) -> &'a str;
# }
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // --snip--
}
```

<span class="caption">Listing 17-17: `State`의 `content` 메소드를
위임하기 위한 `Post`의 메소드 갱신하기</span>

목표하는 바가 `State`를 구현하는 구조체들 내에서 이 모든 규칙을 유지하는 것이기
때문에, 우리는 `state`의 값에 `content` 메소드를 호출하면서 게시물 인스턴스
(여기서는 `self`) 를 인자로 넘깁니다. 그러면 우리는 `state`의 `content`
메소드를 사용하여 반환되는 값을 받게 됩니다.

우리는 `Option`의 `as_ref` 메소드를 호출하는데 `Option` 내의 값에
대한 소유권을 가져오기 보다는 그에 대한 참조자를 원하기 때문입니다. `state`는
`Option<Box<State>>`이므로, 우리가 `as_ref`를 호출하면 `Option<&Box<State>>`가
반환됩니다. `as_ref`를 호출하지 않는다면, 우리는 해당 함수 파라미터의 빌려온
`&self`로부터 `state`를 이동시킬 수 없기 때문에 에러를 얻게 될 겁니다.

그런 다음 우리는 `unwrap` 메소드를 호출하고 이것이 패닉을 결코 발생시키지
않을 것을 알고 있는데, 그 이유는 `Post`의 메소드들은 이들이 실행 완료됐을
때 `state`가 항상 `Some` 값을 담고 있을 것을 보장하기 때문입니다. 이는
우리가 9장의 “여러분이 컴파일러보다 더 많은 정보를 가진 경우” 절에서 말했던
경우중 하나이며, 심지어 컴파일러가 그런 경우를 이해할 수 없더라도 `None`
값이 결코 가능하지 못한 경우입니다.

이 지점에서 우리가 `&Box<State>`의 `content`를 호출할 때, 역참조 강제는
`&`와 `Box`에 영향을 줘서 `content` 메소드가 궁극적으로 `State` 트레잇을
구현하는 타입 상에서 호출되도록 합니다. 이는 우리가 `State` 트레잇 정의에
`content`를 추가할 필요가 있음을 의미하고, 이곳이 Listing 17-18처럼
우리가 가진 상태에 따라 어떤 컨텐츠를 반환할지에 대한 로직을 삽입할 위치입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String
# }
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

<span class="caption">Listing 17-18: `State` 트레잇에 `content` 메소드
추가하기</span>

우리는 빈 스트링 슬라이스를 반환하는 `content` 메소드의 기본 구현을 추가합니다.
이는 우리가 `Draft`와 `PendingReview` 구조체에 대한 `content`를 구현할
필요가 없다는 뜻입니다. `Published` 구조체는 `content` 메소드를 오버라이딩하고
`post.content`의 값을 반환할 겁니다.

10장에서 우리가 토의했던 대로 이 메소드에 대한 라이프타임 명시가 필요함을
주의하세요. 우리는 `post`에 대한 참조자를 인자로 취하고 있고 해당 `post`의
일부에 대한 참조를 반환하는 중이므로, 반환되는 참조자의 라이프타임은 `post`
인자의 라이프타임과 관련이 있습니다.

그리고 이제 끝났습니다—Listing 17-11의 모든 코드가 이제 작동합니다! 우리는
블로그 게시물의 작업 흐름을 상태 패턴을 통해 구현해냈습니다. 규칙과 관련있는 로직들은
`Post`에 흩어져있지 않고 상태 객체에 존재합니다.

### 상태 패턴의 기회비용

우리는 게시물이 각 상태에 대해 가져야 하는 서로 다른 종류의 동작을
캡슐화하기 위해서 러스트로 객체 지향 상태 패턴을 충분히 구현할 수 있음을
보여줬습니다. `Post`의 메소드는 이런 다양한 동작에 대해서 알지 못합니다.
우리가 코드를 구조화한 방식에 따라, 게시된 게시물이 취할 수 있는 서로 다른
방법을 알기 위해서는 단 한 곳만 보면 됩니다: `Published` 구조체의
`State` 트레잇에 구현된 내용말이죠.

만약 우리가 상태 패턴을 사용하지 않고 다른 방식으로 구현한다면,
`Post` 혹은 `main` 코드에서 `match` 표현식을 대신 사용하여
게시물의 상태를 검사하고 이에 따라 해야 할 행동을 변경해야 할지도
모르겠습니다. 이는 우리가 게시된 상태의 게시물의 모든 결과들에 대해
이해하기 위해서 여러 곳을 봐야 한다는 것을 뜻합니다! 이는 우리가
상태를 추가하는 것을 더 늘게 할 뿐입니다: 각각의 `match` 표현식은
또 다른 갈래를 필요로 할테지요.

상태 패턴을 이용하면 `Post` 메소드들과 `Post`를 사용하는 곳에서는
`match` 표현식을 사용할 필요가 없고, 새로운 상태를 추가하려면, 그저 새로운
구조체와 구조체에 대한 트레잇 메소드들을 구현하면 됩니다.

상태 패턴을 사용하면 추가 기능을 구현하기 쉽습니다. 상태 패턴을
사용하는 코드를 유지하는 것의 단순성을 체험해보려면, 다음 제안 중
몇가지를 시도해보세요:

* `reject` 메소드를 추가하여 게시물의 상태를 `PendingReview`에서 `Draft`로
  변경하기
* `Published`로 상태 변경이 가능해지기 전에 `approve`가 두 번 호출되도록 요구하기
* 게시물이 `Draft` 상태일 때는 사용자들에게 글 내용의 추가만 허용하기.
  힌트: 상태 객체가 내용에 관한 변경에는 역할을 하지만 `Post`를 수정하기
  위한 역할은 하지 않게 하기

상태 패턴의 단점 중에 하나는, 상태가 상태 간의 전환을 구현하기
때문에, 몇몇 상태들이 서로 묶이게 된다는 점입니다. 만약 우리가
`PendingReview`와 `Published` 사이에 `Scheduled`와 같은
상태를 추가하면, `PendingReview`에서 `Scheduled`로 전환되도록
코드를 변경하여야 합니다. 새로운 상태의 추가와 함께 `PendingReview`가
변경될 필요가 없었다면 좀 더 작은 작업이 되겠지만, 이는 다른 디자인
패턴으로의 전환을 의미할 겁니다.

또다른 단점은 우리가 몇몇 로직을 중복시킨다는 겁니다. 중복의 일부를 제거하려면,
우리는 `State` 트레잇의 `request_review`와 `approve` 메소드가
`self`를 반환하도록 기본 구현을 만드는 시도를 할 수도 있습니다; 하지만,
이는 객체 안전성을 위배할 수 있는데, 그 이유는 해당 트레잇이 어떤 구체적인
`self`가 될 것인지 알지 못하기 때문입니다. 우리는 `State`를 트레잇 객체로서
사용하길 원하기에, 이것의 메소드들은 객체 안전성을 지킬 필요가 있습니다. 

`Post`에 메소드 `request_review`와 `approve`처럼 유사한 구현들도
그 밖의 중복에 포함됩니다. 두 메소드 모두 `Option`의 `state` 필드 값에
대해 동일한 메소드의 구현을 대행하며, 그 결과값을 `state` 필드의 새 값으로
설정합니다. 이 패턴을 따르는 `Post`의 메소드를 많이 갖게 되면,
이러한 반복을 제거하기 위해 매크로의 정의를 고려할 수도 있습니다
(매크로에 대한 자세한 내용은 부록 D를 참조하세요).

객체 지향 언어에서 정의하는 것과 동일하게 상태 패턴을 구현함으로써,
우리가 사용할 수 있는 러스트의 강점을 모두 이용하지 못하고 있습니다.
유효하지 않은 상태 및 전환이 컴파일 타임 에러가 될 수 있도록 하기 위해
우리가 할 수 있는 `blog` crate에 대한 변경 몇가지를 살펴봅시다.

#### 상태와 동작을 타입처럼 인코딩하기

우리는 다른 기회 비용을 얻기 위해 상태 패턴을 재고하는 방법을 보여줄 것입니다.
오히려 상태와 전환을 완전히 캡슐화하여 외부의 코드들이 이를 알지 못하는
것보다는, 상태를 다른 타입들로 인코딩하려고 합니다. 결론적으로, 러스트의 타입
검사 시스템은 게시된 게시물만 허용되는 곳에서 게시물 초안을 사용하려는 시도에
대해 컴파일 에러를 발생시킴으로서 방지할 것입니다.

Listing 17-11의 `main` 첫 부분을 주의깊게 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

우리는 여전히 `Post::new`를 사용하여 초안 상태의 새 게시물을 생성할 수
있도록 하며 게시물의 내용에 새 글을 추가할 수 있는 기능을 허용합니다. 하지만 빈
문자열을 반환하는 초안 게시물의 `content` 메소드 대신, 초안 게시물이 `content`
메소드를 갖지 않도록 만들려고 합니다. 이렇게 하면, 우리가 초안 게시물의 내용을
얻는 시도를 할 경우, 해당 메소드가 존재하지 않는다는 컴파일 에러를 얻게 될 겁니다.
결과적으로, 우리가 의도치않게 제작 중인 초안 게시물의 내용을 얻게 되는 일이
불가능하게 되는데, 왜냐면 그런 코드는 아예 컴파일이 되지 않으니까요. Listing
17-19에서는 `Post` 구조체와 `DraftPost` 구조체의 정의와 각각의 메소드를
보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">Listing 17-19: `content` 메소드가 있는
`Post`와 `content` 메소드가 없는 `DraftPost`</span>

`Post`와 `DraftPost` 구조체 모두 비공개인 `content` 필드를 가지고
블로그 게시물의 글을 보관합니다. 이 구조체들이 더 이상 `state` 필드를 갖지 않는
이유는 상태의 인코딩을 구조체의 타입으로 이동시켰기 때문입니다. `Post` 구조체는
공개된 게시물을 나타낼 것이고, 그의 `content` 메소드는 `content`를 반환할
겁니다.

우리는 여전히 `Post::new` 함수를 유지하지만, `Post`의 인스턴스를 반환하는
대신 `DraftPost`를 반환합니다. `content`는 비공개이고 `Post`를 반환할
어떤 함수도 존재하지 않기 때문에, 당장 `Post`의 인스턴스를 생성하는 것은
불가능합니다. 

`DraftPost` 구조체는 `add_text` 메소드를 가지고 있으므로, 우리는
전처럼 `content`에 글을 추가할 수 있지만, `DraftPost`는 정의된 `content`
메소드가 없음을 주의하세요! 따라서 이제 프로그램은 모든 게시물이 초안 게시물로
시작되고, 초안 게시물들은 그들의 내용을 출력할 능력이 없음을 보장합니다. 이 제약사항을
벗어나는 어떤 시도라도 컴파일러 에러로 끝나게 될 것입니다.

#### 다른 타입으로 변환하는 것처럼 전환 구현하기

그러면 어떻게 게시된 게시물을 얻는 걸가요? 우리는 초안 게시물이
공개되기 전에 리뷰와 승인을 받도록 강제하고 싶습니다. 리뷰를 기다리는
상태인 게시물은 여전히 어떤 내용도 보여줘서는 안되구요. Listing
17-20처럼 새 구조체 `PendingReviewPost`를 추가하고, `DraftPost`에
`PendingReviewPost`를 반환하는 `request_review` 메소드를 정의하고,
`PendingReviewPost`에 `Post`를 반환하는 `approve` 메소드를 정의하여
위의 제약사항들을 구현해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
# pub struct DraftPost {
#     content: String,
# }
#
impl DraftPost {
    // --snip--

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

<span class="caption">Listing 17-20: `DraftPost`의 `request_review`를
호출하여 생성되는 `PendingReviewPost` 및 `PendingReviewPost`를 게시된
`Post`로 전환하는 `approve` 메소드</span>

`request_review`와 `approve` 메소드는 `self`의 소유권을 취하므로,
`DraftPost`와 `PendingReviewPost`의 인스턴스를 소비하여 이들을
각각 `PendingReviewPost`와 게시된 `Post`로 변환시킵니다. 이 방식으로
우리가 `request_review`를 호출한 후 등등에는 `DraftPost` 인스턴스를
질질 끌지 않게될 겁니다. `PendingReviewPost` 구조체는 `content`
메소드의 정의를 갖지 않기 때문에, 그의 내용물을 읽으려는 시도는 `DraftPost`와
마찬가지로 컴파일 에러를 발생시킵니다. `content` 메소드를 정의하고 있는 게시된
`Post` 인스턴스를 얻을 수 있는 유일한 방법은 `PendingReiewPost`의
`approve` 메소드를 호출하는 것이고, `PendingReviewPost`를 얻을 수
있는 유일한 방법은 `DraftPost`의 `request_review`를 호출하는 것이기에,
우리는 이제 블로그 게시물의 작업 흐름을 타입 시스템으로 인코딩했습니다.

그뿐 아니라 우리는 `main`에 약간의 변화를 줘야 합니다. `request_review`와
`approve` 메소드는 호출되고 있는 구조체를 변경하기 보다는 새 인스턴스를
반환하기 때문에, 우리는 반환되는 인스턴스를 보관하기 위해 더 많은
`let post =`를 추가할 필요가 있습니다. 또한 초안과 리뷰 중인 게시물의
내용이 빈 문자열이라고 단언할 수도 없고, 단언할 필요도 없습니다: 이 상태들에서
게시물이 내용을 사용 시도하는 코드는 더이상 컴파일되지 않습니다. Listing 17-12에
갱신된 `main` 코드가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">Listing 17-21: 블로그 게시물 작업 흐름의 새
구현을 사용하기 위한 `main` 수정</span>


`main`에서 `post`의 다시 대입하기 위해 필요한 이 변경사항은 즉 이
구현이 더이상 객체 지향 상태 패턴을 잘 따르지 않는다는 것을 의미합니다:
상태간의 변환은 더 이상 `Post`의 구현체 내에 모두 캡슐화되지
않습니다. 하지만, 우리가 얻은 것은 타입 시스템과 컴파일 타임에
일어나는 타입 검사 때문에 유효하지 않은 상태가 이제 불가능해진다는
것입니다! 이는 게시되지 않은 게시물의 내용이 보여진다거나 하는 특정
버그들이 제품화가 되기 전에 발견될 것임을 보장합니다.

여러분이 이 버전의 코드 디자인에 대해 어떻게 생각하는지 알아보려면
이번 절의 시작점에서 우리가 언급했던 추가적인 요구사항으로서 제안된
작업을 Listing 17-20 이후의 `blog` 크레이트 상에서 시도해보세요.
몇가지 작업은 이번 디자인에서 이미 완료됐음을 알려드립니다. 

우리는 러스트가 객체 지향 디자인 패턴을 사용할 수 있을지라도,
상태를 타입 시스템으로 인코딩하는 다른 패턴 또한 러스트 내에서
가능함을 봤습니다. 이 패턴들은 서로 다른 기회비용을 갖고 있습니다.
여러분이 객체 지향 패턴에 매우 익숙할런지 몰라도, 몇몇 버그를
컴파일 타임에 방지하는 등 러스트의 기능들이 제공할 수 있는 이점들을
이용하기 위해서는 문제를 다시 생각해보세요. 객체 지향 패턴은
러스트 내에서 제공하는 소유권 같이 객체 지향 언어에서는 갖지 못한
기능들 때문에 늘 최고의 해결책이 되지는 못합니다.

## 정리

이 장을 읽은 후 러스트가 객체 지향 언어라고 생각하든 아니든,
이제 여러분은 트레잇 객체를 사용하여 몇가지 객체 지향 기능을 러스트
내에서 사용할 수 있다는 것을 알게 되었습니다. 동적 디스패치는
약간의 실행 성능과 맞바꿔 여러분의 코드에 유연성을 줄 수 있습니다.
여러분은 이 유연성을 사용하여 여러분의 코드 관리를 도와줄 수 있는
객체 지행 패턴을 구현할 수 있습니다. 러스트는 또한 소유권과 같은
객체 지향 언어들에는 없는 기능들도 갖고 있습니다. 객체 지향 패턴이
항상 러스트의 강점을 이용하는 최고의 방법은 아니겠지만, 선택 가능한
옵션입니다.

다음으로, 우리는 패턴을 살펴볼 것인데, 이는 높은 유연성을 가능케하는
러스트의 또다른 기능 중 하나입니다. 이 책 전체에 걸쳐 간단히 살펴보긴 했지만
아직 패턴들의 모든 능력을 살펴본건 아닙니다. 어서 가즈아!
