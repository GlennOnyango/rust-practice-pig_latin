fn main() {
    let word = String::from("apple");

    let first_letter = &word.chars().nth(0).unwrap();

    let last_letters = &word[1..];

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut is_vowel: bool = false;

    for c in &vowels{
        if c == first_letter{
            is_vowel = true;
        }
    }

    if is_vowel{
        println!("{}-hay",word);
    }else {
        println!("{}-{}ay",last_letters,first_letter)
    }
    

}
