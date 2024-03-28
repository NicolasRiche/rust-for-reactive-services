use std::collections::HashMap;


trait Method {}
struct NoMethod();
struct Get();
struct Post();

impl Method for NoMethod {}
impl Method for Get {}
impl Method for Post {}

trait Payload {}
struct EmptyPayload();
struct NonEmptyPayload(String);

impl Payload for EmptyPayload {}
impl Payload for NonEmptyPayload {}

pub struct HttpGetRequest {
  headers: HashMap<String, String>
}

pub struct HttpPostRequest {
  payload: NonEmptyPayload,
  headers: HashMap<String, String>
}


pub struct HttpRequestBuilder<M: Method, P: Payload> {
    method: M,
    payload: P,
    headers: HashMap<String, String>
}

impl HttpRequestBuilder<NoMethod, EmptyPayload> {
  // We start with no method selected, and empty payload
  pub fn new() -> HttpRequestBuilder<NoMethod, EmptyPayload> {
    Self{
      method: NoMethod(),
      payload: EmptyPayload(),
      headers: HashMap::default()
    }
  }

  // Then the only available function is the select a method
  // Either Get
  pub fn with_get_method(self) -> HttpRequestBuilder<Get, EmptyPayload> {
    HttpRequestBuilder {
        method: Get(),
        payload: self.payload,
        headers: self.headers
    }
  }
  // Or Post
  pub fn with_post_method(self) -> HttpRequestBuilder<Post, EmptyPayload> {
    HttpRequestBuilder {
      method: Post(),
      payload: self.payload,
      headers: self.headers
    }
  }
}

impl HttpRequestBuilder<Post, EmptyPayload> {
  // Only if method is POST, I can add a payload
  pub fn with_payload(self, value: String) -> HttpRequestBuilder<Post, NonEmptyPayload> {
    HttpRequestBuilder {
      method: self.method,
      payload: NonEmptyPayload(value),
      headers: self.headers
    }
  }
}

// We can make functions available in any state
// Useful to avoid to duplicate the impl in each individual state.
impl <M,P> HttpRequestBuilder<M,P> where M:Method, P:Payload {
    pub fn add_header(&mut self, header: String, value: String) -> &mut Self {
      self.headers.insert(header, value);
      self
    }
}

impl <P> HttpRequestBuilder<Post,P> where P:Payload {
  // If I send a POST, I often have to set the content-type too
  pub fn with_content_type(&mut self, content_type: String) -> &mut Self {
    self.headers.insert("Content-Type".to_string(), content_type);
    self
  }
}

pub fn test() {

  let post_request = HttpRequestBuilder::new()
    .with_post_method()
    .with_content_type("text/plain".to_string())
    .with_payload("Try to add a body to a post method".to_string())
    .add_header("Authorization".to_string(), "token".to_string());

  let get_request = HttpRequestBuilder::new()
    .add_header("Authorization".to_string(), "token".to_string())
    .with_get_method()
    .with_payload("Try to add a body to a get method".to_string());
    
   
}

