#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        match client::server_start() {
            Ok(()) => (),
            Err(err) => println!("{:?}", err),
        }
    }
}
