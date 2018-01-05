extern crate aggregator;

use aggregator::Summarizable;
use aggregator::Tweet;
use aggregator::DefaultArticle;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let defautl_article = DefaultArticle{};
    println!("default text: {}", defautl_article.summary());
}
