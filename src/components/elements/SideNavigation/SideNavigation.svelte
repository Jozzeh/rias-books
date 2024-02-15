<script>
  import { navigate } from "svelte-routing";
</script>

<aside class="sidebar">
  <ul class="aside_list">
    <li
      class="aside_list-item"
      on:click={() => {
        navigate("/");
      }}
    >
      Boeken
    </li>
    <li
      class="aside_list-item"
      on:click={() => {
        navigate("/orders");
      }}
    >
      Bestelbonnen
    </li>
    <li
      class="aside_list-item"
      on:click={() => {
        navigate("/invoices");
      }}
    >
      Facturen
    </li>
    <li
      class="aside_list-item"
      on:click={() => {
        navigate("/creditNotes");
      }}
    >
      Creditnota
    </li>
  </ul>
</aside>
