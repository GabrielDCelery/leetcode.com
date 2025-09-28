struct User<'a> {
    // active: bool,
    username: &'a str,
}

#[derive(Debug)]
struct Rectange {
    height: usize,
    width: usize,
}

impl Rectange {
    fn area(&self) -> usize {
        return self.height * self.width;
    }

    fn resize(&mut self, new_size: usize) {
        self.height = new_size;
        self.width = new_size;
    }
}

fn main() {
    let user = User {
        // active: true,
        username: &"Gabe",
    };

    let mut rect = Rectange {
        height: 5,
        width: 5,
    };

    rect.resize(10);

    let rect_area = rect.area();

    println!("{}", user.username);
    println!("rect is {rect:?} and it's size is {rect_area}")
}
