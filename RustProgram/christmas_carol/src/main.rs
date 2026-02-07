fn main() {
    let lyrics=[
        "a partridge in a pear tree",
        "two turtledoves and ",
        "three French hens, ",
        "four calling birds, ",
        "five golden rings, ",
        "six geese a laying, ",
        "seven swans a-swimming",
        "eight maids a-milking, ",
        "nine ladies dancing, ",
        "ten lords a-leaping, ",
        "eleven pipers piping",
        "twelve drummers drumming, "
    ];

    let orders=[
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eigth",
        "Nineth",
        "Tenth",
        "Eleventh",
        "Twelfth"
    ];
    for i in 0..=11 {
        println!("On the {} Day of Christmas", orders[i]);
        println!("my true love gave to me");
        for j in (0..=i).rev() {
            println!("{}", lyrics[j])
        }

    }
}