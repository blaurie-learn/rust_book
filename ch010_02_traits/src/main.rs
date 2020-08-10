//traits tell the compiler what type of behavior a type has (similar to Javas Interfaces)
//We can also use trait bounds to specify that a generic has specific behavior

//defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

//implementing a trait on a type:
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//implementing a trait is similar to implementing regular methods except that we impl the TraitName
//for a type.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//note that we can implement only traits only if the trait or type is local to our crate.
//For example, we can't implement Display on Vec<T> in our own crate.

//sometimes it is useful to provide a default implementation
pub trait Summary {
    fn summarize_author(&self) -> String;

    //of course, we can call other methods in this trait from one another when providing a default
    //impl
    fn summarize(&self) -> String {
        String::from("(Read More...) from {}", self.summarize_author())
    }
}


//traits can also be used as parameter types instead of a concrete type
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//similarly, we can also bound a generic by a trait:
pub fn notify<T: Summary>(item: &t) {
    println!("Breaking news! {}", item.summarize());
}

//You can also specify multiple bounds:
//pub fn notify(item: &(impl Summary + Display)) ...
//pub fn notify<T: Summary + Display>(item: T) ...

//The "where" clause can make trait bounds more clear:
//old: pub fn some_func<T: Distplay + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 ...
pub fn some_func<T, U>(t: T, u: U) -> i32
    where T: Display + Clone
            U: Clone + Display
{
    //...
}

//And we can also have return types that require a trait type:
fn returns_summarizable() -> impl Summary { 
    //...
}   //The restriction here is that the function can only return a single type. For example, we couldnt
    //implement Summary on a secont type and then return both types wrapped in an if block..


fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("{}", article.summarize());
}
