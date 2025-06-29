use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::BocReader;
use crate::BocWriter;
use crate::Cell;
use crate::read_boc3_bytes;
use crate::write_boc3_to_bytes;

struct Stat {
    deser_time: Duration,
    traverse_time: Duration,
    cmp_time: Duration,
    size: usize,
}

#[derive(Default)]
struct Stats {
    times: HashMap<bool, Stat>,
}

impl Stats {
    fn report(
        &mut self,
        use_boc3: bool,
        deser_time: Duration,
        traverse_time: Duration,
        cmp_time: Duration,
        size: usize,
    ) {
        let key = use_boc3;
        if let Some(t) = self.times.get_mut(&key) {
            t.deser_time += deser_time;
            t.traverse_time += traverse_time;
            t.cmp_time += cmp_time;
            t.size += size;
        } else {
            self.times.insert(key, Stat { deser_time, traverse_time, cmp_time, size });
        }
    }
}

#[test]
fn test_boc_reader_writer() {
    let repo_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    println!("Collecting BOC files: {}", repo_dir.display());
    let mut boc_paths = Vec::new();
    collect_boc_files(repo_dir.join("tvm_block/src/tests/data"), &mut boc_paths).unwrap();
    collect_boc_files(repo_dir.join("tvm_block_json/src/tests/data"), &mut boc_paths).unwrap();

    let mut stats = Stats::default();
    for path in &boc_paths {
        println!(
            "Testing file {}, size {}",
            path.file_name().unwrap().to_string_lossy(),
            std::fs::metadata(path).unwrap().len()
        );
        test_boc_file(path, &mut stats);
    }
    println!("total files: {}", boc_paths.len());
    println!("boc3 deser    cmp    traverse     size");
    for use_boc3 in [false, true] {
        if let Some(stat) = stats.times.get(&(use_boc3)) {
            println!(
                "{}   {:6}   {:6}   {:6}  {:8}",
                bs(use_boc3),
                stat.deser_time.as_millis(),
                stat.cmp_time.as_millis(),
                stat.traverse_time.as_millis(),
                stat.size,
            )
        }
    }
}

fn bs(b: bool) -> &'static str {
    if b { "+" } else { "-" }
}

