# book-archive
implementation of a book archiver pallet using Substrate framework

## Overview
This pallet allows an account to archive a book by providing a URL to an item.

### Dispatchable 

* `archive_book(orgin, title, author, url, archiver, timestamp)` - Archive a specified book

### RPC 
* `book_summary( hash(title + author) )` - Retrieve book summary from the archive

## Develop

### setup environment
Ensure that you have the Rust toolchain installed

- Visit [substrate official documentation](https://docs.substrate.io/install/) page for the installation processes.

### build pallet
- Clone the project [repository](https://github.com/cenwadike/book-archive-pallet).

```bash
git clone https://github.com/cenwadike/book-archive-pallet.git
```

- Navigate into the project’s directory.

```bash
cd book-archive-pallet
```

- Run the command below to compile the pallet.

```bash
cargo build --release
```

### test pallet
- Run the command below to test the pallet.

```bash
cargo test
```


