use posts::Post;

mod posts;

#[tokio::main]
async fn main() {
  // Clear the screen
  print!("\x1B[2J\x1B[1;1H");
  loop {
    println!("--------------------------------------------------------------------");
    println!("1. List all Posts");
    println!("2. Get a Post");
    println!("3. Create a new Post");
    println!("4. Exit");
    let input = get_input(&"Please enter one of the above options.");
    match input.as_str() {
      "1" => list_posts().await,
      "2" => {
        let post_id_input = get_input(&"Please enter a post id");
        match post_id_input.trim().parse::<u8>() {
          Ok(post_id) => get_post(Some(&post_id)).await,
          Err(_) => println!("Post ID not recognized.")
        }
      },
      "3" => create_post().await,
      "4" => {
        println!("Exiting program. Thanks for running!");
        return;
      },
      _ => println!("Option not recognized!")
    }
  }
}

fn get_input(prompt: &str) -> String {
  println!("{}", prompt);
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Unable to read line");
  input.trim().to_string()
}

async fn get_post(id: Option<&u8>) {
  // id is an optional param, so we need to handle the case where no id is
  let real_id: &u8;
  match id {
    Some(real) => real_id = real,
    None => {
      // TODO: replace with logic to figure out what post the user wants to see
      real_id = &1;
    }
  }

  let post_resp = posts::get(real_id).await;
  match post_resp {
    Ok(post) => {
      println!("--------------------------------------------------------------------");
      match post.id {
        Some(id) => println!("{}. {}", id, post.title),
        None => println!("N/A. {}", post.title),
      }
      println!("--------------------------------------------------------------------");
      println!("{}", post.body);
    },
    Err(_) => println!("Post not found!")
  }
}

async fn list_posts() {
  let list_posts_req = posts::list().await;
  match list_posts_req {
    Ok(posts) => {
      println!("--------------------------------------------------------------------");
      println!("All Posts:");
      println!("--------------------------------------------------------------------");
      posts.into_iter().for_each(|post| {
        match post.id {
          Some(id) => println!("{}. {}", id, post.title),
          None => println!("N/A. {}", post.title),
        }
      });
      println!("--------------------------------------------------------------------");
    },
    Err(err) => println!("{}", err)
  }
}

async fn create_post() {
  let title = get_input(&"Please enter post title:");
  let body = get_input(&"Please enter post:");
  let post = Post {
    id: None,
    userId: 1,
    title,
    body
  };
  let resp = posts::create(&post).await;
  match resp {
    Ok(created_post) => {
      println!("--------------------------------------------------------------------");
      println!("Created Post with the following traits:");
      println!("userId: {}", post.userId);
      match created_post.id {
        Some(id) => println!("id: {id}"),
        None => println!("id: N/A")
      }
      println!("title: {}", created_post.title);
      println!("body: {}", created_post.body);
      println!("--------------------------------------------------------------------");

    },
    Err(err) => println!("{}", err)
  }
}