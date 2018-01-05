# Merkle

A [merkle tree](http://en.wikipedia.org/wiki/Merkle_tree) implementation in Rust. This library is still fairly early.


## Getting Started

Get it using Cargo!

```toml
# Cargo.toml
[dependencies.merkle]
git = "https://github.com/thehydroimpulse/merkle.git"
```

Now let's link the crate:

```rust
// src/lib.rs
extern crate merkle;

use merkle::{MerkleTree, Node};
```

## Introduction

A merkle tree is a unique type a tree that's used in [Bitcoin](https://en.bitcoin.it/wiki/Protocol_specification#Merkle_Trees), [Cassandra](http://distributeddatastore.blogspot.ca/2013/07/cassandra-using-merkle-trees-to-detect.html), Apache Wave, ZFS, and [Adobe Brackets](http://brackets.io/) just to name a few.

Merkle trees are extremely useful to validate for corruptness, differences, etc... Bitcoin uses it to validate transactions, Cassandra uses it to determine if replicas are corrupt, and Brackets uses it in their virtual DOM system to try and find the minimal possible updates the DOM needs to achieve a final result. Comparing two versions of a virtual DOM tree (and their merkle trees) can easily lead to a list of the most atomic changes and how to go about it most efficiently.

## Creating a Node

Creating a node is extremely easy. In the current variation, any node can contain data, not just the leaf nodes. In this case, we start with the root node and start pushing children onto it.

```rust
let mut root = Node::new("foobar");
root.push_child(Node::new("another_child"));
```

## Creating a Merkle Tree

Now we can create a merkle tree given a root node:

```rust
let mut merkle = MerkleTree::new(root);
```

## Hashing The Tree

The final step is to hash the tree to get a merkle root or root hash.

```rust
merkle.hash();
merkle.root() // &str - containing the root hash
```

# License

The MIT License (MIT)

Copyright (c) 2014 Daniel Fagnan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
