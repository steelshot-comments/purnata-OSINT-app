<script lang="ts">
    import { Mail, User, Lock, KeyRound, ArrowLeft, ShieldCheck } from "lucide-svelte";
    import { fade, fly } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import Auth from "$lib/components/Auth.svelte";

    // App State
    let step = $state("auth"); // 'auth', 'mfa', or 'setup'
    let isLogin = $state(true);
    let loading = $state(false);

    // Form Fields
    let email = $state("");
    let display_name = $state("");
    let password = $state("");
    
    // Auth Data
    let secretCode = $state("");
    let mfaTicket = $state("");    // Used for existing MFA challenge
    let sessionToken = $state(""); // Used for initial MFA setup
    let otpCode = $state("");      // Code for logging in
    let setupCode = $state("");    // Code for initial setup
    let qrSvg = $state("");        // The SVG string from Supabase
    let factorId = $state("");     // The ID of the MFA factor being created

    /**
     * Handles Initial Login or Signup
     */
    async function handleSubmit() {
    loading = true;
    try {
      if (isLogin) {
        const result: any = await invoke("login", { email, password });
        if (result.mfa_required) {
          mfaTicket = result.ticket;
          step = "challenge"; // Existing user MFA
        } else {
          sessionToken = result.access_token;
          step = "setup"; // New user force setup
        }
      } else {
        await invoke("signup", { email, password, displayName: display_name });
        alert("Check email to confirm!");
        // isLogin = true;
      }
    } catch (e) { alert(e); } finally { loading = false; }
  }

    /**
     * Step 1 of Setup: Request QR Code from Supabase
     */
    async function startSetup() {
        loading = true;
        try {
            const data: any = await invoke("enroll_mfa", { sessionToken });
            qrSvg = data.totp.qr_code;
            secretCode = data.totp.secret;
            factorId = data.id;
            step = "setup";
        } catch (e) {
            alert("Enrollment failed: " + e);
            // Fallback: let them in anyway if setup fails
            await goto("/home/projects");
        } finally {
            loading = false;
        }
    }

    /**
     * Step 2 of Setup: Verify the first code to activate MFA
     */
    async function confirmSetup() {
        loading = true;
        try {
            await invoke("verify_factor", { factorId, code: setupCode, sessionToken });
            alert("MFA successfully enabled!");
            await goto("/home/projects");
        } catch (e) {
            alert("Verification failed: " + e);
        } finally {
            loading = false;
        }
    }

    /**
     * Verification for returning users (MFA Challenge)
     */
    async function verifyOtp() {
        loading = true;
        try {
            await invoke("verify_totp", { code: otpCode, ticket: mfaTicket });
            await goto("/home/projects");
        } catch (e) {
            alert("Invalid OTP code. Please try again.");
        } finally {
            loading = false;
        }
    }
</script>

<div class="min-h-screen w-full bg-[#1a2332] flex flex-col items-center justify-center p-6">
  <div class="w-full max-w-sm z-10 flex flex-col gap-6" in:fade>
    
    {#if step === "auth"}
      <div class="bg-white rounded-xl shadow-2xl overflow-hidden divide-y divide-gray-100">
                <div class="flex items-center p-4 gap-4">
                    <Mail size={20} class="text-gray-600" />
                    <input type="email" bind:value={email} placeholder="Email" class="grow outline-none text-gray-800" />
                </div>

                {#if !isLogin}
                    <div class="flex items-center p-4 gap-4" transition:fly={{ y: -10 }}>
                        <User size={20} class="text-gray-600" />
                        <input type="text" bind:value={display_name} placeholder="Display name" class="grow outline-none text-gray-800" />
                    </div>
                {/if}

                <div class="flex items-center p-4 gap-4">
                    <Lock size={20} class="text-gray-600" />
                    <input type="password" bind:value={password} placeholder="Password" class="grow outline-none text-gray-800" />
                </div>
            </div>

            <button onclick={handleSubmit} disabled={loading} class="w-full py-4 bg-linear-to-r from-[#449982] to-[#7db4a2] text-white font-semibold rounded-xl shadow-lg hover:opacity-90 transition-all disabled:opacity-50">
                {loading ? "Processing..." : isLogin ? "Sign in" : "Sign up"}
            </button>

            <button onclick={() => (isLogin = !isLogin)} class="text-gray-300 text-sm hover:text-white transition-colors">
                {isLogin ? "Don't have an account? Sign up" : "Already have an account? Sign in"}
            </button>

    {:else}
      <Auth 
        mode={step === "setup" ? "setup" : "challenge"}
        {mfaTicket}
        {sessionToken}
        onComplete={() => goto("/home/projects")}
        onCancel={() => step = "auth"}
      />
    {/if}

  </div>
</div>

<style>
    /* Ensures the SVG QR code scales correctly */
    :global(.qr-container svg) {
        width: 180px !important;
        height: 180px !important;
        display: block;
    }
</style>