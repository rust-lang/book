## 객체 지향 디자인 패턴 구현

*상태 패턴*은 객체지향 디자인 패턴입니다. 패턴의 핵심은 *상태 객체*들의 집합으로 표현되는 일종의 내부 변수를 갖으며, 변수의 행위 변화는 내부 상태에 기반한다는 겁니다. 상태 객체들은 기능을 공유합니다: Rust에서, 당연하게도, 우리는 구조체와 특성을 객체와 상속대신 사용합니다. 각 상태 객체들은 다른 상태로 변경되어야 할 때의 제어와 그 자신의 행위를 담당합니다. 

상태 패턴을 사용한다는 의미는 비지니스가 프로그램의 변화를 요구할 시에, 상태를 나타내는 변수나 그 변수를 사용하는 코드를 변경할 필요가 없다는 겁니다. 단지 우리는 상태 객체 중에 하나의 내부 코드를 그 규칙대로 변경하거나 상태 객체를 추가하면 되죠. 상태 디자인 패턴 예제를 살펴보고 그를 Rust에서 어떻게 사용할 수 있는지 알아봅시다.

우리는 점진적 방식으로 블로그에 글을 올리는 작업 흐름을 구현하려고 합니다. 블로그의 최종적 기능은 다음과 같습니다. 

1. 블로그 게시물은 빈 초안으로 시작합니다.
2. 초안이 완료되면 게시물의 검토가 요구된다.
3. 게시물이 승인되면 게시됩니다.
4. 오직 공개된 블로그 게시물만이 출력되게 해서, 승인되지 않은 게시물이 실수로라도 공개되지 않도록 합니다.   

이 외에 어떤 시도도 게시물에 영향을 미쳐서는 안됩니다. 예를 들어, 만약 리뷰를 요청하기도 전에 블로그 게시물 초안을 승인하려는 시도를 했다면, 게시물은 비공개 초안인 상태로 유지되야 합니다.

항목 17-11 위의 작업 순서를 코드화: 이는 우리가 우리가 구현할 라이브러리 crate `blog`의 API를 사용하는 예제입니다. 현재는 컴파일되지 않는데 이유는 `blog` crate를 아직 구현하지 않았기 때문이죠. 

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

<span class="caption">항목 17-11: 코드화 된 우리가 `blog` crate에서 기대하는 동작</span>

우리가 허용하고자 하는 것은 사용자가 새로운 블로그 포스트 초안을 `Post::new`를 통해 만들 수 있도록 하는 겁니다. 
이후에는 블로그 게시물에 초안인 상태로 글을 추가할 수 있도록 하고자 합니다. 
만약 우리가 게시물의 내용을 승인 전에 보고자 한다면, 아직 초안이기 때문에 아무 일도 일어나지 않을 겁니다. 
우리가 추가한 `assert_eq!`는 이런 의도를 명확히 했습니다. 
이를 위한 훌륭한 유닛 테스트는 블로그 게시물 초안이 `content` 메소드에 대해 빈 문자열을 반환하는 것이겠지만, 
우리는 이 예제를 위한 테스트를 구현하진 않을 겁니다.

다음으로, 게시물의 리뷰를 요청하는 것을 허용하고자 하고, 리뷰를 기다리는 동안에는 빈 문자열을 반환하도록 하고 싶습니다. 
게시물이 허가를 받은 시점에, 그는 배포는데, 이는 게시물의 글이 `content`의 호출로 반환될 수 있게 된다는 뜻 입니다.


강조하고 싶은 것은, 우리가 상호작용 하는 유일한 타입은 crate `Post` 타입이라는 겁니다. 이 타입은 상태 패턴을 사용할 것이고 값을 저장하여 게시물이 가질 수 있는 다양한 상태 중에 세 가지 상태 객체 중 하나를 나타냅니다 — 초안, 리뷰대기, 혹은 게시됨. 하나의 상태에서 다른 상태로 변경되는 것은 `Post` 타입과 함께 관리됩니다. 상태의 변화는 우리의 라이브러리 사용자가 `Post` 인스턴스의 메소드를 호출하고 이에 대한 응답으로 바뀝니다. 하지만 그들은 상태 변화를 직접적으로 관리할 필요가 없습니다. 또한, 사용자가 상태에 관해 리뷰 전에 게시물이 공개되는 것 같은 실수 할 수 없도록 만듭니다.

