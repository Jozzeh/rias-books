# GRID books

This is the main repository of the GRID book application.  
It uses Tauri + Svelte + Typescript.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Database

package: tauri-plugin-sql (https://github.com/tauri-apps/tauri-plugin-sql)  
Use the migrations to add/alter tables.

Example code:

```
    import { getMainDb } from "./src/funcs/db";

    //GET DATABASE CONNECTION
    const db = await getMainDb();

    //INSERT IN DATABASE
    const inserted = await db.execute(
      "INSERT INTO books (isbn, title, author, price) VALUES ('1234567890', 'joske2', 'Bertrand2', 20.05)"
    );

    //GET ALL BOOKS
    const books = await db.select("SELECT * FROM books");

    //GET LAST ADDED BOOK
    const newBook = await db.select(
      "SELECT * FROM books WHERE id=" + inserted.lastInsertId
    );

    //ALTERNATIVE TO GET LAST INSERT ID
    const newID = await db.execute("SELECT * FROM books");
    console.log(newID.lastInsertId)
```
