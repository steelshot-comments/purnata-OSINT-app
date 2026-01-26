<script lang="ts">
  import { ShieldCheck, ArrowLeft, KeyRound } from "lucide-svelte";
  import { fly } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";

  // Svelte 5 Props
  let { mode = "challenge", mfaTicket = "", sessionToken = "", onComplete, onCancel } = $props<{
    mode: "setup" | "challenge";
    mfaTicket?: string;    // Used for challenge
    sessionToken?: string; // Used for setup
    onComplete: () => void;
    onCancel: () => void;
  }>();

  // Component State
  let loading = $state(false);
  let code = $state("");
  let qrSvg = $state("");
  let factorId = $state("");

  // Step 1 of Setup: Request QR Code from Supabase
  async function fetchEnrollment() {
    try {
      const data: any = await invoke("enroll_mfa", { sessionToken });
      qrSvg = data.totp.qr_code;
      factorId = data.id;
    } catch (e) {
      alert("Enrollment failed: " + e);
    }
  }

  // Effect to fetch QR code if we are in setup mode
  $effect(() => {
    if (mode === "setup" && !qrSvg) {
      fetchEnrollment();
    }
  });

  async function verify() {
    loading = true;
    try {
      if (mode === "setup") {
        await invoke("verify_factor", { factorId, code, sessionToken });
      } else {
        await invoke("verify_totp", { code, ticket: mfaTicket });
      }
      onComplete();
    } catch (e) {
      alert("Verification failed: " + e);
    } finally {
      loading = false;
    }
  }
</script>

<div in:fly={{ x: 20 }} class="flex flex-col gap-4 text-center">
  <button onclick={onCancel} class="text-teal-400 flex items-center gap-2 text-sm mb-2 hover:underline">
    <ArrowLeft size={16} /> Back to login
  </button>

  <h2 class="text-white text-2xl font-bold">
    {mode === "setup" ? "Secure Your Account" : "Two-Factor Auth"}
  </h2>
  
  <p class="text-gray-400 text-sm">
    {mode === "setup" 
      ? "Scan this code with your authenticator app." 
      : "Enter the 6-digit code from your app."}
  </p>

  {#if mode === "setup" && qrSvg}
    <div class="bg-white p-4 rounded-xl shadow-2xl flex justify-center mx-auto w-fit qr-container">
      {@html qrSvg}
    </div>
  {/if}

  <div class="bg-white rounded-xl shadow-2xl flex items-center p-4 gap-4 mt-2">
    {#if mode === "setup"}
      <ShieldCheck size={20} class="text-gray-600" />
    {:else}
      <KeyRound size={20} class="text-gray-600" />
    {/if}
    <input
      type="text"
      maxlength="6"
      bind:value={code}
      placeholder="000000"
      class="grow outline-none text-gray-800 text-2xl tracking-[0.5em] font-mono"
    />
  </div>

  <button
    onclick={verify}
    disabled={code.length < 6 || loading}
    class="w-full py-4 bg-teal-500 text-white font-bold rounded-xl shadow-lg hover:bg-teal-400 disabled:opacity-50 transition-all"
  >
    {loading ? "Verifying..." : "Confirm & Continue"}
  </button>
</div>

<style>
  :global(.qr-container svg) {
    width: 180px !important;
    height: 180px !important;
    display: block;
  }
</style>