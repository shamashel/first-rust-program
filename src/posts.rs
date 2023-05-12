use serde::{Deserialize, Serialize};

/// A single post
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Post {
  /// The ID of the user who created the post
  pub userId: u8,
  /// The unique identifier of the post
  pub id: Option<u8>,
  /// The title of the post
  pub title: String,
  /// The content of the post
  pub body: String,
}

/// The root URL for actions related to posts
const BASE_URL: &str = "https://jsonplaceholder.typicode.com/posts";

pub async fn list() -> Result<Vec<Post>, reqwest::Error> {
  let resp: Vec<Post> = reqwest::get(BASE_URL)
    .await?
    .json::<Vec<Post>>()
    .await?;
  Ok(resp)
}

pub async fn get(id: &u8) -> Result<Post, reqwest::Error> {
  let req_url = format!("{BASE_URL}/{id}");
  let resp: Post = reqwest::get(req_url)
    .await?
    .json::<Post>()
    .await?;
  Ok(resp)
}

pub async fn create(post: &Post) -> Result<Post, reqwest::Error> {
  let http = reqwest::Client::new();
  let resp = http.post(BASE_URL)
    .json(post).send().await?
    .json().await?;
  Ok(resp)
}