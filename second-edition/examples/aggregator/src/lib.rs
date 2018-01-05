use std::cmp::PartialOrd;


pub trait Summarizable {
//  fn summary(&self) -> String;
//  fn summary(&self) -> String {
//      String::from("(Read more...)")
//  }

  fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }

    fn notify<T: Summarizable>(item: T) {
        println!("Breaking news! {}", item.summary());
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}



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

   fn author_summary(&self) -> String{
        format!("{}", self.author)
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

   fn author_summary(&self) -> String{
        format!("@{}", self.username)
   }
}

pub struct WeatherForecast {
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

    fn author_summary(&self) -> String{
         format!("Wheather forecast people")
    }
}

pub struct DefaultArticle {
}
impl Summarizable for DefaultArticle {
  fn author_summary(&self) -> String{
       format!("None")
  }
}
