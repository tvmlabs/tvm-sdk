use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;

use crate::Cell;
use crate::UInt256;
struct VisitedMap {
    map: parking_lot::Mutex<HashMap<UInt256, Cell, UInt256HashBuilder>>,
    dropped: AtomicBool,
    count: AtomicUsize,
}

impl VisitedMap {
    fn new() -> Self {
        VisitedMap {
            map: parking_lot::Mutex::new(HashMap::default()),
            dropped: AtomicBool::new(false),
            count: AtomicUsize::new(0),
        }
    }

    fn is_dropped(&self) -> bool {
        self.dropped.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub struct UsageCell {
    pub(crate) wrapped: Cell,
    visit_on_load: bool,
    visited: Arc<VisitedMap>,
}

impl UsageCell {
    fn new_arc(wrapped: Cell, visit_on_load: bool, visited: Arc<VisitedMap>) -> Arc<UsageCell> {
        let wrapped = if wrapped.is_usage_cell() { wrapped.downcast_usage() } else { wrapped };
        let usage_cell = Self { wrapped, visit_on_load, visited };
        let arc_cell = Arc::new(usage_cell);
        if visit_on_load {
            Self::visit(&arc_cell);
        }
        arc_cell
    }

    fn visit(cell: &Arc<Self>) -> bool {
        if !cell.visited.is_dropped() {
            let mut map = cell.visited.map.lock();
            if map.contains_key(&cell.wrapped.repr_hash()) {
                return true;
            }
            cell.visited.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            map.insert(cell.wrapped.repr_hash(), Cell::Usage(cell.clone()));
            true
        } else {
            false
        }
    }

    pub(crate) fn data(cell: &Arc<Self>) -> &[u8] {
        if !cell.visit_on_load {
            Self::visit(cell);
        }
        cell.wrapped.data()
    }

    pub(crate) fn raw_data(cell: &Arc<Self>) -> crate::Result<&[u8]> {
        if !cell.visit_on_load {
            Self::visit(cell);
        }
        cell.wrapped.raw_data()
    }

    pub fn reference(cell: &Arc<Self>, index: usize) -> crate::Result<Cell> {
        if cell.visit_on_load && !cell.visited.is_dropped() || Self::visit(cell) {
            let child = cell.wrapped.reference(index)?;
            if let Some(existing) = cell.visited.map.lock().get(&child.repr_hash()).cloned() {
                return Ok(existing);
            }
            Ok(Cell::Usage(UsageCell::new_arc(child, cell.visit_on_load, cell.visited.clone())))
        } else {
            cell.wrapped.reference(index)
        }
    }

    pub(crate) fn to_external(cell: &Arc<Self>) -> crate::Result<Cell> {
        Ok(Cell::Usage(UsageCell::new_arc(
            cell.wrapped.to_external()?,
            cell.visit_on_load,
            cell.visited.clone(),
        )))
    }
}

#[derive(Clone)]
pub struct UsageTree {
    root: Cell,
    visited: Arc<VisitedMap>,
}

impl Drop for UsageTree {
    fn drop(&mut self) {
        self.visited.dropped.store(true, std::sync::atomic::Ordering::Release);
    }
}

impl UsageTree {
    pub fn with_root(root: Cell) -> Self {
        Self::with_params(root, false)
    }

    pub fn with_params(root: Cell, visit_on_load: bool) -> Self {
        let visited = Arc::new(VisitedMap::new());
        let root = Cell::Usage(UsageCell::new_arc(root, visit_on_load, visited.clone()));
        Self { root, visited }
    }

    pub fn use_cell(&self, cell: Cell, visit_on_load: bool) -> Cell {
        let usage_cell = UsageCell::new_arc(cell, visit_on_load, self.visited.clone());
        UsageCell::visit(&usage_cell);
        Cell::Usage(usage_cell)
    }

    pub fn use_cell_opt(&self, cell_opt: &mut Option<Cell>, visit_on_load: bool) {
        if let Some(cell) = cell_opt.as_mut() {
            *cell = self.use_cell(cell.clone(), visit_on_load);
        }
    }

    pub fn root_cell(&self) -> Cell {
        self.root.clone()
    }

    pub fn contains(&self, hash: &UInt256) -> bool {
        self.visited.map.lock().contains_key(hash)
    }

    pub fn build_visited_subtree(
        &self,
        is_include: &impl Fn(&UInt256) -> bool,
    ) -> crate::Result<HashSet<UInt256>> {
        Self::build_visited_subtree_inner(&self.visited.map.lock(), is_include)
    }

    fn build_visited_subtree_inner(
        visited: &HashMap<UInt256, Cell, UInt256HashBuilder>,
        is_include: &impl Fn(&UInt256) -> bool,
    ) -> crate::Result<HashSet<UInt256>> {
        let mut subvisited = HashSet::new();
        for (hash, cell) in visited.iter() {
            if is_include(hash) {
                Self::visit_subtree(visited, cell, &mut subvisited)?
            }
        }
        Ok(subvisited)
    }

    fn visit_subtree(
        visited: &HashMap<UInt256, Cell, UInt256HashBuilder>,
        cell: &Cell,
        subvisited: &mut HashSet<UInt256>,
    ) -> crate::Result<()> {
        if subvisited.insert(cell.repr_hash()) {
            for i in 0..cell.references_count() {
                let child_hash = cell.reference_repr_hash(i)?;
                if let Some(child) = visited.get(&child_hash) {
                    Self::visit_subtree(visited, child, subvisited)?
                }
            }
        }
        Ok(())
    }

    pub fn build_visited_set(&self) -> HashSet<UInt256> {
        let mut visited = HashSet::new();
        for hash in self.visited.map.lock().keys() {
            visited.insert(*hash);
        }
        visited
    }

    pub fn take_visited_map(&self) -> HashMap<UInt256, Cell, UInt256HashBuilder> {
        self.visited.dropped.store(true, std::sync::atomic::Ordering::Release);
        self.visited.count.store(0, std::sync::atomic::Ordering::Release);
        std::mem::take(&mut self.visited.map.lock())
    }

    pub fn take_visited_set(&self) -> HashSet<UInt256> {
        HashSet::from_iter(self.take_visited_map().keys().cloned())
    }

    pub fn total_visited_count(&self) -> usize {
        self.visited.count.load(std::sync::atomic::Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell;
    use crate::UsageTree;
    use crate::read_single_root_boc;
    #[test]
    fn test_usage_cell_speed() {
        let original_bytes =
            std::fs::read("../tvm_block/src/tests/data/block_with_ss/shard-states/571524").unwrap();
        let original = read_single_root_boc(&original_bytes).unwrap();
        // Original tree has 37395 cells
        let micros = traverse_cell(&original, 0);
        println!("No usage   micros: {micros}");
        let micros = traverse_cell(&original, 1000);
        println!("1000 usage micros: {micros}");
        let micros = traverse_cell(&original, 10000000);
        println!("Full usage micros: {micros}");
    }

    fn traverse_cell(cell: &Cell, drop_usage_tree_after: usize) -> usize {
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let mut usage = Some(UsageTree::with_root(cell.clone()));
            let usage_root = usage.as_ref().map(|x| x.root_cell()).unwrap();
            let mut visited_cells = 0;
            traverse_cell_inner(
                if drop_usage_tree_after > 0 { &usage_root } else { cell },
                &mut visited_cells,
                &mut usage,
                drop_usage_tree_after,
            );
        }
        start.elapsed().as_micros() as usize
    }

    fn traverse_cell_inner(
        cell: &Cell,
        visited_cells: &mut usize,
        usage_tree: &mut Option<UsageTree>,
        drop_usage_tree_after: usize,
    ) {
        *visited_cells += 1;
        if *visited_cells == drop_usage_tree_after {
            *usage_tree = None;
        }
        for child in cell.clone_references() {
            traverse_cell_inner(&child, visited_cells, usage_tree, drop_usage_tree_after);
        }
    }
}

#[derive(Default, Clone)]
pub struct UInt256HashBuilder;

impl BuildHasher for UInt256HashBuilder {
    type Hasher = UInt256Hasher;

    fn build_hasher(&self) -> UInt256Hasher {
        UInt256Hasher { hash: 0 }
    }
}

#[derive(Default)]
pub struct UInt256Hasher {
    hash: u64,
}

impl Hasher for UInt256Hasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hash = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    }

    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }
}
