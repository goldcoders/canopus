<script lang="ts">
export let name: string;

import Sitecard from "./sitecard.svelte";
import Pagetitle from "./pagetitle.svelte";
import Navbar from "./navbar.svelte";
import Footer from "./footer.svelte";
</script>
<div class="flex flex-col min-h-screen bg-gray-50">
  <div class="bg-gray-800 pb-32">
    <Navbar/>
    <Pagetitle {name}/>
    </div>
  <main class="-mt-32">
    <div class="max-w-7xl mx-auto pb-12 px-4 sm:px-6 lg:px-8">
      <div class="bg-white rounded-lg shadow px-5 py-6 sm:px-6 h-96">
        <Sitecard/>
      </div>
    </div>
  </main>
  <Footer {name}/>
</div>
