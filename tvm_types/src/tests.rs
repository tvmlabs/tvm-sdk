use crate::{BocReader, BocWriter, BocWriterOptions, Cell};
use std::fs::read;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[test]
fn test_boc_reader_writer() {
    let repo_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    println!("Collecting BOC files: {}", repo_dir.display());
    let mut boc_paths = Vec::new();
    collect_boc_files(repo_dir.join("tvm_block/src/tests/data"), &mut boc_paths).unwrap();
    collect_boc_files(repo_dir.join("tvm_block_json/src/tests/data"), &mut boc_paths).unwrap();

    for path in boc_paths {
        println!("Testing file {}", path.file_name().unwrap().to_string_lossy());
        test_boc_file(&path);
    }
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
fn read_boc_ex(boc: &[u8], in_mem: bool, force_cell_finalization: bool) -> Vec<Cell> {
    let reader = BocReader::new();
    if in_mem {
        reader
            .read_inmem_ex(Arc::new(boc.to_vec()), 0, force_cell_finalization)
            .expect("Error deserializing BOC")
            .roots
    } else {
        reader.read(&mut Cursor::new(&boc)).expect("Error deserializing BOC").roots
    }
}

fn read_boc_checked(
    boc: &[u8],
    orig_cells: &[Cell],
    in_mem: bool,
    force_finalize_cells: bool,
    info: String,
) -> Vec<Cell> {
    let cells = read_boc_ex(boc, in_mem, force_finalize_cells);
    if cells.len() != orig_cells.len() {
        panic!("{info} Cells len mismatch: {} != {}", orig_cells.len(), cells.len());
    }
    for i in 0..orig_cells.len() {
        cmp_cell(&orig_cells[i], &cells[i], &info);
    }
    cells
}

fn write_boc_ex(root_cells: &[Cell], include_index: bool, store_hashes: Option<bool>) -> Vec<u8> {
    let mut boc = Vec::new();
    BocWriter::with_roots_ex(
        root_cells.to_vec(),
        BocWriterOptions { store_hashes, ..Default::default() },
    )
    .unwrap()
    .write_ex(&mut boc, include_index, false, None, None)
    .unwrap();
    boc
}

fn info(
    write_index: bool,
    write_store_hashes: Option<bool>,
    read_inmem: bool,
    read_finalize: bool,
) -> String {
    let mut info = "(".to_string();
    if write_index {
        info.push_str("index, ");
    }
    match write_store_hashes {
        Some(true) => info.push_str("store_hashes, "),
        Some(false) => info.push_str("no_store_hashes, "),
        _ => {}
    }
    if read_inmem {
        info.push_str("inmem, ");
    }
    if read_finalize {
        info.push_str("finalize, ");
    }
    info.pop();
    info.pop();
    info.push(')');
    info
}

fn test_boc_file(filename: &Path) -> Cell {
    let orig_bytes =
        read(Path::new(filename)).unwrap_or_else(|_| panic!("Error reading file {:?}", filename));

    let orig_cells = read_boc_ex(&orig_bytes, false, false);

    // try in mem
    for read_inmem in [false, true] {
        for read_force in [false, true] {
            read_boc_checked(
                &orig_bytes,
                &orig_cells,
                read_inmem,
                read_force,
                info(false, None, read_inmem, read_force),
            );
        }
    }

    // try different ser and deser options
    for write_index in [false, true] {
        for write_store_hashes in [Some(false), None, Some(true)] {
            let boc = write_boc_ex(&orig_cells, write_index, write_store_hashes);
            if write_index && write_store_hashes == Some(true) {
                println!("BOC size: {} vs {}", boc.len(), orig_bytes.len());
            }
            for read_inmem in [false, true] {
                for read_force in [false, true] {
                    read_boc_checked(
                        &boc,
                        &orig_cells,
                        read_inmem,
                        read_force,
                        info(write_index, write_store_hashes, read_inmem, read_force),
                    );
                }
            }
        }
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
            "{info} Cell repr_hash mismatch: {} != {}",
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
    if a.store_hashes_depths() != b.store_hashes_depths() {
        panic!("{info} Cell store_hashes_depths mismatch");
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
