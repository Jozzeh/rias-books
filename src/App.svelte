<script lang="ts">
  import { Router, Link, Route } from 'svelte-routing';
  import BooksPage from "./components/pages/BooksPage/BooksPage.svelte";
  import PurchaseOrderPage from "./components/pages/PurchaseOrderPage/PurchaseOrderPage.svelte";
  import InvoicesPage from "./components/pages/InvoicesPage/InvoicesPage.svelte";
  import CreditNotesPage from "./components/pages/CreditNotesPage/CreditNotesPage.svelte";
  import SideNavigation from "./components/elements/SideNavigation/SideNavigation.svelte";
</script>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Montserrat&display=swap" rel="stylesheet">

<main class="layout">
    <SideNavigation />
    <div class="content">
        <Router>
            <Route path="/" component={BooksPage} />
            <Route path="/orders" component={PurchaseOrderPage} />
            <Route path="/invoices" component={InvoicesPage} />
            <Route path="/creditNotes" component={CreditNotesPage} />
        </Router>
    </div>
</main>
