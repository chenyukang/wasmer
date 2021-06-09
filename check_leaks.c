// gcc --std=c99 -L/usr/local/lib check_leaks.c -o check_leaks -lwasmer

#include <stdio.h>
#include <stdlib.h>

#include <assert.h>
#include <wasmer.h>

wasm_byte_vec_t slurp (const char *path) {
    FILE* file = fopen(path, "rb");

    if (NULL == file) {
        perror("fopen");
        exit(1);
    }

    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    fseek(file, 0L, SEEK_SET);
    wasm_byte_vec_t binary;
    wasm_byte_vec_new_uninitialized(&binary, file_size);

    if (fread(binary.data, file_size, 1, file) != 1) {
        perror("> Error loading module!\n");
        exit(1);
    }

    fclose(file);

    return binary;
}

int main(int argc, char *argv[]) {
    assert(argc == 2);

    const char* path = argv[1];
    unsigned i;

    wasm_engine_t* engine = wasm_engine_new();
    wasm_store_t* store = wasm_store_new(engine);

    wasm_byte_vec_t wasm = slurp(path);

    system("ps aux | grep check_leaks");
    fprintf(stdout, "===============================\n");

    for (i=0; i<10; i++) {
        wasm_module_t* module = wasm_module_new(store, &wasm);
        if (NULL == module) {
            fprintf(stderr, "bad module?");
            exit(1);
        }

        wasm_module_delete(module);
    }

    wasm_byte_vec_delete(&wasm);

    system("ps aux | grep check_leaks");

    wasm_store_delete(store);
    wasm_engine_delete(engine);

    fprintf(stdout, "===============================\n");
    system("ps aux | grep check_leaks");
    return 0;
}

