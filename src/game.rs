// This constant can be used to set the board size
// Since Rust's arrays are fat pointers, you won't see this constant referred to again after the
// we declare the type of Game. I mention this because if you were writing in a language like C,
// you would either need to pass the size to every function with the board or rely on this global
// constant. In Rust, that information is stored directly in the array so you always have the
// correct value.
const BOARD_SIZE: usize = 3;

// We want to use an enum for piece because we can either have one piece or the other on a tile,
// but never both at the same time
// `derive` automatically derives certain useful traits. These make this custom type that we've
// defined copyable, comparable for equality, and more without any additional work!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    // Access these variants using `Piece::X` or `Piece::O`
    X,
    O,
}

impl Piece {
    // This method is used to return the opposite piece and is used to quickly determine the next
    // piece after each move
    // By putting `self` as the first parameter, we are copying the piece that this method is
    // called on. This happens because this type derives `Copy` in its declaration. Without that,
    // using `self` alone would "move" the value into this function. Rust would ensure that no
    // other code could access it afterwards. Copy gives us complete control over which values we
    // want Rust to copy and which values we want Rust to move and only copy when we explicitly
    // ask for it.
    // For more information, see: https://doc.rust-lang.org/beta/std/marker/trait.Copy.html
    pub fn other(self) -> Piece {
        // The last expression in a function is returned from that function, so without writing
        // `return` anywhere, we can return the correct Piece from this function.
        // We could have also used multiple if statements, but this is a little simpler to read
        // once you understand the syntax.

        // Rust will tell us if Piece ever changes and this match doesn't cover every case
        match self {
            // match lets us conveniently express both cases without too much additional syntax
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

// By using an Option type, we can represent the possibility of having one of the valid piece
// types, or no piece at all. Notice that we chose not to just add an "Empty" piece type because
// this allows us to use Piece for other things like representing the choices for the current
// piece. The current piece can never be "empty", so it doesn't make sense to have an Empty variant
// in the Piece enum.
pub type Tile = Option<Piece>;
// We represent the tiles of the board using a 2D array
// Each element of the first array is a row of the board.
// tiles[1][2] accesses the second row and third column of the board.
pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// There are three possibilities for the winner at the end of the game. We represent them as an
// enum because only one of them can ever occur at a given time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}

// This type represents the possible errors that can occur when making a move
#[derive(Debug, Clone)]
pub enum MoveError {
    // Putting /// instead of // means that Rust's documentation tool will automatically pickup
    // that comment and use it when generating beautiful documentation for this module.

    /// The game was already over when a move was attempted
    GameAlreadyOver,

    // Fields allow us to provide more information about what happened

    /// The position provided was invalid
    InvalidPosition { row: usize, col: usize },

    /// The tile already contained another piece
    TileNotEmpty { other_piece: Piece, row: usize, col: usize },
}

#[derive(Debug, Clone)]
pub struct Game {
    tiles: Tiles,
    // There is always a current piece, so we don't need to wrap it in an Option type.
    current_piece: Piece,
    // There is only a winner at the end of the game, and once there is, it never changes. If we
    // wanted to, we could use the Rust type system to enforce this invariant and make sure the
    // program can't even be written in a way that would violate that. I decided to keep it simple
    // and not do that, but it's a great exercise to try out!
    // Hint: Replace the Winner enum declaration with a `struct Winner(...)` and make the type of
    // this field `Winner`. If you make that type so that the winner can only be set to something
    // other than None once, it will no longer be possible to write a program that violates the
    // invariant stated above.
    winner: Option<Winner>,
}

impl Game {
    // Using Self inside of an impl allows us to refer to its type (i.e. `Game`) without using the
    // type name explicitly. This is useful for renaming!
    pub fn new() -> Self {
        // Here we construct and return a new instance of Game
        Self {
            // Here, we take advantage of the Default trait to make it so that this code doesn't
            // have to know the type we defined for tiles in order to initialize it. Rust has
            // already defined the trait for arrays and the Option type, so we don't need to
            // implement it ourself!
            // More info: https://doc.rust-lang.org/std/default/trait.Default.html
            tiles: Default::default(),
            // We want to start with X
            current_piece: Piece::X,
            // There is no winner at the start of the game. We cleanly represent this with `None`.
            // Rust will warn us before our program even tries to run if we forget that this value
            // might be None.
            winner: None,
        }
    }

    // `&mut self` reflects that we plan to modify this struct in this method. Rust will ensure
    // that no other thread can access this object while we are modifying it. Thus eliminating any
    // possible data races.
    // Both row and col must be values from 0 to (BOARD_SIZE-1)
    // In the return type, () indicates the "unit type". That means that on success, this function
    // returns nothing.
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError> {
        if self.is_finished() {
            // Here, we use `return` to indicate that we want to leave this function early if this
            // case occurs. We could have written it without return by using `else` and indenting
            // the remaining function.
            return Err(MoveError::GameAlreadyOver);
        }
        // The usize type is "unsigned", meaning it is always positive. That means that this
        // potential error case is unrepresentable. We don't need to check for it if it can't
        // happen!
        // Notice that we use `.len()` instead of the BOARD_SIZE constant we defined because Rust
        // arrays provide their length.
        else if row >= self.tiles.len() || col >= self.tiles[0].len() {
            // Rust supports a "field shorthand" syntax which allows us to write {row, col} instead
            // of {row: row, col: col}
            return Err(MoveError::InvalidPosition {row, col});
        }
        // Rust allows us to conditionally test a pattern match without using `match` directly.
        // This makes it super convenient to check if the tile is empty or not
        else if let Some(other_piece) = self.tiles[row][col] {
            // The pattern match allows us to check if there is a potential value and extract it
            // in one quick sweep. This makes writing the next line very easy!
            return Err(MoveError::TileNotEmpty {other_piece, row, col});
        }

        // Now that we've done all of the error checking, we can proceed with making the move and
        // modifying the tiles and current piece

        // Here we store the current piece at the correct location in self.tiles
        self.tiles[row][col] = Some(self.current_piece);

        // Notice that since we don't publically expose a way to set the current piece, we can
        // always be sure that it will be updated correctly and according the rules we expect.
        self.current_piece = self.current_piece.other();

        // After making a move, it may be that someone won the game. We'll use another method for
        // that since this one is getting quite long.
        self.update_winner(row, col);

        // Now that everything is complete, we can go ahead and return our "nothing" value `()`
        // called "unit" to indicate that this operation was a success. We construct a Result type
        // using its `Ok` variant as the constructor.
        Ok(())
    }

    // We use a private method to separate code that shouldn't be accessed publically
    fn update_winner(&mut self, row: usize, col: usize) {
        // To find a potential winner, we only need to check the row, column and (maybe) diagonal
        // that the last move was made in.

        // Let's make some convenience variables for the number of rows and columns
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        // We can extract the row pretty easily because of how we stored tiles
        let tiles_row = self.tiles[row];

        // To get the correct column, we could do something very fancy that would work for every
        // size of board, but in this case we'll just do the simplest thing and get the column
        // directly using indexing.
        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        // This relies on the assumption that the board has size 3, so let's assert that so that if
        // someone ever changes this code there are no weird bugs
        // This will produce an error at runtime if this assumption is broken.
        assert!(rows == 3 && cols == 3,
            "This code was written with the assumption that there are three rows and columns");

        // There are two diagonals on the board. Their positions are as follows:
        // 1. (0, 0), (1, 1), (2, 2)
        // 2. (0, 2), (1, 1), (2, 0)
        // Due to the possibility of being on (1, 1), we might be on both diagonals. We will check
        // both diagonals separately.
        // Notice that on a 3x3 board, if row == col, we are on the first diagonal
        // and if (rows - row - 1) == col, we are on the second diagonal.
        // If we are on neither diagonal, we can just use an array of None's so that it definitely
        // won't find a match.

        // Here, we see that if statements can be used as expressions just like match statements.
        // That means that we can assign this variable to the result of the if statement.
        let tiles_diagonal_1 = if row == col {
            // Once again, we'll do the simplest thing and just use an array.

            // Diagonal 1
            [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]]
        }
        else {
            // This will never produce a winner, so it is suitable to use for the case where the
            // last move isn't on diagonal 1 anyway.
            [None, None, None]
        };

        let tiles_diagonal_2 = if (rows - row - 1) == col {
            // Diagonal 2
            [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]]
        }
        else {
            // Our last move isn't on diagonal 2.
            [None, None, None]
        };

        // Now that we have the row, column and diagonal of the last move, let's check if we have
        // a winner. To do that, we'll use a check_winner function that either returns a new
        // Winner or None. This is useful because we can chain together the methods of the Option
        // type to produce a result. This is an alternative to multiple if statements that works
        // just as well.
        fn check_winner(row: &[Tile]) -> Option<Winner> {
            // This is an "inner function". It is only visible to this update_winner method. We
            // could have defined this as a method or defined it as a function separate from this
            // impl too.
            // The type `&[Tile]` is known as a slice. This is how we pass an array by reference.
            // We don't have to pass the size with the array because the array pointer also stores
            // its length.
            // By returning an option type, we signal that this function may return some value or
            // no value (i.e. None).

            // Here, we once again do the simplest thing possible and just use indexes to check
            // if the entire row is the same. We could potentially do something more general using
            // iterators, but why do that if this simpler way works?
            if row[0] == row[1] && row[1] == row[2] {
                // We use a match to retrieve the correct winner based on the piece that has filled
                // this row.
                match row[0] {
                    Some(Piece::X) => Some(Winner::X),
                    Some(Piece::O) => Some(Winner::O),
                    None => None,
                }
            }
            else {
                // All the tiles are not the same, there is no winner yet, so let's signal that
                // with None
                None
            }
        }
        // Now that we can determine if there is a winner or not, we can use the option type's
        // methods to chain together the results. See the Option type documentation for more info:
        // https://doc.rust-lang.org/std/option/enum.Option.html
        self.winner = self.winner
            // The || syntax is actually defining a special function called a "closure" (or
            // "lambda" in some languages). That allows us to delay calling the check_winner
            // function until we actually need it.
            // By using or_else over and over again, we never overwrite a previously found winner
            // and the code is only run in case a previous winner was *not* found.
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        // The final case is when the board has filled up. Here, for the first time, we'll be a
        // bit fancy and use the Iterator trait. For more info, see the book:
        // https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html
        // This is also the first time we see a multiline closure using curly braces. Just like
        // any other function, this returns the final (and only) value between the curly braces.
        self.winner = self.winner.or_else(|| {
            // You can read this code as follows:
            // if in each of the rows, all tiles have *something* in them,
            //     return that the winner is a tie.
            // otherwise, return that there is no winner yet
            // For more information on `all`, see:
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
            if self.tiles.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
                Some(Winner::Tie)
            }
            else {
                None
            }
        });
    }

    // We can define helpful accessor functions for common questions that will be asked about this
    // type. This makes it so that people using this type won't have to rely on how the type is
    // represented.
    // `&self` tells the Rust compiler that we won't be modifying this type
    pub fn is_finished(&self) -> bool {
        // The last line of a function is its return value, so we don't need to write return for
        // simple one line functions.

        // The game is finished if there is a winner.
        // Since we used an Option type, we can use the convenient method it provides for checking
        // if it is Some or None instead of having to match on the type itself.
        self.winner.is_some()
    }

    // This method returns the winner of the game (if any). Since Winner derives the Copy trait, we
    // can return it directly from this function without moving its value. Rust will copy the value
    // (including the Option type that wraps it). For small types, this can make writing the code
    // much easier without introducing any additional performance penalty.
    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    // This method is similar to the winner method above. It returns a copy of the current piece.
    // Just like Winner, Piece also implements the Copy trait.
    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    // This function gives public, read-only access to the tiles of the board. Rust will enforce
    // at compile-time that no outside entity is able to modify the tiles from this reference.
    pub fn tiles(&self) -> &Tiles {
        // The `&` at the front creates a read-only reference. `self.tiles` accesses the tiles
        // field of this struct.
        &self.tiles
    }
}
