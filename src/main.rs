// This tells the Rust compiler that there is a module called "game" in a file called "game.rs"
// Conventions like this make it really easy to write code fast. If you want to customize that
// behaviour, Rust gives you the power to do that too.
mod game;

// This is how we "import" a module from the standard library. A module is a group of functions and
// types. "std" stands for "standard library" and "io" stands for "input/output". We will use this
// module to read input from the user of our application.
// The import "self" imports the name "io" itself, and "Write" imports the "Write trait" which we
// need to flush stdout below.
use std::io::{self, Write};
// We use the process::exit function to quit the program when we need to.
use std::process;

// This is how we import names from our own module. Notice that there is no "std::" prefix.
// For more information on modules, see:
// https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
use game::{Game, Piece, Winner, Tiles, MoveError};

// This type is used to provide an error when the user provides an invalid move string. If we
// wanted to avoid copying the invalid string, we could use &str instead and Rust would enforce at
// compile time that the reference remained valid until any instance of InvalidPiece containing it
// goes out of scope. String is used for the same of simplicity. By marking the type stored in this
// struct as `pub`, its value can be freely accessed even in patterns (for example, match
// statements).
#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);

// The main function is where Rust starts running our program from. No code is allowed outside of
// functions so that you can rely on the code in main() running first.
fn main() {
    // The constructor for Game creates a new, empty Tic-Tac-Toe board. `mut` signals that we plan
    // to modify the value of the game variable. Rust will tell us if we forget to use this and
    // warn us if we use it but it isn't needed.
    let mut game = Game::new();

    // Let's continuously prompt the user for input using a loop until the game is finished
    while !game.is_finished() {
        // First, print out the current board
        print_tiles(game.tiles());

        // Inform the user of who's turn it currently is
        // match will enforce that we do not forget any case and the string that it produces will
        // replace `{}` in the printed string.
        println!("Current piece: {}", match game.current_piece() {
            Piece::X => "x",
            Piece::O => "o",
        });

        // prompt_move continuously prompts for a valid move from the user, determines exactly
        // which position on the board that move is referring to, and then returns that move
        let (row, col) = prompt_move();

        // Now that we have a move, let's attempt to make it
        // We use match to account for every case of the result
        match game.make_move(row, col) {
            // If the move is made successfully, we can just move on. You can think of empty
            // curly braces as an "empty expression". We could have also used the unit value `()`.
            Ok(()) => {},
            // Match allows us to conveniently match even nested types like Result and pull out the
            // fields as variables

            // Since we are using is_finished(), it should never be possible for this error to
            // occur. If it does, that means that we (the programmer) did something wrong, not the
            // user. `unreachable!()` works a lot like `println!();` except it exits the program
            // with an error using the message that we provided it. Use `unreachable!()` whenever
            // you encounter a case that you think should never be reached.
            Err(MoveError::GameAlreadyOver) => unreachable!("Game was already over when it should not have been"),
            // Since prompt_move limits the range of what can be returned, it should never allow
            // the user to enter a move that is out of range. Thus, this case is unreachable as
            // well.
            Err(MoveError::InvalidPosition {row, col}) => {
                unreachable!("Should not be able to enter an invalid move, but still got ({}, {})", row, col)
            },

            // Notice that we have already eliminated two possible errors just by structuring our
            // code in a certain way!

            // This is the only case that prompt_move does not account for, so if this happens, we
            // print an error message.
            // The `eprintln!` macro is exactly the same as `println!` except it prints to stderr
            // instead of stdout.
            Err(MoveError::TileNotEmpty {other_piece, row, col}) => eprintln!(
                // Each {} will be replaced with one of the arguments following this string
                "The tile at position {}{} already has piece {} in it!",
                // The row number that is displayed starts at 1, not zero, so we add 1 to get the
                // correct value
                row + 1,
                // `b'A'` produces the ASCII character code for the letter A (i.e. 65)
                // Adding col to it will produce either 65 (A), 66 (B), or 67 (C).
                // `as u8` is necessary because b'A' has type u8 and we can't add u8 to usize
                // without performing a conversion first.
                // Converting it to char using `as char` will get Rust to format this as a
                // character rather than printing the number out
                (b'A' + col as u8) as char,
                // match allows us to print something for each case and will tell us if something
                // ever changes such that this is no longer complete
                match other_piece {
                    Piece::X => "x",
                    Piece::O => "o",
                },
            ),
        }
    }

    // Once the loop is over, the game is finished. Let's output the results

    // First, we'll print the board again
    print_tiles(game.tiles());

    // Then print out which piece won the game
    // We use expect() to express that there should definitely be a winner now and if the winner
    // method returns None, the program should exit with this error
    match game.winner().expect("finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}

// Functions do not need to be ordered in any particular way in the file. That means that Rust
// doesn't suffer from any forward declaration issues where those declarations can get out of sync
// with the actual function implementation.

// This function returns a "tuple" of two values, the row and column of the selected move. Tuples
// are very useful for when you have a function that needs to return two values because it saves
// you from having to define a custom struct just for that purpose.
fn prompt_move() -> (usize, usize) {
    // We'll use `loop` to continuously prompt for input until the user provides what we want. When
    // we get the answer we want, the loop will return the value and it will be used as the return
    // value of this function
    loop {
        // Rust supports convenient `print!` and `println!` macros which support easy and
        // customizable formatting of values from your program. Here we are just using them to
        // prompt for some values that we want the user of our program to provide.
        print!("Enter move (e.g. 1A): ");

        // Line-buffering is when something waits until it sees a new line character before
        // actually writing to its designated destination. Rust's stdout is line-buffered by
        // default, so `print!` does not produce any output unless we "flush" the contents of
        // stdout's buffer in the line below.
        // expect() is how we "ignore" any error that could occur during this process. If an error
        // does occur, the program will exit with the message we provided.
        io::stdout().flush().expect("Failed to flush stdout");

        // The read_line() function is something we defined below to make reading input quick and
        // easy.
        let line = read_line();

        // We delegate reading the line as a move to the parse_move function. That function takes a
        // string and converts it to a "tuple" of two values (row, col). The read_line function
        // returns the type String, but parse_move expects a &str. We use `&` here to convert
        // String to &String. Rust then automatically converts &String to &str. This isn't a
        // special case for just strings, Rust supports a feature called "deref conversions" and
        // this is just a consequence of that. For more information, see:
        // http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
        match parse_move(&line) {
            // The benefit of parse_move returning a Result is that we can't forget to handle the
            // case where the input might be invalid. match gives us a convenient syntax for
            // handling each case.

            // Rust allows us to "return" a value from a loop by providing it to break. When
            // the loop exits, this will be the return value of the function too because the loop
            // is the last statement in this function.
            Ok((row, col)) => break (row, col),
            // Instead of defining methods to extract the value from InvalidMove, we can use
            // pattern matching to extract its value and print a helpful error message. The
            // `eprintln!` macro is exactly the same as `println!` except it prints to stderr
            // instead of stdout.
            Err(InvalidMove(invalid_str)) => eprintln!(
                // The `{}` is replaced with the next argument passed to eprintln. We can pass an
                // arbitrary amount of arguments and Rust can even tell us at compile time if there
                // is a mismatch between the number of {} and the number of additional arguments
                // passed.
                "Invalid move: '{}'. Please try again.",
                invalid_str,
            ),
        }
    }
}

// This function gets the row and column of the move the user entered. If the string doesn't
// represent a valid move, we return Result::Err to indicate failure.
// We pretty much always want to use &str instead of String in function arguments.
// For learn why, see:
// http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// NOTE: There are various ways that we could make this more "idiomatic" using some of the advanced
// features of Rust. However, notice though that we don't really lose anything or make anything
// worse for ourselves by keeping it simple. Rust lets you write nice code even if you haven't
// mastered all of its features just yet.
fn parse_move(input: &str) -> Result<(usize, usize), InvalidMove> {
    // The move will be in the format 1A, 2C, 3B, etc.
    // Let's start by rejecting any input that isn't of size 2
    if input.len() != 2 {
        // We use `return` to exit early from this function in case the size of the input is
        // incorrect.
        return Err(InvalidMove(input.to_string()));
    }

    // Let's start by getting the row number
    // Using match allows us to easily accept the cases we want to support and reject everything
    // else. If none of the cases match, an error will be returned.
    let row = match &input[0..1] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _ => return Err(InvalidMove(input.to_string())),
    };

    let col = match &input[1..2] {
        // Rust lets us match against multiple patterns using | to separate them. This
        // lets us accept either lowercase or uppercase versions of the letters.
        "A" | "a" => 0,
        "B" | "b" => 1,
        "C" | "c" => 2,

        // We didn't find a match so far, so the string must be invalid. We use the `Err`
        // variant of Result to express that.
        // We can convert a &str to a String using `to_string()`. InvalidMove expects a String,
        // so we need to do this for this code to work.
        invalid => return Err(InvalidMove(invalid.to_string())),
    };

    // The last line of the function is the return value, so we construct the tuple that we want
    // to return with the move that the user selected
    Ok((row, col))
}

// This function is something we've defined to make reading a line of input convenient. Rust gives
// us a lot of control over our program so we could do many fancy things like buffer the input as
// we read it or properly handle error conditions. However, since this is a simple application, we
// have chosen to just exit the program when an error occurs and do no extra buffering of the
// input. Since we're just reading a line at a time and we expect the lines to be short, this
// should not cause problems in the majority of cases. Rust gives us the power to make that choice
// explicitly and know that we are making it in the code.
fn read_line() -> String {
    // This creates a new growable/heap-allocated string. The `mut` after `let` declares that we
    // plan to modify the string. Saying this explicitly lets the compiler automatically check that
    // we don't modify any variables that we don't intend to. Many languages encourage you to use
    // `const` or `final` on pretty much everything until you don't need to. In Rust, that
    // behaviour is by default.
    let mut input = String::new();
    // Here, we read a line of input from the standard input stream stdin. `&mut input` passes a
    // mutable reference to the String in the input variable. This allows the function to modify
    // input without taking ownership of its value. That way we can return it from this function
    // afterwards.
    // expect() is a function that takes a Result value and exits the program with an error message
    // if the Result value is anything other than Ok(...). This in a way is "ignoring" any error
    // that can occur while reading input. However, instead of ignoring it implicitly, we explciitly
    // call out that we intend to just exit the program with an error if this operation fails. This
    // is one of the ways that Rust gives you control. Don't want to deal with a potential failure?
    // You don't have to! But it's really nice to know where the error came from if something ever
    // does go wrong and you want to figure out why.
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // An empty string will only be returned if we reach the end of input (otherwise we always
    // receive at least a newline character).
    if input.is_empty() {
        // We print a final newline because otherwise the cursor may still be at the end of one
        // of our `print!` calls earlier.
        println!();

        // process::exit(0) indicates that the program exited successfully. This will end the
        // program right here, and none of the rest of our code will run.
        process::exit(0);
    }

    // read_line leaves the trailing newline on the string, so we remove it using truncate. By
    // modifying the string in place, we avoid copying its contents after it was just allocated.
    let len_without_newline = input.trim_right().len();
    input.truncate(len_without_newline);

    // The last expression in a function is returned from that function. We want to return the
    // line that was read, so we put that variable on its own at the end of the function in order
    // to provide it as the result of this function.
    input
}

// This function is used to print out the board in a human readable way
fn print_tiles(tiles: &Tiles) {
    // The result of this function will be something like the following:
    //   A B C
    // 1 x ▢ ▢
    // 2 ▢ ▢ o
    // 3 ▢ ▢ ▢
    //
    // The boxes represent empty tiles, and x and o are placed wherever a tile is filled.

    // First we print the space before the column letters
    print!("  ");
    // Then we look from the numbers 0 to 2.
    // `a..b` creates a "range" of numbers from a to one less than b.
    // `tiles[0].len()` gets the number of columns (i.e. 2)
    // `as u8` converts the length from the type `usize` to the type `u8` so that it works in the
    // body of the loop
    for j in 0..tiles[0].len() as u8 {
        // `b'A'` produces the ASCII character code for the letter A (i.e. 65)
        // By adding j to it, we get 'A', then 'B', and then 'C'.
        // We don't just want to print the ASCII character code, so we convert that number into
        // a character using `as char`. That way Rust will print it correctly.
        print!(" {}", (b'A' + j) as char);
    }
    // This prints the final newline after the row of column letters
    println!();

    // Now we print each row preceeded by its row number
    // .iter().enumerate() goes through each row and provides a row number with each element using
    // a tuple.
    for (i, row) in tiles.iter().enumerate() {
        // We print the row number with a space in front of it
        print!(" {}", i + 1);
        // Now we go through each tile in the row and print it out
        for tile in row {
            // Here, we match on the value of the tile. We use `*` to "dereference" the tile and
            // match on its value of type Option<Piece>. This is just for convenience and is
            // actually something that future versions of Rust might not even require in order to
            // match on something as simple as this.
            print!(" {}", match *tile {
                // The string produced by this match will be printed in `print!`. This match works
                // because we return the same type, &str, in each branch. Rust still requires that
                // if a match statement produces a value, it produces a value of the same type in
                // every branch.
                // Notice that we don't need to create another match for the piece produced in
                // Some(...). Rust allows us to match arbitrarily nested structures with no
                // additional syntax.
                Some(Piece::X) => "x",
                Some(Piece::O) => "o",
                None => "\u{25A2}",
            });
        }
        // We finish each row by printing a final new line
        println!();
    }

    // Add an extra line at the end of the board to space it out from the prompts that follow
    println!();
}
