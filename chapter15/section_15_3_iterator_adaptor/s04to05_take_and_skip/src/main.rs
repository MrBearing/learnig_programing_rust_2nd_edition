fn main() {
    let message = "To: jimb\r\n\
    From: superego <editor@oreilly.com>\r\n\
    \r\n\
    Did you get any writing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";

    for header in message.lines()
        .take_while(|line| !line.is_empty()) {
        println!("{}", header);
    }

    for body in message.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1) {
        println!("{}", body);
    }


}