### `Post`를 정의하고 초안 상태의 새 인스턴스 생성하기

라이브러리의 구현을 시작해보죠! 우리가 아는 것은 우리가 public `Post` 구조체를 컨텐츠를 유지하는데 필요하다는 겁니다, 그러니 우리는 구조체에 대한 정의를 시작할 것이며  관련된 public `new` 함수를 정의하여 `Post`의 인스턴스를 생성할겁니다, 항목 17-12처럼 말이죠. 또한 private `State` 또한 만들겁니다. 이후에 `Post`는 `Box<State>`인 특성 객체를 `Option` 내부에 private 필드 `state`로 포함할겁니다. 곧 `Option`이 왜 필요한지 보게 될 겁니다. 

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

<span class="caption">
항목 17-12: `Post` 구조체의 정의와 `Post`인스턴스를 만드는 `new` 함수의 정의, `State` 특성과 `Draft` 구조체</span>

`State` 특성은 게시물의 상태 변화에 따라 달라지는 동작을 정의하고, `Draft`, `PendingReview`, 그리고 `Published` 상태는 모두 `State` 특성을 구현하게 됩니다. 특성이 지금은 아무런 메소드도 갖지 않고, 우리는 지금부터 `Draft` 상태의 구현부터 시작하려고 합니다, 왜냐면 그게 게시물이 최초로 갖는 상태이거든요.

우리가 새로운 `Post`를 생성할 때, 그의 `state`필드에 `Box`를 가진 `어떤` 값을 설정합니다. 이 `Box`는 `Draft` 구조체의 새 인스턴스를 가르킵니다. 이는 우리가 어디에서 `Post`의 새 인스턴스를 생성하든지, 초안으로 시작하게 해줍니다. 왜냐면, `Post`의 `state` 필드가 private하기 때문이죠, `Post`는 다른 상태로는 생성할 수 없습니다! `Post::new` 함수에서, 우리는 `content` 필드를 새로운, 빈 `문자열`로 설정합니다.

### 게시물 컨텐츠의 글을 저장하기

항목 	17-11이 보여준 것은 우리가 `add_text`로 명명된 메소드를 호출가능하게 하는 것과 그에 `&str`을 넘겨 블로그 게시물의 컨텐츠 글에 추가하게 하는 것입니다. 우리는 이를 `content` 필드를 `pub`으로 노출시키기 보다는 메소드로 구현할 겁니다. 이는 우리가 나중에 구현한 메소드가 `content` 필드의 데이터를 읽는 방식을 제어할 수 있음을 의미합니다. `add_text` 메소드는 매우 직관적이니까, 항목 17-13에서 `impl Post` 블록에 구현을 추가해봅시다!

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

<span class="caption">항목 17-13: `add_text` 메소드를 구현하여 게시물의 `content`에 글 추가하기</span>

`add_text`메소드 가변 참조자 `self`를 취하는데, 그 이유는 우리가 호출하는 `add_text`이 존재하는 `Post` 인스턴스를 변경하게 되기 때문입니다. 이 때 우리는 `push_str`을 `content`의 `String`에 호출하며 `text`를 인자로 전달해 저장된 `content`에 추가합니다. 이 행위는 게시물의 상태와 무관하게 이뤄지므로, 상태 패턴의 일부가 아닙니다. `add_text` 메소드는 `state` 필드와 전혀 상호작용을 하지 않지만, 우리가 지원하고자 하는 행위 요소입니다.

### 게시물 초안이 빈 상태임을 확실히 하기

우리가 `add_text`를 호출한 이후에도 어떤 컨텐츠가 우리의 게시물에 추가될 수 있다고 해도, 여전히 `content` 메소드가 빈 문자열 조각을 반환하길 원하는 이유는 게시물이 아직도 초안 상태이기 때문입니다, 항목 17-11의 8번째 라인처럼요. 당장은 `content` 메소드를 구현하여 최소한도로 필요한 요소를 구현해놓으려고 합니다. 우리는 나중에 게시물의 상태를 변경할 수 있는 기능을 구현하고, 공개될 수 있도록 하려고 합니다. 그 때까지 게시물은 오직 초안 상태로만 존재하기에 게시물 컨텐츠는 항상 비어 있어야 합니다. 항목 17-14는 이를 미리 구현한 내용입니다. 

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

