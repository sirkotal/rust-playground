pub fn verse(n: u32) -> String {
    let mut v = String::new();
    if (n == 2) {
        v.push_str(&n.to_string());
        v.push_str(" bottles of beer on the wall, ");
        v.push_str(&n.to_string());
        v.push_str(" bottles of beer.\n");
        v.push_str("Take one down and pass it around, ");
        v.push_str(&(n - 1).to_string());
        v.push_str(" bottle of beer on the wall.\n");
        return v;
    }

    if (n == 1) {
        v.push_str(&n.to_string());
        v.push_str(" bottle of beer on the wall, ");
        v.push_str(&n.to_string());
        v.push_str(" bottle of beer.\n");
        v.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n");
        return v;
    }

    if (n == 0) {
        v.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
        v.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
        return v;
    }

    v.push_str(&n.to_string());
    v.push_str(" bottles of beer on the wall, ");
    v.push_str(&n.to_string());
    v.push_str(" bottles of beer.\n");
    v.push_str("Take one down and pass it around, ");
    v.push_str(&(n - 1).to_string());
    v.push_str(" bottles of beer on the wall.\n");
    return v;
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = start;
    let mut song = String::new();
    while (s >= end) {
        song.push_str(&verse(s));

        if (s != end) {
            song.push_str("\n");
        }

        if (s == 0) {
            break;
        }
        
        s -= 1;
    }
    return song;
}
