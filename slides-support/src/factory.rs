use std::collections::HashMap;


pub struct Factory<T> {
  marker: std::marker::PhantomData<T>
}

impl <T> Factory<T> {
    pub fn new() -> Self {
      Factory{ marker: std::marker::PhantomData::<T>::default()}
    }
}

trait CreateId {
  type Response;

  fn create(&self, input: &str) -> Self::Response;
}

impl CreateId for Factory<i64> {
    type Response = String;

    fn create(&self, input: &str) -> Self::Response {
        todo!()
    }
}


pub fn test() {

  let factory = Factory::<i64>::new();
  let _ = factory.create("test");

  ()
   
}