<span class="caption">
항목 17-14: 항상 비어있는 문자열 조각을 반환하는 `Post`의 `content` 메소드를 미리구현하여 추가</span>

`conetnet` 메소드를 추가함으로서, 항목 17-11에서 의도한 모든 작업은 8줄로 늘어납니다.

### 게시물에 대한 리뷰 요청으로 그의 상태 변경

다음으로, 우리는 게시물의 리뷰를 요청하는 기능을 만들어야 하는데, 이를 위해 게시물의 상태를 `Draft`에서 `PendingReview`로 변경해야 합니다. 항목 17-15는 이에 관련된 코드 입니다:

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

<span class="caption">
항목 17-15: `Post`와 `State`에 `request_review` 메소드를 구현하기</span>

우리는 `self`의 가변 참조를 취할 수 있도록 `request_review`란 이름의 공개 메소드를 `Post`에 주어줬습니다. 이후 우리가 `Post`의 현재 상태의 내부 메소드 `request_review`를 호출하면, 이 두 번째 `request_review` 메소드는 현재의 상태를 소멸하고 새로운 상태를 반환합니다. 

우리는 `State` 특성에 `request_review` 메소드를 추가했습니다; 특성을 구현하는 모든 타입은 이제 `request_review` 메소드를 구현해야 합니다. 주목할 점은 메소드의 첫 인자를 `self`, `&self`, 나 `&mut self`를 취하기 보다 `self:Box<Self>`를 가졌다는 겁니다. 이 문법의 의미는 메소드가 오직 `Box`가 지목하는 타입에만 유효함입니다. 해당 문법은 `Box<Self>`의 소유권을 취하고, `Post`의 예전 상태를 무효화하여 새 상태로 변화하게 해줍니다. 

이전 상태를 소멸하기 위해 `request_review` 메소드는 상태 값의 소유권을 취할 필요가 있습니다. 이는 `Post`의 필드 `state`에 `Option`에서 옵니다: 우리는 `take`메소드를 호출하여 `state`필드의 `Some`값을 취하는데, 왜냐면 Rust는 구조체에 존재하지 않는 필드에 대한 접근을 허용하지 않기 때문입니다. 이는 우리가 대여하기 보다는 게시물 밖으로 `state`의 값을  이동시키도록 만듭니다. 이후 우리는 이런 일련의 동작을 통해 게시물의 `state` 값을 설정하려고 합니다.

우리가 필요한 것은 `self.state = self.state.request_review();`처럼 직접적인 방식보다 `state`를 임시로 `None`으로 설정하여 `state` 값의 소유권을 얻는 것 입니다. 이는 `Post`가 이전 `state` 값을 우리가 그를 새 상태로 변경시킨 뒤에는 사용할 수 없도록 확고히 합니다.

`Draft`의 `request_review` 메소드는 새로운 포장된 `PendingReview` 구조체의 새 인스턴스를 반환해야 하며, 이는 게시물이 리뷰를 기다린다는 상태를 표현합니다. `PendingReview` 구조체는 또한 `request_review` 메소드를 구현하지만 어떤 변경도 하지 않습니다. 대신, 그는 자신을 반환하여 우리가 이미 `PendingReview` 상태인 게시물에 대한 리뷰를 요청하면 그는 `PendingReview` 상태 그대로 유지됩니다. 

이제 우리는 상태 패턴의 장점을 알 수 있습니다: `Post`의 `request_review` 메소드는 그의 `state`가 무엇이든 상관없이 동일합니다. 각 상태는 그 자신의 규칙에 따라 맡은 책임을 다할 것 입니다.

우리는 `Post`의 `content` 메소드가 여전히 빈 문자 조각을 반환하도록 나두려고 합니다. 현재 우리는 `PendingReview`  상태를 `Draft` 상태처럼 소유할 수 있게 됐습니다만, `PendingReview`에서도 동일한 동작을 수행하기를 기대합니다. 항목 17-11은 이제 11라인으로 늘었습니다!

### `approve` 메소드를 추가하여 `content`의 동작을 변경하기

`approve` 메소드는 `request_review` 메소드와 유사할겁니다; 이는 `state`를 승인이 된 후에 가져야 할 상태로 변경할 겁니다, 항목 17-16처럼요:

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

<span class="caption">
항목 17-16: `Post`에 `approve` 메소드 구현하고 `State` 특성 구현
</span>

