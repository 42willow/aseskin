<script>
  import "../app.css";
  import Navbar from "../components/Navbar/Navbar.svelte";
  import Sidebar from "../components/Sidebar/Sidebar.svelte";
</script>

<Navbar />
<slot />
<Sidebar />
