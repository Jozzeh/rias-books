<script>
  // @ts-nocheck
  import { navigate } from "svelte-routing";

  import "./CreateBook.css";
  import { getMainDb } from "../../../../funcs/db";
  import { fetch } from "@tauri-apps/api/http";
  // export let params;

  let isbn = "";
  let isbnError = "";
  let bookLoading = false;
  let title = "";
  let author = "";
  let price = 0;

  async function submitForm() {
    const db = await getMainDb();
    const isbnBook = await db.select("SELECT * FROM books WHERE isbn=$1", [
      isbn,
    ]);
    if (isbnBook.length >= 1) {
      isbnError = "Er is al een boek met dit ISBN nummer in de database.";
    } else {
      await db.execute(
        "INSERT INTO books (isbn, title, author, price) VALUES ($1, $2, $3, $4)",
        [isbn, title, author, Number(price)]
      );
      navigate("/");
    }
  }

  async function getBookDetails() {
    if (isbn.length === 10 || isbn.length === 13 || isbn.length === 16) {
      bookLoading = true;
      const response = await fetch(
        "https://openlibrary.org/search.json?lang=nl&isbn=" + isbn
      );
      if (response.data.numFound === 1) {
        isbnError = "";
        author = response.data.docs[0].author_name.join(", ");
        title = response.data.docs[0].title;
      } else if (response.data.numFound === 0) {
        isbnError =
          "Boek niet gevonden in database. Gelieve manueel aan te vullen.";
      } else {
        isbnError =
          "ISBN nummer komt overeen met meerdere boeken... Dat kan niet :-|";
      }
      bookLoading = false;
    }
  }
</script>

<header class="bookCreateHeader">
  <div class="bookCreateTitle"><h1>Voeg boek toe</h1></div>
  <div>
    <button
      class="secondary"
      on:click={() => {
        navigate("/");
      }}>x</button
    >
  </div>
</header>
<main>
  <form
    on:submit|preventDefault={() => {
      submitForm();
    }}
  >
    <div class="formGroup">
      <label for="isbn">isbn</label>
      <div class="formInput">
        <input
          name="isbn"
          required
          id="isbn"
          type="text"
          bind:value={isbn}
          on:keyup={(evt) => {
            getBookDetails();
          }}
        />
        {#if bookLoading === true}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24px"
            height="24px"
            viewBox="0 0 24 24"
            {...$$props}
          >
            <path
              fill="none"
              stroke="currentColor"
              stroke-dasharray="15"
              stroke-dashoffset="15"
              stroke-linecap="round"
              stroke-width="3"
              d="M12 3C16.9706 3 21 7.02944 21 12"
            >
              <animate
                fill="freeze"
                attributeName="stroke-dashoffset"
                dur="0.3s"
                values="15;0"
              />
              <animateTransform
                attributeName="transform"
                dur="1.5s"
                repeatCount="indefinite"
                type="rotate"
                values="0 12 12;360 12 12"
              />
            </path>
          </svg>
        {/if}
      </div>
      {#if isbnError.length !== 0}
        <span class="errorText">{isbnError}</span>
      {/if}
    </div>
    <div class="formGroup">
      <label for="title">titel</label>
      <input name="title" bind:value={title} required id="title" type="text" />
    </div>
    <div class="formGroup">
      <label for="author">auteur</label>
      <input
        name="author"
        bind:value={author}
        required
        id="author"
        type="text"
      />
    </div>
    <div class="formGroup">
      <label for="price">prijs in cents (excl. BTW)</label>
      <input
        name="price"
        bind:value={price}
        step="1"
        min="1"
        required
        id="price"
        type="number"
      />
    </div>

    <input type="submit" value="Opslaan" />
  </form>
</main>
