#[derive(Debug)]
struct Vector {
    id: u32,        // Unique identifier for each vector
    data: Vec<f32>, // The vector components
}

impl Vector {
    fn new(id: u32, data: Vec<f32>) -> Self {
        Vector { id, data }
    }
}

struct Database {
    vectors: Vec<Vector>,
}

impl Database {
    fn new() -> Self {
        Database {
            vectors: Vec::new(),
        }
    }

    fn insert(&mut self, vector: Vector) {
        self.vectors.push(vector);
    }

    fn update(&mut self, id: u32, data: Vec<f32>) {
        for vector in self.vectors.iter_mut() {
            if vector.id == id {
                vector.data = data;
                break;
            }
        }
    }

    fn delete(&mut self, id: u32) {
        self.vectors.retain(|vector| vector.id != id);
    }

    fn get(&self, id: u32) -> Option<&Vector> {
        self.vectors.iter().find(|vector| vector.id == id)
    }
}

fn main() {
    // Insert vectors
    let mut db = Database::new();
    db.insert(Vector::new(1, vec![1.0, 2.0, 3.0]));
    db.insert(Vector::new(2, vec![4.0, 5.0, 6.0]));
    db.insert(Vector::new(3, vec![7.0, 8.0, 9.0]));

    let vector = db.get(2);
    match vector {
        Some(vector) => {
            println!("Vector found: {:?}", vector);
        }
        None => {
            println!("Vector not found");
        }
    }
}
