<script lang="ts">
    import {Plus} from "lucide-svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    
    interface Project {
        id: number;
        name: string;
        description: string;
        created_at: string;
        visibility: "public" | "private";
    }

    interface ApiResponse {
        projects: Project[];
        success: boolean;
        message?: string;
    }

    // State with explicit types
    let projects: Project[] = [];
    let isLoading: boolean = true;
    let errorMessage: string | null = null;

    // Fetch Projects
    async function fetchProjects(): Promise<void> {
        isLoading = true;
        errorMessage = null;

        try {
            // We tell invoke that it returns an ApiResponse
            const data = await invoke<ApiResponse>("get_projects");

            projects = data.projects.map((p) => ({
                ...p,
                // Format date string just like your Flutter code's split('.').first
                created_at: p.created_at
                    ? new Date(p.created_at)
                          .toISOString()
                          .replace("T", " ")
                          .split(".")[0]
                    : "N/A",
            }));
        } catch (e) {
            errorMessage = "Failed to load projects. Please try again.";
            console.error(e);
        } finally {
            isLoading = false;
        }
    }

    // Delete Project
    async function deleteProject(id: number): Promise<void> {
        if (!confirm("Are you sure you want to delete this project?")) return;

        try {
            // Pass the argument as an object matching the Rust function signature
            await invoke("delete_project", { projectID: id });
            await fetchProjects();
        } catch (e) {
            alert("Could not delete project");
        }
    }

    onMount(() => {
        fetchProjects();
    });
</script>

<div class="p-6">
    <header class="mb-4">
        <h1>My Projects</h1>
    </header>

    {#if isLoading}
        <div class="status-container">
            <div class="loader"></div>
        </div>
    {:else if errorMessage}
        <div class="status-container">
            <p class="error">{errorMessage}</p>
        </div>
    {:else if projects.length === 0}
        <div class="status-container">
            <p class="empty">No projects yet. Tap + to create one.</p>
        </div>
    {:else}
        <div class="project-grid">
            {#each projects as project (project.id)}
                <a
                    
                    class="project-card"
                    aria-busy="false"
                    href='/home/projects/${project.id}'
                >
                    <div class="card-body">
                        <h3 class="text-white">{project.name}</h3>
                        <p class="desc">
                            {project.description || "No description"}
                        </p>
                        <p class="timestamp">Created: {project.created_at}</p>

                        <div class="actions">
                            <button
                                class="sq-btn edit"
                                onclick={() => {}}
                            >
                                <span>✏️</span> Edit
                            </button>
                            <button
                                class="sq-btn delete"
                                onclick={() =>
                                    deleteProject(project.id)}
                            >
                                <span>🗑️</span> Delete
                            </button>
                        </div>
                    </div>
                </a>
            {/each}
        </div>
    {/if}

    <button class="floating floating-button" onclick={() => alert("Open Create Modal")}>
        <Plus size={24} />
    </button>
</div>

<style>

    .project-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: 1.5rem;
    }

    .project-card {
        background-color: #1a2a26; /* Dark teal from your image */
        border-radius: 1.25rem;
        padding: 1.5rem;
        transition: all 0.2s ease;
        border: 1px solid rgba(255, 255, 255, 0.05);
        cursor: pointer;
    }

    .project-card:hover {
        background-color: #223a35;
        transform: translateY(-2px);
    }

    h3 {
        margin: 0 0 0.5rem 0;
        font-weight: 600;
    }
    .desc {
        font-size: 0.95rem;
        color: #acb6bf;
        margin-bottom: 1rem;
    }
    .timestamp {
        font-size: 0.8rem;
        color: #6e7681;
        margin-bottom: 1.5rem;
    }

    .actions {
        display: flex;
        gap: 0.75rem;
    }

    /* Squircle Button Style */
    .sq-btn {
        border: none;
        border-radius: 0.75rem;
        padding: 0.5rem 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 500;
        cursor: pointer;
    }

    .edit {
        background-color: #fef08a;
        color: #1c1917;
    }
    .delete {
        background-color: #fca5a5;
        color: #1c1917;
    }

    .status-container {
        display: flex;
        justify-content: center;
        padding-top: 5rem;
    }
</style>
