use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Weak;

use crate::Cell;
use crate::CellImpl;
use crate::CellType;
use crate::LevelMask;
use crate::UInt256;

#[derive(Clone)]
pub struct UsageCell {
    cell: Cell,
    visit_on_load: bool,
    visited: Weak<lockfree::map::Map<UInt256, Cell>>,
    usage_level: u64,
}

impl UsageCell {
    fn new(
        inner: Cell,
        visit_on_load: bool,
        visited: Weak<lockfree::map::Map<UInt256, Cell>>,
    ) -> Self {
        let usage_level = inner.usage_level() + 1;
        assert!(usage_level <= 1, "Nested usage cells can cause stack overflow");
        let cell = Self { cell: inner, visit_on_load, visited, usage_level };
        if visit_on_load {
            cell.visit();
        }
        cell
    }

    fn visit(&self) -> bool {
        if let Some(visited) = self.visited.upgrade() {
            visited.insert(self.cell.repr_hash(), self.cell.clone());
            return true;
        }
        false
    }
}

impl CellImpl for UsageCell {
    fn data(&self) -> &[u8] {
        if !self.visit_on_load {
            self.visit();
        }
        self.cell.data()
    }

    fn raw_data(&self) -> crate::Result<&[u8]> {
        if !self.visit_on_load {
            self.visit();
        }
        self.cell.raw_data()
    }

    fn bit_length(&self) -> usize {
        self.cell.bit_length()
    }

    fn references_count(&self) -> usize {
        self.cell.references_count()
    }

    fn reference(&self, index: usize) -> crate::Result<Cell> {
        if self.visit_on_load && self.visited.upgrade().is_some() || self.visit() {
            let cell = self.cell.reference(index)?;
            let cell = if cell.is_usage_cell() { cell.downcast_usage() } else { cell };
            Ok(Cell::with_usage(UsageCell::new(cell, self.visit_on_load, self.visited.clone())))
        } else {
            self.cell.reference(index)
        }
    }

    fn cell_type(&self) -> CellType {
        self.cell.cell_type()
    }

    fn level_mask(&self) -> LevelMask {
        self.cell.level_mask()
    }

    fn hash(&self, index: usize) -> UInt256 {
        self.cell.hash(index)
    }

    fn depth(&self, index: usize) -> u16 {
        self.cell.depth(index)
    }

    fn store_hashes(&self) -> bool {
        self.cell.store_hashes()
    }

    fn tree_bits_count(&self) -> u64 {
        self.cell.tree_bits_count()
    }

    fn tree_cell_count(&self) -> u64 {
        self.cell.tree_cell_count()
    }

    fn usage_level(&self) -> u64 {
        self.usage_level
    }

    fn is_usage_cell(&self) -> bool {
        true
    }

    fn downcast_usage(&self) -> Cell {
        self.cell.clone()
    }

    fn to_external(&self) -> crate::Result<Cell> {
        Ok(Cell::with_usage(UsageCell::new(
            self.cell.to_external()?,
            self.visit_on_load,
            self.visited.clone(),
        )))
    }
}

#[derive(Default)]
pub struct UsageTree {
    root: Cell,
    visited: Arc<lockfree::map::Map<UInt256, Cell>>,
}

impl UsageTree {
    pub fn with_root(root: Cell) -> Self {
        let visited = Arc::new(lockfree::map::Map::new());
        let root = Cell::with_usage(UsageCell::new(root, false, Arc::downgrade(&visited)));
        Self { root, visited }
    }

    pub fn with_params(root: Cell, visit_on_load: bool) -> Self {
        let visited = Arc::new(lockfree::map::Map::new());
        let root = Cell::with_usage(UsageCell::new(root, visit_on_load, Arc::downgrade(&visited)));
        Self { root, visited }
    }

    pub fn use_cell(&self, cell: Cell, visit_on_load: bool) -> Cell {
        let usage_cell = UsageCell::new(cell, visit_on_load, Arc::downgrade(&self.visited));
        usage_cell.visit();
        Cell::with_usage(usage_cell)
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
        self.visited.get(hash).is_some()
    }

    pub fn build_visited_subtree(
        &self,
        is_include: &impl Fn(&UInt256) -> bool,
    ) -> crate::Result<HashSet<UInt256>> {
        let mut subvisited = HashSet::new();
        for guard in self.visited.iter() {
            if is_include(guard.key()) {
                self.visit_subtree(guard.val(), &mut subvisited)?
            }
        }
        Ok(subvisited)
    }

    fn visit_subtree(&self, cell: &Cell, subvisited: &mut HashSet<UInt256>) -> crate::Result<()> {
        if subvisited.insert(cell.repr_hash()) {
            for i in 0..cell.references_count() {
                let child_hash = cell.reference_repr_hash(i)?;
                if let Some(guard) = self.visited.get(&child_hash) {
                    self.visit_subtree(guard.val(), subvisited)?
                }
            }
        }
        Ok(())
    }

    pub fn build_visited_set(&self) -> HashSet<UInt256> {
        let mut visited = HashSet::new();
        for guard in self.visited.iter() {
            visited.insert(*guard.key());
        }
        visited
    }
}
