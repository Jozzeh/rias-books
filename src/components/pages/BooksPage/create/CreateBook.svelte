<script>
  // @ts-nocheck
  import { navigate } from "svelte-routing";

  import "./CreateBook.css";
  import { getMainDb } from "../../../../funcs/db";
  // export let params;

  async function submitForm(evt) {
    const db = await getMainDb();
    const formData = new FormData(evt.target);

    let data = {};
    for (let field of formData.entries()) {
      const [name, value] = field;
      data[name] = value;
    }

    const inserted = await db.execute(
      "INSERT INTO books (isbn, title, author, price) VALUES ($1, $2, $3, $4)",
      [data.isbn, data.title, data.author, Number(data.price)]
    );
    navigate("/");
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
    on:submit|preventDefault={(evt) => {
      submitForm(evt);
    }}
  >
    <div class="formGroup">
      <label for="isbn">isbn</label>
      <input name="isbn" required id="isbn" type="text" />
    </div>
    <div class="formGroup">
      <label for="title">titel</label>
      <input name="title" required id="title" type="text" />
    </div>
    <div class="formGroup">
      <label for="author">auteur</label>
      <input name="author" required id="author" type="text" />
    </div>
    <div class="formGroup">
      <label for="price">prijs in cents (excl. BTW)</label>
      <input name="price" step="1" min="1" required id="price" type="number" />
    </div>

    <input type="submit" value="Opslaan" />
  </form>
</main>
