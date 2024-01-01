#[derive(Debug)]


// MAKING A STRUCT
struct Person {
    first_name: String,
    last_name: String,
    // OPTION SAYS WE MIGHT GET A VALUE FOR AGE
    age: Option<u8>,
}

// STRUCT WITH CONSTRUCTOR METHOD
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}
// CONSTRUCTOR METHOD FOR STRUCT
impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

// STRUCT AS A TUPLE
struct Point(i32, i32, i32);

fn main() {
    // PRINTING A STRUCT THAT IS CREATED ON THE FLY
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        // TO USE AN OPTION WE HAVE TO USE Some()
        age: Some(30),

    });

    // CREATING AN INSTANCE OF A STRUCT 
    let alfredo: Person = Person {
    first_name: "Alfredo".to_string(),
    last_name: "Garcia".to_string(),
    age: None,
    };

    println!("{:?}", alfredo);
    println!("{}", alfredo.first_name);
    println!("{}", alfredo.last_name);
    println!("{:?}", alfredo.age);


    let mut a_user: User = User::new(
        String::from("Klee"),
        String::from("klee@klee.com"),
        String::from("www.klee.com"),
        );
    

        println!("{}", a_user.username);
        println!("{}", a_user.email);
        println!("{}", a_user.uri);
        println!("active status: {}", a_user.active);

        a_user.deactivate();
        println!("active status: {}", a_user.active);


        let username: String = String::from("Klee2");
        let email: String = String::from("emailXXXXXXXXX");
        let uri: String = String::from("uriXXXXXXXXX");
        let active: bool = true;

        let user_2: User = User {
            username, email, uri, active,
        };

        println!("{}", user_2.username);
        println!("{}", user_2.email);
        println!("{}", user_2.uri);
        println!("active status: {}", user_2.active);


        // STRUCT TUPLES ACCESS DATA VIA INDEX
        let my_point: Point = Point(10, 20, 30);
        println!("points: {}, {}, {}", my_point.0, my_point.1, my_point.2);

}
