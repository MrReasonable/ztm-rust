// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug, Clone, Copy)]
struct LuggageId(usize);
#[derive(Debug)]
struct Luggage(LuggageId);
#[derive(Debug)]
struct CheckIn(LuggageId);
#[derive(Debug)]
struct OnLoad(LuggageId);
#[derive(Debug)]
struct OffLoad(LuggageId);
#[derive(Debug)]
struct AwaitPickup(LuggageId);
#[derive(Debug)]
struct EndCustody(LuggageId);

impl Luggage {
    pub fn new(id: LuggageId) -> Self {
        Self(id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwaitPickup {
        AwaitPickup(self.0)
    }
}

impl AwaitPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}

fn main() {
    let id = LuggageId(10);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel().pickup();
    println!("{:?}", luggage);
}
