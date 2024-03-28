use std::marker::PhantomData;


/*
  Traffic Light with a flashing state.
  Transition to flashing state can only happen from Red.
  Then Flashing can only transition back to Red, to return to regular operation.
  This implementation guarantees at compile time these constraints.

    ┌───────────────────────────┐
    │                           │
    ▼
   Red  ──────► Green ──────► Yellow
  ┌─ ◄┐
  │   │
  ▼   │
 Flashing
 */
#[derive(Default, Clone)]
pub struct TrafficLight<S: TrafficLightState> {
  // PhantomData is a zero-sized marker just to reassures the compiler than S parameter is used.
  // This is made to pretend that the struct owns S
  marker: std::marker::PhantomData<S>
  // As alternative, you can also store the state, depending on you want a zero-sized state marker or to use the state as well
  // state: TrafficLightState
}


// Empty enum is again zero-sized, just exist at compile-time (You can not instantiate a empty enum)
// If you want to store the state, you can use empty struct() or struct with fields instead
enum Green {}
enum Yellow {} 
enum Red {} 
enum Flashing {}

pub trait TrafficLightState {}
impl TrafficLightState for Green {}
impl TrafficLightState for Yellow {}
impl TrafficLightState for Red {}
impl TrafficLightState for Flashing {}

impl TrafficLight<Red> {
    // default new() method build a TrafficLight<Red>
    // Note how self isn't a parameter, because this a constructor
    pub fn new() -> Self {
        TrafficLight{marker: PhantomData}
    }

    pub fn to_green(self) -> TrafficLight<Green> {
      TrafficLight{marker: PhantomData}
    }

    // Red state is only state where we can transition to Flashing state
    pub fn to_flashing(self) -> TrafficLight<Flashing> {
      TrafficLight{marker: PhantomData}
    }
}

impl TrafficLight<Flashing> {
    pub fn to_red(self) -> TrafficLight<Red> {
      TrafficLight{marker: PhantomData}
    }
}

impl TrafficLight<Green> {
    pub fn to_yellow(self) -> TrafficLight<Yellow> {
      TrafficLight{marker: PhantomData}
    }
}

impl TrafficLight<Yellow> {
    pub fn to_red(self) -> TrafficLight<Red> {
      TrafficLight{marker: PhantomData}
    }
}

pub fn usage() {

    TrafficLight::new()
      .to_green()
      .to_yellow()
      .to_flashing()

       // Compiler error:
        /*
          no method named `to_flashing` found for struct `TrafficLight<Yellow>` in the current scope
          the method was found for - `TrafficLight<Red>`
         */
   
}

