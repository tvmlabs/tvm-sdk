
#[test]
fn test_account_merkle_update() -> anyhow::Result<()> {
    use tvm_types::Cell;

    use tvm_block::Deserializable;

    // restrict memory allocation for this test
    rlimit::setrlimit(rlimit::Resource::AS, 512 * 1024 * 1024, 512 * 1024 * 1024)
        .expect("Failed to set memory limit");

    let old_cell = Cell::construct_from_file("tests/data/test_accounts/b48dc01a6674e616f1f7c4c5e7212af4227b95c3111e1c2051850c35425f7d68:b48dc01a6674e616f1f7c4c5e7212af4227b95c3111e1c2051850c35425f7d68_old").expect("Failed to load cell from file");
    let new_cell = Cell::construct_from_file("tests/data/test_accounts/b48dc01a6674e616f1f7c4c5e7212af4227b95c3111e1c2051850c35425f7d68:b48dc01a6674e616f1f7c4c5e7212af4227b95c3111e1c2051850c35425f7d68_new").expect("Failed to load cell from file");
    let merkle_update = tvm_block::MerkleUpdate::create(&old_cell, &new_cell).ok();
    assert!(merkle_update.is_some());
    Ok(())
}
