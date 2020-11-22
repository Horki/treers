# treers
## Sedgewick's Tree Maps, simple implementations

![Travis](https://img.shields.io/travis/com/Horki/treers)
![Status](https://img.shields.io/badge/status-active-success.svg)
![Version](https://img.shields.io/crates/v/treers)
[![GitHub Issues](https://img.shields.io/github/issues/Horki/treers.svg)](https://github.com/Horki/treers/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/Horki/treers.svg)](https://github.com/Horki/treers/pulls)
![Downloads](https://img.shields.io/crates/d/treers)
![License](https://img.shields.io/crates/l/treers)
![GitHub last commit](https://img.shields.io/github/last-commit/Horki/treers?color=red&style=plastic)
![Twitter](https://img.shields.io/twitter/follow/ivanhorvat82?label=Follow)

## About

This is a hobby project, simple rewrite of Sedgewick's tree structures in Rust.

## Contribute
Please contribute, feel free to [write an issue](https://github.com/Horki/treers/issues), there are still plenty things to improve (such as improvement of docs).

## Tree Maps

### ~~Interfaces~~ Traits

* SedgewickMap

| Name               | Description |
|-----------------------------|:------------------------:|
| new | New Instance of Tree Map |
| size | Count of items in map |
| get | Fetch an value in map by key |
| put | Insert by key-value |
| height | Tree Height |
| is_empty | Checks if map is empty  |
| contains | Returns `true` if item exists |
| min | Retrieve a minimum key in map |
| max | Retrieve a maximum key in map |
| delete | TODO |

* TreeTraversal

| Name               | Description |
|-----------------------------|:------------------------:|
| pre_order | [Pre Order Traversal](https://en.wikipedia.org/wiki/Tree_traversal#Pre-order_(NLR)); [DFS](https://en.wikipedia.org/wiki/Depth-first_search) |
| in_order | [In Order Traversal](https://en.wikipedia.org/wiki/Tree_traversal#In-order); [DFS](https://en.wikipedia.org/wiki/Depth-first_search) |
| post_order | [Post Order Traversal](https://en.wikipedia.org/wiki/Tree_traversal#Post-order); [DFS](https://en.wikipedia.org/wiki/Depth-first_search) |
| level_order | [Level Order Traversal](https://en.wikipedia.org/wiki/Tree_traversal#Breadth-first_search_/_level_order); [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) |

### BST - Binary Search Tree

* Really slow (check benchmarks)
* Has a Tree Traversal implementation

| Algorithm | Average | Worst Case |
|-----------|---------|:---------:|
| Space | O(n) | O(n) |
| Search | O(log n) | O(n) |
| Insert | O(log n) | O(n) |

### Red-Black Tree

* *Only* 3x slower (check benchmarks) times that [Standard](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) Balanced Tree Implementation
* Has a Tree Traversal implementation
* Basic Rules
- Every node can be only red or black
- Root node has to be black
- Red nodes lean left
- Every path from the root to a null link has the same number of black links
- Popular usage in [CFS: Completely Fair Scheduler](https://en.wikipedia.org/wiki/Completely_Fair_Scheduler)

| Algorithm | Average | Worst Case |
|-----------|---------|:---------:|
| Space | O(n) | O(n) |
| Search | O(log n) | O(log n) |
| Insert | O(log n) | O(log n) |

### BTree - Balanced Tree

* Really slow (check benchmarks)
* Doesn't have a Tree Traversal implementation
* Popular usage in Databases and File Systems
* NOTE: I have fixed a loitering (memory) bug in official [algs4](https://github.com/kevin-wayne/algs4/pull/93)

| Algorithm | Average | Worst Case |
|-----------|---------|:---------:|
| Space | O(n) | O(n) |
| Search | O(log n) | O(log n) |
| Insert | O(log n) | O(log n) |

## Documentation

https://docs.rs/treers

## TODO

- [ ] More work on documentation and README
- [ ] BTree, use stack memory for entries
- [ ] Replace tree traversals with iterators
- [ ] Implement remaining methods for trees
- [ ] Make Red-Black Tree blazingly fast

## Resources

* [Algorithms, 4th Edition](https://algs4.cs.princeton.edu/home/)
* [Coursera - Algorithms, Part I](https://www.coursera.org/learn/algorithms-part1/)
* [Coursera - Algorithms, Part II](https://www.coursera.org/learn/algorithms-part2/) 
* [algs4, GitHub](https://github.com/kevin-wayne/algs4/)

## Authors

* **Ivan Zvonimir Horvat** [GitHub Profile](https://github.com/Horki/)

## License

Licensed under the [MIT License](LICENSE).
