pub fn reply(message: &str) -> &str {
    let chars =  message.trim().chars();
    let mess: Vec<char> = chars.collect();
    let contains_chars = message.trim().chars().any(|x| x.is_alphabetic());

    let sure = &"Sure.";
    let we = &"Whatever.";
    let fbtw = &"Fine. Be that way!";
    let co = &"Whoa, chill out!";
    let cd = &"Calm down, I know what I'm doing!";

    if  mess.len() == 0 {
        fbtw
    } else if message.to_uppercase() == message && mess.last() == Some(&'?') && contains_chars {
        cd
    } else if mess.last() == Some(&'?') {
        sure
    } else if message.to_uppercase() == message && contains_chars {
        co
    } else {
        we
    }
}
