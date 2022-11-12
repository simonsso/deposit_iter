fn main() {
    println!("Hello, world!");
}
#[derive(Default)]
struct Deposit {
    times: Option<u8>,
}

impl Deposit {
    fn new(t: u8) -> Self {
        Deposit { times: Some(t) }
    }
}
impl Iterator for Deposit {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(times) = self.times {
            if times == 0 {
                self.times = None;
                Some(true)
            } else {
                self.times = Some(times.saturating_sub(1));
                Some(false)
            }
        } else {
            None
        }
    }
}


#[test]
fn test_iter(){
    let mut m = Deposit::new(4);
    assert_eq!(Some(false),m.next());
    assert_eq!(Some(false),m.next());
    assert_eq!(Some(false),m.next());
    assert_eq!(Some(false),m.next());
    assert_eq!(Some(true),m.next());
    assert_eq!(None,m.next());
}