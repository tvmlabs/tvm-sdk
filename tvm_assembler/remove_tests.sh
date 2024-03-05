#!/bin/bash
rm -f src/test_framework.rs || exit 0
rm -f src/logger.rs || exit 0
rm -f src/bin/check.rs || exit 0
rm -rf target || exit 0
rm -rf real_ton_boc || exit 0
rm -rf src/examples || exit 0
find . -name tests -type d -print0 | xargs -0 rm -fr
find . -name '*.rs' -type f -print0 | xargs -0 sed -i -e '/logger::init/d' -e '/use tvm::logger/d'
find . -name '*.rs' -type f -print0 | xargs -0 sed -i -e '/#\[cfg(feature = "full")\]/d' -e '/logger::init/d' -e '/use tvm::logger/d'
sed -i '/extern crate log4rs/d' src/lib.rs
sed -i '/extern crate parking_lot/d' src/lib.rs
sed -i '/pub mod logger/d' src/lib.rs
sed -i '/#\[cfg(feature = \"use_test_framework\")\]/,/pub mod test_framework;/d' src/lib.rs
sed -i '/extern crate zip;/d' src/lib.rs
sed -i '/\/\/ TBD/d' src/lib.rs

sed -i '/log4rs =/d' Cargo.toml
sed -i '/pretty_assertions =/d' Cargo.toml
sed -i '/libloading =/d' Cargo.toml
sed -i '/parking_lot =/d' Cargo.toml
sed -i '/zip =.*$/d' Cargo.toml
sed -i '/# TBD/d' Cargo.toml
sed -i 's/full = .*$/full = []/' Cargo.toml
find ./ -name Cargo.toml -type f -print0 | xargs -0 sed -i -E "s/git = (\"|')(ssh|https):\/\/(git@)?github.com\/tonlabs\/ever-([A-Za-z0-9_-]*)-private(\.git)?/git =\1https:\/\/github.com\/tonlabs\/ever-\4\5/g"

rm -rf $0
