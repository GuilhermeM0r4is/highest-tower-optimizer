use std::io;    // this is the library used to input/output tasks

/* block structure that stores information and assures conditions
   uses flags later functions usage */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    length: u32,
    width: u32,
    height: u32
}

impl Block {

    /// Creates a new block, assuring L > W, this way, only 3 rotations exist.
    const fn new(l: u32, w: u32, h: u32) -> Self {

        // if w < l, then we change the tuple values of width and length
        let (width, length) = if w < l {
            (w, l)
        }
        else {
            (l, w)
        };
        Self { length, width, height: h }
    }

    /// Creates a vector of orientations that can be made.
    const fn orientations(self) -> [Self; 3] {
        [
            Block::new(self.length, self.width, self.height),
            Block::new(self.length, self.height, self.width),
            Block::new(self.width, self.height, self.length),
        ]
    }
}

struct SegmentTree {
    tree: Vec<(u32, usize)>,     // the tree itself
}

impl SegmentTree {

    /// Creates a new tree with all values set to 0
    fn new(n: usize) -> Self {
        SegmentTree {
            tree: vec![(0, usize::MAX); (n + 1) * 4]
        }
    }

    /// Updates a new width of values with a new value
    ///
    /// [node]: A node to follow with the value of max height;
    /// [st]: Start value of the branch;
    /// [end]: End value of the branch;
    /// [idx]: Width value to update;
    /// [val]: Value to update the respective branch.
    ///
    fn update(&mut self, node: usize, st: usize,
                end: usize, idx: usize, val: (u32, usize)) {

        // if both values are equal, we arrived at the idx we wanted to change
        if st == end {
            self.tree[node] = self.tree[node].max(val);
            return;
        }

        // making the logic for where to go next
        let mid: usize = (st + end) / 2;

        // we go to left branch with new interval
        if idx <= mid {
            self.update(node * 2, st, mid, idx, val);
        }
        // we check the right branch instead
        else {
           self.update(node * 2 + 1, mid + 1, end, idx, val);
        }

        // the parent gets the tuple of the biggest height between the childs
        self.tree[node] = self.tree[node * 2]
                            .max(self.tree[node * 2 + 1]);
    }

    /// Searches for the biggest value inside a [Left, Right] interval
    ///
    /// [node]: A node to follow through all the process;
    /// [st]: Start value of the branch;
    /// [end]: End value of the branch;
    /// [l]: Left branch to start searching for;
    /// [r]: Right branch to continue the search.
    ///
    fn query(&self, node: usize, st: usize,
                end: usize, l: usize, r: usize) -> (u32, usize) {

        // out of the given interval
        if r < st || end < l {
            return (0, usize::MAX);
        }
        // exactly inside the given range, making it exact
        else if l <= st && end <= r {
            return self.tree[node];
        }
        // its only a partial overlap
        else {
            let mid: usize = (st + end) / 2;

            let q1 = self.query(2 * node, st, mid, l, r);
            let q2 = self.query(2 * node + 1, mid + 1, end, l, r);

            // returns the maximum value between the 2
            q1.max(q2)
        }
    }
}

fn main() {

    // vector where all different blocks will be stored
    let mut block_vec: Vec<Block> = Vec::new();
    let mut num_blocks = String::new();

    /* reads the line from input given from the user
       handling any failure during the process */
    if io::stdin()
        .read_line(&mut num_blocks)
        .is_err() {
            return;
        }

    // turns the String into a usize value to use for the for loop
    let num_blocks: u32 = match num_blocks.trim().parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    fill_vec_from_input(&mut block_vec, num_blocks);
    order_and_clean_vec(&mut block_vec);
    search_for_highest(&mut block_vec);
}



/// Fills a given vector from a num ammount of inputs containing 3 numbers each input.
///
/// [vec]: A vector to fill with Block from input;
/// [num]: Ammount of loops to do in usize.
///
fn fill_vec_from_input(vec: &mut Vec<Block>, num: u32) {

    // the for loop will read the ammount of inputs until we reach the given number
    for _ in 0u32..num {

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        /* we create a small values variable which stores all given line input
           into a u32 size group of numbers, splitting on the whitespaces */
        let mut values = input
                        .split_whitespace()
                        .map(|s| s.parse::<u32>()
                            .expect("Failed to parse into u32"));

        /* Some() is a built in function in Rust that allows to be sure there is a next
           comming value, if not returns NONE using an iterator Option<u32>, assures there
           are exaclty 3 values given to make a block */
        if let (Some(l), Some(w), Some(h)) = (values.next(), values.next(), values.next()) {

            // extends the vec with the new made 3 sized array with all orientations
            vec.extend_from_slice(
                &Block::orientations(
                    Block::new(l, w, h)
                )
            );
        }
    }
}

