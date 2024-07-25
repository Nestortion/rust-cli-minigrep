use rust_cli::{search, search_insensitive};

#[test]
fn search_test() {
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    let word_to_search = "nobody";

    assert!(search(contents, word_to_search).len() == 2);
}

#[test]
fn search_test_insensitive() {
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    let word_to_search = "NOBODY";

    assert!(search_insensitive(contents, word_to_search).len() == 2);
}
