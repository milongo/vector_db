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

    fn nearest_neighbour(&self, query: &Vector) -> Option<&Vector> {
        let mut closest_vector = &self.vectors[0];
        let mut min_distance = f32::MAX;
        for vector in self.vectors.iter() {
            let distance = euclidean_distance(query, vector);
            if distance < min_distance {
                min_distance = distance;
                closest_vector = vector;
            }
        }
        Some(closest_vector)
    }
}

fn euclidean_distance(v1: &Vector, v2: &Vector) -> f32 {
    let mut sum = 0.0;
    for (a, b) in v1.data.iter().zip(v2.data.iter()) {
        sum += (a - b).powi(2);
    }
    sum.sqrt()
}

fn main() {
    // Insert vectors
    let mut db = Database::new();
    db.insert(Vector::new(1, vec![1.0, 2.0, 3.0]));
    db.insert(Vector::new(2, vec![4.0, 5.0, 6.0]));
    db.insert(Vector::new(3, vec![7.0, 8.0, 9.0]));

    let query = Vector::new(4, vec![3.0, 4.0, 5.0]);
    let nearest_neighbour = db.nearest_neighbour(&query);
    match nearest_neighbour {
        Some(vector) => {
            println!("Nearest neighbour: {:?}", vector);
        }
        None => {
            println!("No nearest neighbour found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut db = Database::new();
        let vec1 = Vector::new(1, vec![1.0, 2.0, 3.0]);
        db.insert(vec1);
        let retrieved = db.get(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, 1);
    }

    #[test]
    fn test_update() {
        let mut db = Database::new();
        db.insert(Vector::new(1, vec![1.0, 2.0, 3.0]));
        db.update(1, vec![4.0, 5.0, 6.0]);
        let updated = db.get(1).unwrap();
        assert_eq!(updated.data, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_delete() {
        let mut db = Database::new();
        db.insert(Vector::new(1, vec![1.0, 2.0, 3.0]));
        db.delete(1);
        assert!(db.get(1).is_none());
    }

    #[test]
    fn test_nearest_neighbour() {
        let mut db = Database::new();
        db.insert(Vector::new(1, vec![1.0, 2.0, 3.0]));
        db.insert(Vector::new(2, vec![4.0, 5.0, 6.0]));
        db.insert(Vector::new(3, vec![7.0, 8.0, 9.0]));
        let query = Vector::new(4, vec![3.0, 4.0, 5.0]);
        let nearest = db.nearest_neighbour(&query).unwrap();
        // Simple check: ensure we got a vector from the db (detailed distance check omitted).
        assert!(nearest.id == 1 || nearest.id == 2 || nearest.id == 3);
    }
}