우리는 `approve` 메소드를 `State` 특성에 추가했고 `State`를 구현하는 새 구조체 `Published` 상태도 추가했습니다.

`request_review`와 유사하게, 우리가 `Draft`의 `approve`메소드를 호출하면, 이는 별 효과가 없는데 이유는 이 때 반환되는 것은 `self`이기 때문이죠. 우리가 `approve`를 `PendingReview`에서 호출하면, 박스포장된 `Published` 구조체의 새 인스턴스가 반환됩니다. `Published` 구조체는 `State` 특성을 구현하고, `request_review`와 `approve` 메소드 양 쪽 모두에서 자기 자신을 반환하므로서 게시물은 `Published` 상태를 유지하게 됩니다.

이제 우리가 해야 할 일은 `Post`의 `content` 메소드를 갱신하는 겁니다: 상태가 `Published`이면, 우리는 게시물의 `content` 필드의 값을 반환하려고 합니다; 그렇지 않다면, 우리는 빈 문자 조각을 항목 17-17처럼 반환할겁니다:

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

<span class="caption">
항목 17-17: `Post`의 메소드 `content`를 갱신하여 `State`의 `content`메소드 대리하기</span>

목표하는 바가 이 모든 규칙을 `State`를 구현하는 구조체 내부에 유지하는 것이기 때문에, 우리가 `state`의 값에 `content`메소드를 호출하면서 post 인스턴스(여기서는 `self`)를 인자로 넘기면, 우리는 `state`에 `content` 메소드로부터 반환된 값을 반환하게 됩니다. 

우리는 `Option`의 `as_ref` 메소드를 호출하여 그의 소유권을 빌리지 않고 값을 참조하려고 합니다. 왜냐면 `state`는 `Option<Box<State>>`이니, 우리가 `as_ref`를 호출하면 `Option<&Box<State>>`가 반환되기 때문이며, 우리가 대여한 함수 매개변수인 `&self`로부터는 `state`를 이동시킬 수 없기 때문에 에러를 얻게 될 겁니다.

이 때 우리는 `unwrap`메소드를 호출하면, 패닉을 발생시키지 않을 것을 알고 있는데, 그 이유는 `Post`의 메소드는 그들이 완료됐을때 `state`를 항상 `Some` 값으로 보관하고 있을 것을 확신하고 있기 때문입니다. 이는 우리가 9장에서 얘기했던 "컴파일러보다 더 많은 정보를 당신이 가진 상황" 중 하나로, 우리는 컴파일러는 이해하지 못하지만 우리는 `None` 값이 결코 오지 않을 것이라는 걸 알고 있습니다. 

이런 관점에 따라, 우리가 `&Box<State>`의 `content`를 호출할 때의 강제 역참조는 `&`와 `Box`에 영향을 줘서 `content` 메소드가 궁극적으로는 `State` 특성이 구현된 타입에 대해 호출되게 됩니다. 무슨 뜻이냐면, 우리에게 필요한 것은 `content`를 `State` 특성의 정의에 추가하는 것으로,  우리가 가진 상태에 따라 어떤 컨텐츠를 반환할지에 대한 로직을 여기에 삽입할 겁니다; 항목 17-18처럼요.

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

<span class="caption">
항목 17-18: `State`특성에 `content` 메소드를 추가하기</span>

우리는 기본적인 `content` 메소드를 구현하여 빈 문자 조각을 반환할 겁니다. 이는 우리가 `Draft`와 `PendingReview` 구조체에 `content`를 구현할 필요가 없다는 뜻입니다. `Published` 구조체는 `content`메소드를 재정의하고 `post.content`의 값을 반환할 겁니다. 

주의할 점은 10장에서 우리가 토의했던 대로 이 메소드에 대한 생명주기를 주석으로 달 필요가 있다는 겁니다. 우리는 `post`의 참조를 인자로 취할 것이고 그 `post`의 일부에 대한 참조를 반환할 것이기에, 반환되는 참조에 대한 생명주기는 `post` 인자의 생명주기와 관련이 있습니다.

그리고 끝났습니다—항목 17-11의 모두가 이제 작동합니다! 우리는 블로그 게시물의 작업 흐름을 상태 패턴을 통해 구현해냈습니다. 규칙과 관련있는 로직들은 `Post`에 흩어져있지 않고 상태 객체에 존재합니다.

