//! Integration tests for the crate.
//!
//! These all live in a *single* integration test crate, `tests/integration`,
//! because each integration test is a dedicated binary crate which has to be
//! compiled separately. While that is not really a problem for a crate this
//! small, we have chosen to follow this “best practice” here as a good example.
//!
//! For more details on why you might prefer this pattern see [this post][post].
//!
//! [post]: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html

use std::{pin::Pin, time::Duration};

use futures::Future;
use trpl::{Either, Receiver, Sender};

/// We initially named the function `run` and an online version of the async chapter
/// was released with that name, so we want to keep it working. We decided to rename
/// `run` to be `block_on` to more closely match other crates' names, so most of the
/// tests now use the `block_on` name.
#[test]
fn using_run_works() {
    let val = trpl::run(async { "Hello" });
    assert_eq!(val, "Hello");
}

/// This test is foundational for all the others, as they depend on `block_on`.
///
/// If we mess this up, *all* the tests below will fail -- so by the same token,
/// if all the tests below are failing, this one probably is too; fix it and the
/// others will likely start working again.
#[test]
fn re_exported_block_on_works() {
    let val = trpl::block_on(async { "Hello" });
    assert_eq!(val, "Hello");
}

#[test]
fn re_exported_spawn_works() {
    let result = trpl::block_on(async {
        let handle_a = trpl::spawn_task(async { "Hello" });
        let handle_b = trpl::spawn_task(async { "Goodbye" });
        vec![handle_a.await.unwrap(), handle_b.await.unwrap()]
    });

    assert_eq!(result, vec!["Hello", "Goodbye"]);
}

#[test]
fn re_exported_sleep_works() {
    let val = trpl::block_on(async {
        trpl::sleep(Duration::from_micros(1)).await;
        "Done!"
    });
    assert_eq!(val, "Done!");
}

#[test]
fn re_exported_channel_apis_work() {
    trpl::block_on(async {
        // Explicitly naming the type to confirm the re-exports are aligned.
        let (tx, mut rx): (Sender<&str>, Receiver<&str>) = trpl::channel();

        tx.send("Hello").unwrap();
        trpl::sleep(Duration::from_millis(1)).await;
        tx.send("Goodbye").unwrap();
        drop(tx);

        assert_eq!(rx.recv().await, Some("Hello"));
        assert_eq!(rx.recv().await, Some("Goodbye"));
        assert_eq!(rx.recv().await, None);
    });
}

mod re_exported_join_apis_work {
    use super::*;

    #[test]
    fn join_fn() {
        let result = trpl::block_on(async {
            let a = async { 1 };
            let b = async { 2 };
            trpl::join(a, b).await
        });

        assert_eq!(result, (1, 2));
    }

    #[test]
    fn join3_fn() {
        let result = trpl::block_on(async {
            let a = async { 1 };
            let b = async { 2 };
            let c = async { 3 };

            trpl::join3(a, b, c).await
        });

        assert_eq!(result, (1, 2, 3));
    }

    #[test]
    fn join_all_fn() {
        let result = trpl::block_on(async {
            let a = async { format!("{}", 1) };

            let b = async { "Hello".to_string() };

            let outer = String::from("World");
            let c = async move { outer.to_string() };

            let futures: Vec<Pin<Box<dyn Future<Output = String>>>> =
                vec![Box::pin(a), Box::pin(b), Box::pin(c)];

            trpl::join_all(futures).await
        });

        assert_eq!(
            result,
            vec![
                String::from("1"),
                String::from("Hello"),
                String::from("World")
            ]
        );
    }

    #[test]
    fn join_macro() {
        let result = trpl::block_on(async {
            let a = async { 1 };
            let b = async { "Hello" };

            let outer = vec![String::from("World")];
            let c = async move { outer };

            trpl::join!(a, b, c)
        });

        assert_eq!(result, (1, "Hello", vec![String::from("World")]));
    }
}

#[test]
fn select() {
    #[derive(Debug, PartialEq)]
    struct Slow;

    #[derive(Debug, PartialEq)]
    struct Fast;

    let val = trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(1_000)).await;
            Slow
        };

        let fast = async {
            trpl::sleep(Duration::from_millis(1)).await;
            Fast
        };

        trpl::select(slow, fast).await
    });

    assert!(matches!(val, Either::Right(Fast)));
}

#[test]
fn race_continues_to_work() {
    #[derive(Debug, PartialEq)]
    struct Slow;

    #[derive(Debug, PartialEq)]
    struct Fast;

    let val = trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(1_000)).await;
            Slow
        };

        let fast = async {
            trpl::sleep(Duration::from_millis(1)).await;
            Fast
        };

        trpl::race(slow, fast).await
    });

    assert!(matches!(val, Either::Right(Fast)));
}

#[test]
fn yield_now() {
    let result = trpl::block_on(async {
        trpl::yield_now().await;
        "done"
    });

    assert_eq!(result, "done");
}

#[test]
fn read_to_string() {
    let result = trpl::block_on(async {
        trpl::read_to_string("tests/integration/to-read.txt")
            .await
            .unwrap()
    });

    assert_eq!(result, String::from("This is some text!\n"));
}

#[test]
fn stream_iter() {
    use trpl::StreamExt;

    let result = trpl::block_on(async {
        let ns = vec![1, 2, 3];
        let mut stream = trpl::stream_from_iter(ns);
        let mut result = vec![];
        while let Some(n) = stream.next().await {
            result.push(format!("{n}"));
        }
        result
    });

    assert_eq!(
        result,
        vec![String::from("1"), String::from("2"), String::from("3")]
    )
}

#[test]
fn receiver_stream() {
    use trpl::ReceiverStream;
    use trpl::StreamExt;

    let result: Vec<u32> = trpl::block_on(async {
        println!("startup");
        let (tx, rx) = trpl::channel();
        let rx_stream = ReceiverStream::new(rx);
        println!("sending 123");
        tx.send(123).unwrap();
        drop(tx); // So the receiver channel closes!

        rx_stream.collect().await
    });

    assert_eq!(result, vec![123]);
}

#[test]
fn re_exported_interval_stream_works() {
    use trpl::{IntervalStream, StreamExt};

    trpl::block_on(async {
        let mut interval_stream =
            IntervalStream::new(trpl::interval(Duration::from_millis(1)))
                .take(1);

        assert!(interval_stream.next().await.is_some());
        assert!(interval_stream.next().await.is_none());
    });
}

#[test]
fn re_exported_html() {
    use trpl::Html;

    let doc = Html::parse(
        "<html><head><title></title></head><body><p>Hello!</p></body></html>",
    );
    let p = doc.select_first("p").map(|el| el.inner_html());
    assert_eq!(p, Some(String::from("Hello!")));
}
