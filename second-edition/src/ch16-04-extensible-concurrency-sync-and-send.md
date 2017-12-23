## Расширения возможностей многопоточных программ с помощью типажей `Sync` и `Send`

Как вы уже успели заметить в самом языке Rust нет языковых конструкций для работы
с многопоточностью. Всё опции становятся доступны при использовании библиотек.

### Типаж `Send` индикатор владения, которое может быть передано от одного потока к другому

Почти в все типы в стандартной библиотеке Rust реализуют типаж `Send`. Если он
не реализует (например, `Rc<T>` - значит он не должен использоваться в многопоточной
среде).

### Реализация типажа `Sync` говорит о том, что это его можно использовать в многопоточной среде

Реализация типажа `Sync` - индикатор того, что тип потокобезопасный. Т.е.
если тип `T` является `Sync`если `&T` является `Send`.
The `Sync` marker trait indicates that a type is safe to have references to a
value from multiple threads. Another way to say this is for any type `T`, `T`
is `Sync` if `&T` (a reference to `T`) is `Send` so that the reference can be
sent safely to another thread. In a similar manner as `Send`, primitive types
are `Sync` and types composed entirely of types that are `Sync` are also `Sync`.

Умные указатели `Rc<T>`, `RefCell<T>` и производные от `Cell<T>` не являются
потокобезопасными  - не реализуют `Sync`.

### Реализация `Send` и `Sync`в ручном режиме - небезопасна

Обычно Вам не нужно реализовывать типажи `Send` и `Sync`. Т.к. это типажи-маркеры
они не имеют методов.

## Итого

Компилятор проводит работу благодаря, которой ваш код может быть потокобезопасным.
Реализовав все его требования вы можете быть уверены, что ваша многопоточное решение
будет работать безотказно.

Next, let’s talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger, and how Rust’s idioms relate to those you
might be familiar with from Object Oriented Programming.
В следующей главе мы рассмотрим вопросы архитектуру приложений. Рассмотрим связи
различных концепций программирования реализованных в Rust для выбора подходящих решений.
