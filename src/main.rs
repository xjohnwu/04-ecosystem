use ecosystem::MyError;

fn main() -> Result<(), MyError> {
    println!("Size of my error is {}", std::mem::size_of::<MyError>());

    // let filename = "non-existent-file.txt";
    // let _fd =
    //     std::fs::File::open(filename).with_context(|| format!("Cannot find file: {}", filename))?;
    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
