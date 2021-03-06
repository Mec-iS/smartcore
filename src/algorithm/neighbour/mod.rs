//! # Nearest Neighbors Search Algorithms and Data Structures
//!
//! Nearest neighbor search is a basic computational tool that is particularly relevant to machine learning,
//! where it is often believed that highdimensional datasets have low-dimensional intrinsic structure.
//! The basic nearest neighbor problem is formalized as follows: given a set \\( S \\) of \\( n \\) points in some metric space \\( (X, d) \\),
//!  the problem is to preprocess \\( S \\) so that given a query point \\( p \in X \\), one can efficiently find a point \\( q \in S \\)
//!  which minimizes \\( d(p, q) \\).
//!
//! [The most straightforward nearest neighbor search algorithm](linear_search/index.html) finds k nearest points using the brute-force approach where distances between all
//! pairs of points in the dataset are calculated. This approach scales as \\( O(nd^2) \\) where \\( n = \lvert S \rvert \\), is number of samples and \\( d \\) is number
//! of dimentions in metric space. As the number of samples  grows, the brute-force approach quickly becomes infeasible.
//!
//! [Cover Tree](cover_tree/index.html) is data structure that partitions metric spaces to speed up nearest neighbor search. Cover tree requires \\( O(n) \\) space and
//! have nice theoretical properties:
//!
//! * construction time: \\( O(c^6n \log n) \\),
//! * insertion time \\( O(c^6 \log n) \\),
//! * removal time: \\( O(c^6 \log n) \\),
//! * query time: \\( O(c^{12} \log n) \\),
//!
//! Where \\( c \\) is a constant.
//!
//! ## References:
//! * ["The Art of Computer Programming" Knuth, D, Vol. 3, 2nd ed, Sorting and Searching, 1998](https://www-cs-faculty.stanford.edu/~knuth/taocp.html)
//! * ["Cover Trees for Nearest Neighbor" Beygelzimer et al., Proceedings of the 23rd international conference on Machine learning, ICML'06 (2006)](https://hunch.net/~jl/projects/cover_tree/cover_tree.html)
//! * ["Faster cover trees." Izbicki et al., Proceedings of the 32nd International Conference on Machine Learning, ICML'15 (2015)](http://www.cs.ucr.edu/~cshelton/papers/index.cgi%3FIzbShe15)
//! * ["The Elements of Statistical Learning: Data Mining, Inference, and Prediction" Trevor et al., 2nd edition, chapter 13](https://web.stanford.edu/~hastie/ElemStatLearn/)
//!
//! <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
//! <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

pub(crate) mod bbd_tree;
/// tree data structure for fast nearest neighbor search
pub mod cover_tree;
/// very simple algorithm that sequentially checks each element of the list until a match is found or the whole list has been searched.
pub mod linear_search;
