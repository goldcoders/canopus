<script lang="ts">
export let name: string;

import Sitecard from "./sitecard.svelte";
import Pagetitle from "./pagetitle.svelte";
import Navbar from "./navbar.svelte";
import Footer from "./footer.svelte";
import AlertError from "./alert-error.svelte";
import AlertSuccess from "./alert-success.svelte";
import DeviceSize from "./device-size.svelte";

import { onMount } from "svelte";
import { promisified } from "tauri/api/tauri";

let errorMessage: string;
let successMessage: string;

interface TauriTestResponse {
  message: string;
  value: number;
}

const callTauriTestCMD = async (): Promise<TauriTestResponse> => {
  return await promisified({
    cmd: "tauriTest",
    count: 6,
    payload: {
      state: "some string data",
      data: 17,
    },
  });
};

onMount(
  async (): Promise<void> => {
    try {
      let response = await callTauriTestCMD();
      const { message } = response;
      successMessage = message;
    } catch (error) {
      console.log(error);
      errorMessage = error;
    }
  }
);
</script>

<div class="flex flex-col min-h-screen bg-gray-50">
  <DeviceSize />

  {#if errorMessage}
    <AlertError errorMessage="{errorMessage}" />
  {/if}

  {#if successMessage}
    <AlertSuccess successMessage="{successMessage}" />
  {/if}

  <div class="bg-gray-800 pb-32">
    <Navbar />
    <Pagetitle name="{name}" />
  </div>
  <main class="-mt-32">
    <div class="max-w-7xl mx-auto pb-12 px-4 sm:px-6 lg:px-8">
      <div class="bg-white rounded-lg shadow px-5 py-6 sm:px-6 h-96">
        <Sitecard />
      </div>
    </div>
  </main>
  <Footer name="{name}" />
</div>
