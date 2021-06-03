# We check if `cargo` is installed
if ! command -v cargo &> /dev/null ; then
    echo -e "[\e[31mError\e[0m]: Cargo is not installed"
    echo -e "[\e[31mError\e[0m]: Please install it first"
    echo -e "[\e[31mError\e[0m]: Aborting..."
    exit
fi

# We check if `rustc` is installed
if ! command -v rustc &> /dev/null ; then
    echo -e "[\e[31mError\e[0m]: Rust is not installed"
    echo -e "[\e[31mError\e[0m]: Please install it first"
    echo -e "[\e[31mError\e[0m]: Aborting..."
    exit
fi

# We build our project
echo -e "[\e[32mInfo\e[0m]: Building..."
cargo build -q

# We check if we failed at build
if [ ! -f -x ./target/debug/rusty_brain ] ; then
    echo -e "[\e[31mError\e[0m]: Build failed"
    echo -e "[\e[31mError\e[0m]: Aborting..."
    exit
fi

# We run our executable
echo -e "[\e32mInfo\e[0m]: Build successful"
echo -e "[\e32mInfo\e[0m]: Running..."
clear
cargo run -q