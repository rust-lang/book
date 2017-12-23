## Защита от создания ссылочного зацикливания и утечки памяти

Компилятор обеспечивает множеством различных защит от ошибок: от недействительных
ссылок, эфект гонки. Также весьма удобна система обеспечения очистки ресурсов памяти
(что также называют утечкой памяти). В тоже время, компилятор не может гарантировать,
что это невозможно. Иными словами утечка памяти может быть безопасной.

Используя умные указатели `Rc<T>` и `RefCell<T>` возможно создать цепочки ссылок,
где элементы циклично ссылаются друг на друга. Это плохая ситуация, т.к. количество
ссылок каждого элемента никогда не достигнет 0 и, следовательно, постоянно будет
находится в памяти. Давайте разберёмся, как это происходит и постараемся найти
пути предотвращения.

В примере кода 15-16 мы будем использовать другой вариант определения `List`.
Мы будем снова сохранять значение `i32` в первом элементе. Второй элемент теперь
будет `RefCell<Rc<List>>`. Вместо изменения значения первого элемента мы будем
изменять второй. Мы также добавим метод `tail` для удобного доступак к второму
элементу:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<span class="caption">код 15-16: определение списка сons, который содержит `RefCell`.
Мы можетм изменять `Cons` значение, на которое элемент ссылается</span>

Далее, в коде 15-17, мы создадим экземпляр `List` и сохраним его в переменную `a`,
которая изначально будет иметь значения `5, Nil`. Далее, мы создаём переменную
`b` содержащую 10 и ссылку на `a`. И в конце мы изменяем `a` так, что она ссылается
на `b` вместо `Nil`. Так мы создаём зацикливание ссылок:

<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# enum List {
#     Cons(i32, RefCell<Rc<List>>),
#     Nil,
# }
#
# impl List {
#     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

<span class="caption">код 15-17: создание циклической ссылки</span>

Мы использовали метод `tail` для получения ссылки на `RefCell` в `a`, которую мы
поместили в переменную `link`. Далее, мы использовали метод `borrow_mut` для получения
ссылки на `RefCell` в для изменения экземпляра `Rc`, который содержал `Nil` на
`Rc` в `b`. В результате мы создали следующее (15-18):

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-18: A reference cycle of lists `a` and `b`
pointing to each other</span>

Если вы раскоментируете посследнюю строку с вызовом макроса `println!` вы получите
ошибку переполнения (overflow).

Посмотрите на результат вывода на консоль! Защита сработала - ничего страшного не
случилось, но это говорит о более сложной проблема - программа используем больше
памяти, чем ей нужно.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. But it’s not impossible: preventing memory leaks in the form of reference
cycles is not one of the guarantees Rust makes. If you have `RefCell<T>` values
that contain `Rc<T>` values or similar nested combinations of types with
interior mutability and reference counting, be aware that you’ll have to ensure
that you don’t create cycles. In the example in Listing 15-14, the solution
would probably be to not write code that could create cycles like this, since
we do want `Cons` variants to own the list they point to.

With data structures like graphs, it’s sometimes necessary to have references
that create cycles in order to have parent nodes point to their children and
children nodes point back in the opposite direction to their parents, for
example. If one of the directions is expressing ownership and the other isn’t,
one way of being able to model the relationship of the data without creating
reference cycles and memory leaks is using `Weak<T>`. Let’s explore that next!

### Предотвращение циклических ссылок: замена умного указателя `Rc<T>` на `Weak<T>`

Стандартная библиотека Rust предоставляет умный указатель `Weak<T>`. Его необходимо
использовать для предотвращения циклических ссылок. Эта проблема решается путем
однапраленного владения. Мы уже показывали, как клонирования `Rc<T>` увеличивает
`strong_count` ссылки. `Weak<T>` позволяет не увеличивать `strong_count`, а увеличивать
`weak_count` на `Rc`. Когда `Rc` выходит за область видимости внутреннее значение
удаляется если `strong_count` = 0. Для того чтобы получить значение из `Weak<T>`
прежде всего, нам необходимо обновить его с помощью метода `upgrage`. Результатом
будет `Some` или `None`.


<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```
Мы хотим, чтобы `Node` мог иметь своих собственных подчиненных узлов и хотим иметь
возможность непосредственного доступа к ним. Поэтому в `Vec` элементы `Rc<Node>`.
Мы также хотим иметь возможность изменять узлы и их подчёненность, поэтому `Vec`
обёрнут умным указателем `RefCell`. В примере 15-19 мы создадим экземпляр `Node`
с именем `leaf`с значением 3 и без подчиненых узлов и другой экземпляр `branch`
со значением 5 и `leaf`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![leaf.clone()]),
    });
}
```

<span class="caption">Listing 15-19: Создание узла `leaf` и`branch`, где `branch`
родитель `leaf`, но `leaf` не имеет ссылки на `branch`</span>