fn collect_boc_files(path: impl AsRef<Path>, result: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let path = Path::new(file!()).parent().unwrap().parent().unwrap().parent().unwrap().join(path);
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                collect_boc_files(&entry_path, result)?;
            } else if entry_path.is_file() {
                match entry_path.extension() {
                    Some(ext) if ext == "boc" => result.push(entry_path),
                    None => result.push(entry_path), // no extension
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
fn read_boc_ex(boc: Arc<Vec<u8>>, use_boc3: bool) -> Vec<Cell> {
    let reader = BocReader::new();
    if use_boc3 {
        read_boc3_bytes(boc, 0).unwrap()

        // let len = boc.len();
        // reader
        //     .read_inmem_ex(boc, 0..len, force_cell_finalization)
        //     .expect("Error deserializing BOC")
        //     .roots
    } else {
        reader.read(&mut Cursor::new(boc.as_slice())).expect("Error deserializing BOC").roots
    }
}

fn read_boc_checked(
    boc: Arc<Vec<u8>>,
    orig_cells: &[Cell],
    use_boc3: bool,
    stats: &mut Stats,
) -> Vec<Cell> {
    let size = boc.len();
    let start = Instant::now();
    let cells = read_boc_ex(boc, use_boc3);
    let deser_time = start.elapsed();
    let info = info(use_boc3);
    if cells.len() != orig_cells.len() {
        panic!("{info} Cells len mismatch: {} != {}", orig_cells.len(), cells.len());
    }
    let start = Instant::now();
    for i in 0..orig_cells.len() {
        cmp_cell(&orig_cells[i], &cells[i], &info);
    }
    let cmp_time = start.elapsed();
    let start = Instant::now();
    for cell in &cells {
        traverse_cell(cell);
    }
    let traverse_time = start.elapsed();
    stats.report(use_boc3, deser_time, traverse_time, cmp_time, size);
    cells
}

fn write_boc_ex(root_cells: &[Cell], boc3: bool) -> Arc<Vec<u8>> {
    if boc3 {
        Arc::new(write_boc3_to_bytes(root_cells).unwrap())
    } else {
        let mut boc = Vec::new();
        BocWriter::with_roots(root_cells.to_vec())
            .unwrap()
            .write_ex(&mut boc, false, false, None, None)
            .unwrap();
        Arc::new(boc)
    }
}

fn info(use_boc3: bool) -> String {
    if use_boc3 { "(boc3)" } else { "" }.to_string()
}

fn test_boc_file(filename: &Path, stats: &mut Stats) -> Cell {
    let orig_bytes = Arc::new(
        read(Path::new(filename)).unwrap_or_else(|_| panic!("Error reading file {filename:?}")),
    );

    let orig_cells = read_boc_ex(orig_bytes.clone(), false);

    // try different ser and deser options
    for use_boc3 in [false, true] {
        let boc = write_boc_ex(&orig_cells, use_boc3);
        read_boc_checked(boc.clone(), &orig_cells, use_boc3, stats);
    }

    let mut cells = orig_cells;
    cells.remove(0)
}

fn cmp_cell(a: &Cell, b: &Cell, info: &str) {
    if a.cell_type() != b.cell_type() {
        panic!("{info} Cell type mismatch: {} != {}", a.cell_type(), b.cell_type());
    }
    if a.repr_hash() != b.repr_hash() {
        panic!(
            "{info} Cell {} repr_hash mismatch: {} != {}",
            a.cell_type(),
            a.repr_hash().to_hex_string(),
            b.repr_hash().to_hex_string()
        );
    }
    if a.data() != b.data() {
        panic!("{info} Cell data mismatch: {:?} != {:?}", a.data(), b.data());
    }
    if a.hashes() != b.hashes() {
        panic!("{info} Cell hashes mismatch: {:?} != {:?}", a.hashes(), b.hashes());
    }
    if a.depths() != b.depths() {
        panic!("{info} Cell depths mismatch: {:?} != {:?}", a.depths(), b.depths());
    }
    if a.level() != b.level() {
        panic!("{info} Cell level mismatch: {} != {}", a.level(), b.level());
    }
    if a.tree_cell_count() != b.tree_cell_count() {
        panic!(
            "{info} Cell tree_cell_count mismatch: {} != {}",
            a.tree_cell_count(),
            b.tree_cell_count()
        );
    }
    if a.tree_bits_count() != b.tree_bits_count() {
        panic!(
            "{info} Cell tree_bits_count mismatch: {} != {}",
            a.tree_bits_count(),
            b.tree_bits_count()
        );
    }
    for i in 0..a.level() + 1 {
        if a.hash(i as usize) != b.hash(i as usize) {
            panic!(
                "{info} Cell hash {} mismatch: {} != {}",
                i,
                a.hash(i as usize).to_hex_string(),
                b.hash(i as usize).to_hex_string()
            );
        }
        if a.depth(i as usize) != b.depth(i as usize) {
            panic!(
                "{info} Cell depth {} mismatch: {} != {}",
                i,
                a.depth(i as usize),
                b.depth(i as usize)
            );
        }
    }
    if a.references_count() != b.references_count() {
        panic!(
            "{info} Cell references_count mismatch: {} != {}",
            a.references_count(),
            b.references_count()
        );
    }
    for i in 0..a.references_count() {
        cmp_cell(&a.reference(i).unwrap(), &b.reference(i).unwrap(), info);
    }
}

fn traverse_cell(a: &Cell) {
    assert!((a.cell_type() as u8) < 200);
    assert!((a.repr_hash().as_slice()[0] as u32) < 1000);
    assert!(a.data().len() < 1000000000);
    assert!(a.hashes().len() < 1000000000);
    assert!(a.depths().len() < 1000000000);
    assert!(a.level() < 100);
    assert!(a.tree_cell_count() < 1000000000);
    assert!(a.tree_bits_count() < 1000000000);
    for i in 0..a.level() + 1 {
        assert!((a.hash(i as usize).as_slice()[0] as u32) < 1000);
        assert!((a.depth(i as usize) as u32) < 100000000);
    }
    // assert!(a.store_hashes_depths().len() < 1000000000);
    for i in 0..a.references_count() {
        traverse_cell(&a.reference(i).unwrap());
    }
}