### 상태 패턴의 기회비용

게시물이 각 상태에 대해 가져야 하는 제각각의 특성이 있는 동작을 캡슐화하여 객체-지향인 상태 패턴을 Rust로 충분히 구현할 수 있음을 보여줬습니다. `Post`의 메소드는 이런 다양한 동작에 대해서 알 수가 없죠. 우리가 코드를 구조화한 방식에 따라, 공개된 게시물이 할 수 있는 다양한 방식을 알기 위해 오직 한 곳만 보면 됩니다: `Published` 구조체의 `State`에 구현된 내용말이죠.

만약 우리가 상태 패턴을 사용하지 않고 다른 방식으로 구현한다면, `Post` 혹은 `main` 코드에서 `match` 표현문을 대신 사용하여 게시물의 상태를 검사하고 이에 따라 해야 할 행동을 변경해야 할지도 모르겠습니다. 이는 우리가 공개된 상태의 게시물의 구현에 대해 알기 위해서 여러 곳을 보고 이해해야 한다는 것을 뜻합니다! 여기에 우리가 상태를 추가하게 된다면 `match` 표현문은 또 다른 갈래를 추가해야 합니다. 

상태 패턴을 통해, `Post` 메소드들과 `Post`를 사용하는 곳에서 `match` 표현식을 사용할 필요가 없고, 새로운 상태를 추가하려면, 그저 새로운 구조체와 구조체에 특성 메소드들을 구현하면 됩니다.

상태 패턴을 사용하면 추가 기능을 구현하기 쉽습니다. 상태 패턴을 사용하여 코드의 단순성을 유지하는 것을 체험해보려면 다음 몇 가지 제안을 시도해보세요:

  * `reject` 메소드를 추가하여 게시물의 상태를 `PendingReview`에서 `Draft`로 변경
  * `Published`로 상태를 변경하기 전에 `approve`가 두 번 호출되도록 강제하기
  * 사용자들에게 게시물이 `Draft` 상태일 때는 오직 글의 추가만 허용하기. 
  힌트: 상태 객체가 content를 변경하는 데애 대한 역할을 하지만 `Post`의 수정하기 위한 역할은 하지 않게

상태 패턴의 단점 중에 하나는, 상태가 상태 간의 전환을 구현하기 때문에, 어떤 상태들은 서로 간에 의존성이 생기게 됩니다. 만약 우리가 `PendingReview`와 `Published` 사이에 `Scheduled`와 같은 상태를 추가하면, `PendingReview`에서 `Scheduled`로 전환되도록 코드를 변경하여야 합니다. `PendingReview`가 변경될 필요 없도록 수정하는 것은 작은 작업이지만, 이는 다른 디자인 패턴으로의 전환을 의미합니다. 

또 다른 단점은 우리가 어떤 로직을 중복시킨 겁니다. 중복의 일부를 제거하려면, 우리는 `State`특성에 `request_review`와 `approve` 메소드를 `self`를 반환하도록 기본 구현을 만들 수 있습니다. 하지만, 이는 특성은 `self`가 확실히 될 고정타입을 알 수 없기 때문에 객체 안정성을 해칩니다. 우리는 `State`를 특성 객체로 사용하길 원하기에, 그의 메소드들은 객체 안전할 필요가 있습니다. 

다른 중복은 `Post`에 메소드 `request_review`와 `approve`처럼 유사한 구현을 갖는 것 입니다. 양 메소드는 `Option`의 필드 `state`의 값에 대해 동일한 메소드의 구현을 대행하여, `state` 필드의 새 값을 결과로 설정합니다. 이 패턴을 따라 `Post`에 다량의 메소드를 구현하게 되면, 반복됨을 제거하기 위해 매크로의 정의를 고려해봐야 할 겁니다. (매크로에 대한 자세한 내용은 부록 D 참조) 

객체 지향의 언어에서 정의하는 것과 동일하게 상태 패턴을 구현 함으로서, 우리가 사용할 수 있는 Rust의 강점을 다루지 못했습니다. 유효하지 않은 상태나 컴파일 타임의 에러가 될 수 있는 `blog` crate에 대한 몇 가지 변경을 살펴보도록 하겠습니다. 

#### 상태들을 엔코딩하여 타입으로 동작

