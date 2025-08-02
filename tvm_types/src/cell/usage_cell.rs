use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::Arc;

use crate::Cell;
use crate::CellImpl;
use crate::CellType;
use crate::LevelMask;
use crate::UInt256;

type VisitedSet = Arc<parking_lot::Mutex<HashSet<UInt256>>>;
type VisitedMap = Arc<parking_lot::Mutex<HashMap<UInt256, Cell>>>;

#[derive(Clone)]
enum Visited {
    Set(VisitedSet),
    Map(VisitedMap),
}

#[derive(Clone)]
pub struct UsageCell {
    cell: Cell,
    visit_on_load: bool,
    visited: Visited,
    usage_level: u64,
}

impl UsageCell {
    fn new(inner: Cell, visit_on_load: bool, usage: Visited) -> Self {
        let usage_level = inner.usage_level() + 1;
        assert!(usage_level <= 1, "Nested usage cells can cause stack overflow");
        let cell = Self { cell: inner, visit_on_load, visited: usage, usage_level };
        if visit_on_load {
            cell.visit();
        }
        cell
    }

    fn visit(&self) -> bool {
        match &self.visited {
            Visited::Set(visited) => {
                visited.lock().insert(self.cell.repr_hash());
            }
            Visited::Map(visited) => {
                visited.lock().insert(self.cell.repr_hash(), self.cell.clone());
            }
        }
        true
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
        if self.visit_on_load && self.visit() {
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

#[derive(Clone, Default)]
pub struct UsageTree {
    root: Cell,
    visited: VisitedMap,
}

impl UsageTree {
    pub fn with_root(root: Cell) -> Self {
        Self::with_params(root, false)
    }

    pub fn with_params(root: Cell, visit_on_load: bool) -> Self {
        let visited = Arc::new(parking_lot::Mutex::new(HashMap::new()));
        let root =
            Cell::with_usage(UsageCell::new(root, visit_on_load, Visited::Map(visited.clone())));
        Self { root, visited }
    }

    pub fn use_cell(&self, cell: Cell, visit_on_load: bool) -> Cell {
        let usage_cell = UsageCell::new(cell, visit_on_load, Visited::Map(self.visited.clone()));
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
        self.visited.lock().contains_key(hash)
    }

    pub fn build_visited_subtree(
        &self,
        is_include: &impl Fn(&UInt256) -> bool,
    ) -> crate::Result<HashSet<UInt256>> {
        Self::build_visited_subtree_inner(&self.visited.lock(), is_include)
    }

    fn build_visited_subtree_inner(
        visited: &HashMap<UInt256, Cell>,
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
        visited: &HashMap<UInt256, Cell>,
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
        for hash in self.visited.lock().keys() {
            visited.insert(hash.clone());
        }
        visited
    }
}

#[derive(Clone, Default)]
pub struct UsageSet {
    root: Cell,
    visited: VisitedSet,
}

impl UsageSet {
    pub fn with_root(root: Cell) -> Self {
        Self::with_params(root, false)
    }

    pub fn with_params(root: Cell, visit_on_load: bool) -> Self {
        let visited = Arc::new(parking_lot::Mutex::new(HashSet::new()));
        let root =
            Cell::with_usage(UsageCell::new(root, visit_on_load, Visited::Set(visited.clone())));
        Self { root, visited }
    }

    pub fn use_cell(&self, cell: Cell, visit_on_load: bool) -> Cell {
        let usage_cell = UsageCell::new(cell, visit_on_load, Visited::Set(self.visited.clone()));
        usage_cell.visit();
        Cell::with_usage(usage_cell)
    }

    pub fn root_cell(&self) -> Cell {
        self.root.clone()
    }

    pub fn build_visited_set(&self) -> HashSet<UInt256> {
        self.visited.lock().clone()
    }

    pub fn take_visited_set(&mut self) -> HashSet<UInt256> {
        mem::take(&mut self.visited.lock())
    }
}