/// Gathers a given vector, sorts by decrescing L compriments, then takes the tied
/// values and sorts them by decrescing W, then it removes all duplicates.
///
/// [vec]: A given vector full of Block to sort and remove dupes.
///
fn order_and_clean_vec(vec: &mut Vec<Block>) {

    /* sorts by decrescing L, that's why b comes first, and only then a
       then for tied values .then(), we sort by W */
    vec.sort_by(|a, b|
                b.length.cmp(&a.length)
                .then(b.width.cmp(&a.width))
    );
    vec.dedup();    // clears duplicates
}

/// Uses a given vector together with a Segment Tree vector to build
/// the highest possible tower of blocks.
///
/// [vec]: A given vector full of Block to build the highest tower possible.
///
fn search_for_highest(vec: &mut Vec<Block>) {

    // assures memory optimization for trees, storing only indexes
    let mut widths: Vec<u32> =
            vec.iter()
                .map(|b| b.width)
                .collect();

    widths.sort_unstable();
    widths.dedup();

    let n_widths = widths.len();

    let mut tree: SegmentTree = SegmentTree::new(n_widths);

    // a vector containing all parents path so we can search it
    let mut parent: Vec<usize> = vec![usize::MAX; vec.len()];
    let mut max_overall_h: u32 = 0;
    let mut best_end_block: usize = 0;

    search_inside_tree(vec, widths, n_widths, &mut tree, &mut parent,
                        &mut max_overall_h, &mut best_end_block);
}

/// Auxiliar function that loops in O(NlogN) inside a SegmentTree.
///
/// [c_width]: New given index of the compressed widths vector;
/// [n_width]: Amount of widths inside the tree to search;
/// [tree]: Segment Tree to look inside;
/// [parent]: Parent vector containing all the path for the children;
/// [max_ovrll_h]: Biggest height value found until now;
/// [best_end_b]: Best block for the ending (top) of tower.
///
fn search_inside_tree(vec: &mut Vec<Block>, widths: Vec<u32>, n_widths: usize,
                        tree: &mut SegmentTree, parent: &mut Vec<usize>,
                        max_ovrll_h: &mut u32, best_end_b: &mut usize) {

    // search O(N), for each element as top of the tower
    for (i, block) in vec.iter().enumerate() {

        let c_width: usize =
            widths.binary_search(&block.width).unwrap();

        // updates the best height with the tree query info
        let (best_prev_h, best_idx): (u32, usize) =
            tree.query(1, 0, n_widths - 1,
                        c_width + 1, n_widths - 1);

        // if not usize::MAX then we found a valid block
        if best_idx != usize::MAX {
            parent[i] = best_idx;
        }

        // the block size of this tower is the sum of this + best
        let t_height: u32 = block.height + best_prev_h;

        if t_height > *max_ovrll_h {
            *max_ovrll_h = t_height;
            *best_end_b = i;
        }

        // update to know the biggest tower value
        tree.update(1, 0, n_widths - 1, c_width, (t_height, i));
    }
    reconstruct_tower(vec, &parent, *best_end_b, *max_ovrll_h);
}

/// Gets the best ending block info, the highest height we found up until now
/// and the vector full of blocks to check with parent to reconstruct the tower.
///
/// [vec]: Vector containing all Block;
/// [p]: The parent vector with the path built into;
/// [best_end_b]: The best ending block that achieves highest height;
/// [max_h]: Highest tower height found in construction.
///
fn reconstruct_tower(vec: &Vec<Block>, parent: &Vec<usize>,
                        best_end_b: usize, max_h: u32) {

    // sets the variables to start building
    let mut block = Some(best_end_b);
    let mut tower: Vec<Block> = Vec::new();

    // rebuild tower with idx being the block we're looking for
    while let Some(idx) = block {

        tower.push(vec[idx]);

        // goes up until we reach the idx we're looking for
        let p = parent[idx];

        if p == usize::MAX {
            block = None;
        }
        else {
            block = Some(p);
        }
    }

    // tower is made from top to bottom
    tower.reverse();

    // prints all important information on the building tower
    println!("{max_h}");
    for (_, block) in tower.iter().enumerate() {
        println!("{} {} {}", block.width, block.length, block.height);
    }
}