우리는 다른 기회비용을 얻기 위해 상태 패턴을 고찰하여 보여주려고 합니다. 오히려 상태와 전환을 완전히 캡슐화하여 외부의 코드들이 이를 알 수 없으므로, 상태를 다른 타입들로 인코드 하려고 합니다. 따라서 Rust의 타입 체크 시스템은 공개된 게시물만 허용된 곳에서 게시물 초안을 사용하려는 시도를 방지하기 위해 컴파일 에러를 발생시킵니다. 

항목 17-11의 `main` 첫 부분을 주의깊게 살펴보세요:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

우리는 여전히 `Post::new`를 사용하여 초안 상태의 새 게시물을 생성할 수 있도록 하며 게시물의 컨텐츠에 새 글을 추가할 수 있는 기능을 허용합니다. 그 대신 초안 게시물의 `content` 메소드는 빈 문자열을 반환하지만, 대신 우리는 초안 게시물이 `content` 메소드를 갖지 않도록 만들려고 합니다. 이러면, 우리가 만약 초안 게시물의 컨텐츠를 얻고자 시도할 때, 해당 메소드가 존재하지 않는다는 컴파일 에러를 얻게 될 겁니다. 결과적으로, 혹여 제작 중인 컨텐츠의 초안을 얻게 되는 일이 불가능하게 됩니다. 왜냐면 아예 컴파일이 되지 않거든요! 항목 17-19에서 `Post` 구조체와 `DraftPost` 구조체의 정의와 각각의 메소드를 보여줍니다:

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

<span class="caption">
항목 17-19: `content` 메소드가 있는 `Post`와 `content` 메소드가 없는 `DraftPost`</span>

`Post`와 `DraftPost` 구조체 모두 private `content` 필드가 있어 블로그 게시물의 글을 보관합니다. 구조체가 더 이상 `state`필드를 갖지 않는 이유는 상태의 인코딩을 구조체의 타입으로 이동시켰기 때문입니다. `Post` 구조체는 공개된 게시물을 나타낼 것이고, 그의 `content` 메소드는 `content`를 반환할 겁니다.

우리는 여전히 `Post::new` 함수를 유지하지만, `Post`의 인스턴스를 반환하지 않고, 대신 `DraftPost`를 반환합니다. 왜냐면 `content`는 private이고 `Post`를 반환할 어떤 함수도 존재하지 않기 때문에, `Post`의 인스턴스를 생성하는 것은 당장은 불가합니다. 

`DraftPost` 구조체가 가진 `add_text`메소드로, 우리는 전처럼 `content`에 글을 추가할 수 있지만, 주의할 것은 `DraftPost`는 `content` 메소드의 정의를 갖지 않습니다! 그래서 현재 프로그램은 모든 게시물이 초안 게시물로 생성될 것이라고 확고히 할 수 있으며 초안 게시물들은 그들의 컨텐츠를 출력할 수 없습니다. 이 제약사항을 벗어나는 어떤 시도도 컴파일러 에러를 얻게 될 겁니다. 

#### 다른 타입으로 변화하는 전환 구현

그래서 우리는 어떻게 공개된 게시물을 얻을 수 있을까? 우리는 게시물 초안이 공개되기 전에 리뷰와 승인을 받게 강제하고 싶습니다. 리뷰 상태인 게시물은 어떤 컨텐츠도 공개되서는 안되구요. 이런 제약사항들을 새 구조체 `PendingReviewPost`를 추가하여 구현해봅시다. `request_review` 메소드를 `DraftPost`에 구현하여 `PendingReviewPost`를 반환하도록 정의하고, `approve`메소드를 `PendingReviewPost`에 정의하여 `Post`를 반환하도록, 항목 17-20처럼 만들어 보겠습니다.

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

<span class="caption">
항목 17-20: `PendingReviewPost`는 `DraftPost`에 `request_review`의 호출을 통해 생성하여 얻을 수 있고, `approve` 메소드는 `PendingReviewPost`를 공개된 `Post`로 전환합니다. </span>

