//! Running a WASI compiled WebAssembly module with Wasmer.
//!
//! This example illustrates how to run WASI modules with
//! Wasmer. To run WASI we have to have to do mainly 3 steps:
//!
//!   1. Create a `WasiEnv` instance
//!   2. Attach the imports from the `WasiEnv` to a new instance
//!   3. Run the `WASI` module.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example cache --release --features "cranelift,cache"
//! ```
//!
//! Ready?

use tempfile::TempDir;
use wasmer::Store;
use wasmer_cache::Cache;
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = TempDir::new().unwrap();
    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/lib/c-api/examples/assets/qjs.wasm"
    );
    // Let's declare the Wasm module with the text representation.
    let wasm_bin = std::fs::read(wasm_path)?;

    // Write a test app file
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    let import_object = wasmer::imports! {};
    let hash = wasmer_cache::Hash::generate(&wasm_bin);
    let module = wasmer::Module::new(&store, wasm_bin).unwrap();

    // Instantiate from compiled
    //drop(wasmer::Instance::new(&module, &import_object).unwrap());

    // Instantiate from cache
    let mut cache = wasmer_cache::FileSystemCache::new(&path.path()).unwrap();
    cache.store(hash, &module).unwrap();
    let module = unsafe { cache.load(&store, hash) }.unwrap();
    //drop(wasmer::Instance::new(&module, &import_object).unwrap());

    println!("Finished ...");
    Ok(())
}

#[test]
#[cfg(feature = "wasi")]
fn test_cache() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