The `Node` in `leaf` now has two owners: `leaf` and `branch`, since we clone
the `Rc` in `leaf` and store that in `branch`. The `Node` in `branch` knows
it’s related to `leaf` since `branch` has a reference to `leaf` in
`branch.children`. However, `leaf` doesn’t know that it’s related to `branch`,
and we’d like `leaf` to know that `branch` is its parent.

To do that, we’re going to add a `parent` field to our `Node` struct
definition, but what should the type of `parent` be? We know it can’t contain
an `Rc<T>`, since `leaf.parent` would point to `branch` and `branch.children`
contains a pointer to `leaf`, which makes a reference cycle. Neither `leaf` nor
`branch` would get dropped since they would always refer to each other and
their reference counts would never be zero.

So instead of `Rc`, we’re going to make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

This way, a node will be able to refer to its parent node if it has one,
but it does not own its parent. A parent node will be dropped even if
it has child nodes referring to it, as long as it doesn’t have a parent
node as well. Now let’s update `main` to look like Listing 15-20:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

<span class="caption">Listing 15-20: A `leaf` node and a `branch` node where
`leaf` has a `Weak` reference to its parent, `branch`</span>

Creating the `leaf` node looks similar; since it starts out without a parent,
we create a new `Weak` reference instance. When we try to get a reference to
the parent of `leaf` by using the `upgrade` method, we’ll get a `None` value,
as shown by the first `println!` that outputs:

```text
leaf parent = None
```

Similarly, `branch` will also have a new `Weak` reference, since `branch` does
not have a parent node. We still make `leaf` be one of the children of
`branch`. Once we have a new `Node` instance in `branch`, we can modify `leaf`
to have a `Weak` reference to `branch` for its parent. We use the `borrow_mut`
method on the `RefCell` in the `parent` field of `leaf`, then we use the
`Rc::downgrade` function to create a `Weak` reference to `branch` from the `Rc`
in `branch.`

When we print out the parent of `leaf` again, this time we’ll get a `Some`
variant holding `branch`. Also notice we don’t get a cycle printed out that
eventually ends in a stack overflow like we did in Listing 15-14: the `Weak`
references are just printed as `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The fact that we don’t get infinite output (or at least until the stack
overflows) is one way we can see that we don’t have a reference cycle in this
case. Another way we can tell is by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`. In Listing 15-21, let’s create a new
inner scope and move the creation of `branch` in there, so that we can see what
happens when `branch` is created and then dropped when it goes out of scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

<span class="caption">Listing 15-21: Creating `branch` in an inner scope and
examining strong and weak reference counts of `leaf` and `branch`</span>

Right after creating `leaf`, its strong count is 1 (for `leaf` itself) and its
weak count is 0. In the inner scope, after we create `branch` and associate
`leaf` and `branch`, `branch` will have a strong count of 1 (for `branch`
itself) and a weak count of 1 (for `leaf.parent` pointing to `branch` with a
`Weak<T>`). `leaf` will have a strong count of 2, since `branch` now has a
clone the `Rc` of `leaf` stored in `branch.children`. `leaf` still has a weak
count of 0.

When the inner scope ends, `branch` goes out of scope, and its strong count
decreases to 0, so its `Node` gets dropped. The weak count of 1 from
`leaf.parent` has no bearing on whether `Node` gets dropped or not, so we don’t
have a memory leak!

If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again like we did before `leaf` had a parent. At the end of the program,
`leaf` has a strong count of 1 and a weak count of 0, since `leaf` is now the
only thing pointing to it again.

All of the logic managing the counts and whether a value should be dropped or
not was managed by `Rc` and `Weak` and their implementations of the `Drop`
trait. By specifying that the relationship from a child to its parent should be
a `Weak<T>` reference in the definition of `Node`, we’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

## Итоги

Мы рассмотрели, как вы можете использовать различные типы умных указателей для выбора
различных гарантий и компромиссов, в отличии от обычных ссылок.
`Box <T>` имеет известный размер и указывает на данные, выделенные в куче.
`Rc <T>` отслеживает количество ссылок на данные в куче, так что
данные могут иметь несколько владельцев.
`RefCell <T>` с его внутренней изменчивостью дает нам тип, который может использоваться
там, где нам нужен неизменный тип, и применяет правила заимствования во время
выполнения, а не во время компиляции.

Мы также обсудили типажи `Deref` и` Drop`, которые предоставляют функционал умных
указателей. Мы исследовали, как можно создать циклические ссылки, которые могут
вызвать утечку памяти, и как это предотвратить используя `Weak <T>`.

Если эта глава заинтересовала вас, и теперь вы хотите реализовать свои собственные
умные указатели, проверьте [The Nomicon] чтобы узнать от этом функционале подробнее.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/

Далее, давайте поговорим о параллелизме в Rust. Мы даже узнаем о нескольких новых
умных указателях, которые могут помочь нам в этом.
