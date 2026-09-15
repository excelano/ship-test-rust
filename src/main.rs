//! A throwaway command that exists to be released.

fn main() {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() == Some("--version") {
        println!("ship-test-rust {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    println!("hello from ship-test-rust");
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_set() {
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }
}
