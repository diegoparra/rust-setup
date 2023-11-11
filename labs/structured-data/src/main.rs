fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        uri: String,
        active: bool,
    }

    let diego = User {
        name: "diego".to_string(),
        email: "diego@example.com".to_string(),
        uri: "https://diego.com".to_string(),
        active: true,
    };

    println!("User: {:?}", diego);

    impl User {
        fn new(name: String, email: String, uri: String) -> Self {
            Self {
                name: name,
                email: email,
                uri: uri,
                active: true,
            }
        }

        fn deactivate(&mut self) {
            self.active = false
        }
    }

    let name = String::from("carla");
    let email = String::from("carla@example.com");
    let uri = String::from("https://carla.example.com");

    let mut carla = User::new(name, email, uri);

    println!("User name: {}, User status: {}", carla.name, carla.active);
    carla.deactivate();
    println!("User name: {}, User status: {}", carla.name, carla.active);
}
