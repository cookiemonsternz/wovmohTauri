# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Data Structure / Distribution

For this project, I will be approaching it through the method of only having data which is needed where it is needed. For instance, information about node positions and colours will not be stored in the backend and propogated to the frontend, rather it will only ever be present in the backend while loading or saving.

In practice this leads to a distribution of data like:

#### Backend:

Graph:
    Nodes[]:
        id: int
        type: enum (Node type)
        fields (inputs)[pin]:
            id: int
            dataType: enum
            value: dataType
        outputs[pin]:
            id: int
            dataType: enum (Data type)