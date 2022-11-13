#![no_std]


#[derive(Default)]
pub struct Deposit {
    times: Option<usize>,
}

impl Deposit {
    pub fn new(t: usize) -> Self {
        Deposit { times: Some(t) }
    }
}

/// Create an iterator which will yield Some(false) for n times and then Some(true) until finlay yielding None until end of time 
impl Iterator for Deposit {
    type Item = bool;
    /// An iterator which will yield Some(false) for n times and then Some(true) until finlay yielding None until end of time 
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

#[test]
fn test_zero(){
    let mut m = Deposit::default();
    assert_eq!(None,m.next());

    let mut m = Deposit::new(0);
    assert_eq!(Some(true),m.next());
    assert_eq!(None,m.next());
}
#[test]
fn test_loop(){
    for (i,m) in Deposit::new(40).enumerate() {
        assert_eq!((i==40),m);
    }

}