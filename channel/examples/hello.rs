use crossbeam::crossbeam_channel::unbounded;

fn main() {
    // Create a channel of unbounded capacity.
    let (s, r) = unbounded();

    // Send a message into the channel.
    s.send("Hello, world!").unwrap();

    // Receive the message from the channel.
    assert_eq!(r.recv(), Ok("Hello, world!"));
}
