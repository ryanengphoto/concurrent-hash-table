fn main() {
    // A small, friendly CLI that prints a belly-wash routine.
    // Kept intentionally simple per user request: "wash mah belly" -> "in rust".
    let instructions = r#"
Belly-wash routine (gentle):

1) Use warm, not hot water to wet the belly.
2) Apply a small amount of mild soap or body wash to your hands or a soft washcloth.
3) Gently rub in circular motions over the belly for 20–30 seconds. Don't scrub if skin is irritated.
4) Rinse thoroughly with warm water.
5) Pat dry with a clean towel — don't rub.
6) Moisturize if your skin feels dry.

Stay gentle and stop if you feel pain or irritation.
"#;

    println!("{}", instructions);
}
