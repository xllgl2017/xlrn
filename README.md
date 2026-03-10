# xlrn- A library binding for xlnt

## Example
```rust
use xlnt::*;

fn dd() {
    //create
    let (book, sheet) = WorkBook::create_book();
    sheet.cell(0, 0).set_string("123");
    book.save("1.xlsx").unwrap();

    //open
    let book = WorkBook::open("1.xlsx").unwrap();
    let sheet = book.get_sheet(0).unwrap();
    for row in 0..100 {
        for col in 0..100 {
            sheet.cell(row, col).set_string(row * col)
        }
    }
    book.save("1.xlsx").unwrap();
}

```