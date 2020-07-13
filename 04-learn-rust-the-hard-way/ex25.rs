fn break_words(stuff: &str) -> Vec<&str> {
    stuff.split(' ').collect()
}

fn sort_words(words: Vec<&str>) -> Vec<&str> {
    // words.sort()
    let mut words_clone = words.clone();
    words_clone.sort();
    words_clone
}

fn print_first_word(words: &mut Vec<&str>) {
    let word = words.remove(0);
    println!("{}", word);
}

fn print_last_word(words: &mut Vec<&str>) {
    let word = words.pop().unwrap();
    println!("{}", word);
}

fn sort_sentence<'a>(sentence: &'a str) -> Vec<&'a str> {
    let words = break_words(sentence);
    sort_words(words)
}

fn print_first_and_last(sentence: &str) {
    let mut words = break_words(sentence);
    print_first_word(&mut words);
    print_last_word(&mut words);
}

fn print_first_and_last_sorted(sentence: &str) {
    let mut words = sort_sentence(sentence);
    print_first_word(&mut words);
    print_last_word(&mut words);
}

fn main() {
    let sentence = "All good things come to those who wait.";
    let mut words = break_words(sentence);
    println!("{:?}", words);
    let mut sorted_words = sort_words(words);
    println!("{:?}", sorted_words);
    // --------------------------------
    // In fact, there is not this sentence
    // But without it I can't make it pass the check of compiler
    let mut words = break_words(sentence);
    // --------------------------------
    print_first_word(&mut words);
    print_last_word(&mut words);
    print_first_word(&mut sorted_words);
    print_last_word(&mut sorted_words);
    println!("{:?}", sorted_words);
    let sorted_words = sort_sentence(sentence);
    println!("{:?}", sorted_words);
    print_first_and_last(sentence);
    print_first_and_last_sorted(sentence);
}

