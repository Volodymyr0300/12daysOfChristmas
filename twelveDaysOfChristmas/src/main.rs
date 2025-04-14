fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", ordinals[day]);

        for gift_index in (0..=day).rev() {
            if gift_index == 0 && day !=0 {
                println!("and {}", gifts[0]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }
        println!();
    }
}
