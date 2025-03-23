# Vector database in Rust

This is a blog about writing a vector database in Rust. The blog is available at https://milongo.github.io/vector_db/.

## Introduction to vector databases

Vector databases are a new breed of data storage solutions designed to efficiently manage high-dimensional vector data, which represent complex entities like text, images, and audio.

At their core, vectors are simply arrays of numbers that capture the essential features or “embeddings” of an item. For example, a sentence can be transformed into a numerical vector that encodes its semantic meaning. Unlike traditional databases that rely on exact matching using structured data, vector databases use similarity searches to find related items. This makes them ideal for powering recommendation systems, image search, and natural language processing applications.

One of the key advantages of vector databases is their ability to handle “unstructured” data. By using specialized indexing algorithms—such as Approximate Nearest Neighbor (ANN) techniques—they can quickly locate similar vectors in large datasets. This means that even if you query with an image or a piece of text, the database can return results that are contextually relevant, rather than strictly identical.

In this blog post, I'll be implementing a vector database, starting from a simple in-memory database with nearest neighbour search, and progressively improving it. I'll be writing this fun side project in Rust since this is a language I'm trying to learn!

## Why Rust?

Rust has rapidly gained popularity as a systems programming language that emphasizes both performance and safety. Unlike Python, which abstracts away many of the low-level details, Rust exposes you to the nuts and bolts of how computers operate. Its unique ownership model and strict compile-time checks force you to think carefully about memory management and concurrency. This hands-on approach not only minimizes common programming errors but also deepens your understanding of the inner workings of your code—something that high-level languages like Python tend to hide behind layers of abstraction.


## Blog Posts
<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ site.baseurl }}{{ post.url }}">{{ post.title }}</a> - <small>{{ post.date | date: "%B %d, %Y" }}</small>
    </li>
  {% endfor %}
</ul>
