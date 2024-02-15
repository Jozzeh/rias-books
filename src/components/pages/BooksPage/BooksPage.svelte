<script>
  // export let params;
  import "./BooksPage.css";
  import { navigate } from "svelte-routing";
  import { getMainDb } from "../../../funcs/db";

  /**
   * @type {Array.<{
   * id: Number,
   * isbn: String,
   * title: String,
   * author: String,
   * coverImageUrl: String,
   * price: Number}>}
   */
  let books = [];

  async function getBooks() {
    //GET DATABASE CONNECTION
    const db = await getMainDb();

    //GET ALL BOOKS
    books = await db.select("SELECT * FROM books");
    console.log(books);
  }

  /**
   * @param {number} id
   */
  async function deleteBook(id) {
    //GET DATABASE CONNECTION
    const db = await getMainDb();

    //GET ALL BOOKS
    await db.execute("DELETE FROM books WHERE id=$1", [id]);
    getBooks();
  }

  getBooks();
</script>

<header class="bookListHeader">
  <div class="bookListTitle"><h1>Boeken</h1></div>
  <div>
    <button
      on:click={() => {
        navigate("/books/create");
      }}><iconify-icon icon="mdi:plus"></iconify-icon></button
    >
  </div>
</header>
<main>
  <table>
    <tr
      ><th>id</th><th>isbn</th><th>titel / auteur</th><th>prijs (euro)</th><th>
      </th></tr
    >
    {#each books as book}
      <tr>
        <td>{book.id}</td>
        <td>{book.isbn}</td>
        <td
          >{book.title}
          <br />
          <span class="bookListAuthor">van: {book.author} </span>
        </td>
        <td>&euro; {Number(book.price) / 100}</td>
        <td>
          <button
            class="destroy"
            on:click={() => {
              deleteBook(book.id);
            }}><iconify-icon icon="mdi:trash"></iconify-icon></button
          >
        </td>
      </tr>
    {/each}
  </table>
</main>

<button
  class="tertairy"
  on:click={() => {
    window.print();
  }}>Print</button
>
