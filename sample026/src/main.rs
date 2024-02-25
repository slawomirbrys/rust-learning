enum VeryLongEnumEvent {
    Start,
    Stop,
    Mid(i32),
    Period { start: i32, duration: i32 },
    Name(String),
}

type Event = VeryLongEnumEvent;

impl Event {
    fn run(&self) {
        match self {
            Self::Start => print(self),
            Self::Stop => print(self),
            Self::Mid(_time) => print(self),
            Self::Period { start: _, duration: _ } => print(self),
            Self::Name(_name) => print(self)
        }
    }
}

fn print(event: &Event) {
    match event {
        Event::Start => println!("Start"),
        Event::Stop => println!("Stop"),
        Event::Mid(time) => println!("Time: {}", time),
        Event::Period { start, duration } => println!("Period started on {} with duration {}", start, duration),
        Event::Name(name) => println!("Event named '{}'", name)
    }
}

fn main() {
    let start = Event::Start;
    let stop = Event::Stop;
    let mid = Event::Mid(10);
    let period = Event::Period { start: 5, duration: 15 };
    let name = Event::Name("some event".to_owned());

    print(&start);
    print(&stop);
    print(&mid);
    print(&period);
    print(&name);

    start.run();
    stop.run();
    mid.run();
    period.run();
    name.run();
}
