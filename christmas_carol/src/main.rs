fn main() {
    let days = [
        ("first", "A partridge in a pear tree"),
        ("second", "Two turtle doves and"),
        ("third", "Three french hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five golden rings"),
        ("sixth", "Six geese a laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming")
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", day.0);

        for count in (0..index + 1).rev() {
            println!("{}", days[count].1);
        };

        println!("");
    };
}
