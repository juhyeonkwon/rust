  pub trait Summarizable {
    fn summary(&self) -> String;
  }
  
  pub trait Summarizable2 {
    fn author_summary(&self) -> String;
  
    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
  }
  
  
  pub struct NewArticle {
    pub headline: String,
    pub location : String,
    pub author: String,
    pub content: String,
  }
  
  pub struct OldArticle {
    pub headline: String,
    pub location : String,
    pub author: String,
    pub content: String,
    pub username: String,

  }
  
  impl Summarizable for NewArticle {
    fn summary(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }
  
  impl Summarizable2 for OldArticle {
    fn author_summary(&self) -> String {
      format!("@{}", self.username)
    }
  }