`request_review`와 `approve` 메소드는 `self`의 소유권을 취합니다, 이는 각각의 `DraftPost`와 `PendingReviewPost`의 인스턴스를 소비하여 그를 `PendingReviewPost`와 공개된 `Post`로 변화시킵니다. 여기서, 우리는 `DraftPost` 인스턴스를 `request_review`를 호출한 후에 유지하는 것을 원치 않습니다. `PendingReviewPost` 구조체는 `content`메소드의 정의를 갖지 않기 때문에, 그의 컨텐츠를 읽으려는 시도는 `DraftPost`와 마찬가지로 컴파일 에러를 발생시킵니다. `content` 메소드를 정의하고 있는 공개된 `Post` 인스턴스를 얻을 수 있는 유일한 방법은 `PendingReiewPost`에 `approve` 메소드를 호출하는 것이고, `PendingReviewPost`를 얻을 수 있는 유일한 방법은 `DraftPost`에 `request_review`를 호출하는 것이기에, 우리는 이제 블로그 게시물의 작업 흐름을 타입 시스템으로 인코드 해야 합니다. 

하지만 우리는 여전히 `main`에 약간의 변화를 줘야 합니다. `request_review`와 `approve` 메소드는 구조체를 변경하지 않고 새 인스턴스를 반환하기 때문에, 우리는 `let post =`를 추가하여 shadowing 대입을 통해 반환되는 인스턴스를 보관해야 합니다. 또한 초안과 리뷰 중인 게시물의 컨텐츠가 빈 문자열이라고 assert할 수 없기 때문에, assert가 필요하지 않습니다: 더 이상 이들 상태들에서 게시물이ㅡ 컨텐츠를 사용하려는 시도들은 컴파일 할 수 없습니다. 갱신된 `main`코드는 항목 17-21에서 보여줍니다:

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

<span class="caption">
항목 17-21: `main`을 수정하여 블로그 게시물 작업 흐름의 새 구현을 사용하기</span>

우리가 해야하는 변경은 `main`에서 `post`의 재할당으로 이는 더 이상 구현이 객체-지향 상태 패턴을 따르지 않는다는 것을 의미합니다: 상태간의 전환은 더 이상 `Post`의 구현 전체에 대해 캡슐화되지 않습니다. 하지만, 우리가 얻은 것은 비허용된 상태가 불가능해지는 것으로 컴파일 타임에 타입 시스템과 타입 체킹을 하기 때문입죠! 이를 통해 공개되지 않은 게시물의 내용이 보여진다거나 하는 버그들은 제품화가 되기 전에 파악될 것을 확신할 수 있습니다. 

이번 섹션 시작부에 우리가 언급했던 추가 제약사항으로 건의한 작업을 항목 17-20 이후의 `blog` crate에서 수행하여 이번 버전의 코드 디자인에 대해 생각해보세요. 몇 가지 작업은 이번 디자인에서 이미 완료됐음을 알려드립니다. 

우리는 Rust를 쓰더라도 객체-지향 디자인 패턴 뿐 아니라 상태를 타입 시스템으로 인코딩하는 다른 패턴들의 구현이 충분히 가능함을 보여줬습니다. 이 패턴들은 서로 다른 기회비용을 가졌습니다. 여러분이 객체 지향 패턴에 익숙하더라도, 컴파일 타임의 버그 방지 같은 Rust의 기능들로 이득을 얻을 수 있는 방식으로 다시 생각해보자. 객체지향 패턴은 Rust가 제공하는 소유권 같이 객체지향 언어에서는 갖지 못한 기능들 덕분에 늘 최고의 해결책이지는 않습니다.

## 요약

이 장을 읽은 이후에 여러분이 Rust가 객체지향 언어라고 생각하든 아니든, 여러분은 특성 객체를 사용하여 객체지향 기능을 Rust에서 사용할 수 있다는 것을 알았을 겁니다. 동적 변환은 여러분의 코드에 유연성을 주긴 하지만 실행 성능을 약간 희생해야 합니다. 여러분은 이런 객체지향 패턴을 구현하여 얻은 유연성을 코드의 유지관리에 도움을 받을 수 있습니다. Rust는 소유권과 같은 객체지향 언어들이 갖지 못한 기능들도 갖고 있습니다. 객체지향 패턴은 Rust의 강점을 이용하는 최고의 방법은 아닙니다만, 선택 가능한 옵션입니다.

다음으로, 우리는 Rust의 기능으로 높은 유연성을 가능케하는 패턴들에 대해 살펴보고자 합니다. 우리는 이 책에서 간단히 살펴보긴 했지만, 아직 그들의 모든 능력을 살펴본건 아닙니다. 어서 가즈아!
