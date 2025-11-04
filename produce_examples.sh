DEPTH=5
cargo run -p cell_generator -- --depth $DEPTH > /tmp/out.txt
head -n 1 /tmp/out.txt > contract/ord.txt
head -n 2 /tmp/out.txt | tail -n 1 > contract/non_forged.txt
tail -n 1 /tmp/out.txt > contract/forged.txt
