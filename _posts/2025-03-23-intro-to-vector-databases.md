# What is a vector database?

"A vector database is a database that can store vectors (fixed-length lists of numbers)" [[1]](https://en.wikipedia.org/wiki/Vector_database). Each item in a vector corresponds to a feature of an object of interest. For example, we can vectorize words by having a list of 26 numbers and counting the number of occurrences of each letter in the world. There are many ways to vectorize different kinds of data, and some are more useful than others, depending on the task at hand. Vector databases are now often used for natural language processing applications. For example, they can be used with large language models (LLMs) to give them more context given a user's query/prompt. The user's query is "embedded", that is, converted into a vector and compared to other vectors in the database. The most similar vectors are returned and the information they represent added to the user's query to the LLM. This allows LLMs to access information that is not in their training data or information they can't access using other tools (web search, etc).

To implement a vector database, we need a way to store lists of numbers. Given a query vector, we'll want to find vectors similar to it and return the most similar ones. To find the most similar vector, we can compute the distance between vectors. Vectors with a small distance between them are more similar, so all we'll need to do is return the closest vectors. 


Let's set the stage:

```rust
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
```

We define a `struct` for a Vector. We'll hold a vector (rust vector) of 32-bit floating point numbers in it as well as an identifier to query the database by id. Then our database is simply a list of our new vectors.

To compute the distance between two vectors, we'll just start with the Euclidean distance:

```rust
fn euclidean_distance(v1: &Vector, v2: &Vector) -> f32 {
    let mut sum = 0.0;
    for (a, b) in v1.data.iter().zip(v2.data.iter()) {
        sum += (a - b).powi(2);
    }
    sum.sqrt()
}
```

To implement operations such as querying vectors by id, update, delete:

```rust
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
```

Now we have most of our building blocks. We're just missing the ability to actually get a queried vector's nearest neighbour:

```rust
impl Database {
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
```

To check our implementation, we can run this:

```rust
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
```

which prints (after warnings etc.):

```
Nearest neighbour: Vector { id: 2, data: [4.0, 5.0, 6.0] }
```

Now, all this is not really impressive at all. Our operations are inefficient: our search algorithm and euclidean distance function have linear time complexity (for the search algorithm `O(n)` if there are `n` vectors in our database and for the euclidean distance `O(k)` if the dimensionality of our vectors is `k`.)
And because we're holding everything in memory, we're also taking `O(n)` space.
Imagine having to search thousands of times, and every single time we'd have to go through these operations. 
