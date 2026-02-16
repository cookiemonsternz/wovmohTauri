<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import {
        SvelteFlow,
        Background,
        Controls,
        Position,
        type EdgeTypes,
        type NodeTypes,
    } from "@xyflow/svelte";

    import CustomEdge from "./SvelteFlow/CustomEdge.svelte";
    import Node from "$lib/core/node.svelte";

    import "@xyflow/svelte/dist/style.css";

    const nodeTypes: NodeTypes = {
        node: Node,
    };

    const edgeTypes: EdgeTypes = {
        "custom-edge": CustomEdge,
    };

    let name = $state("");
    let greetMsg = $state("");

    async function greet(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        greetMsg = await invoke("greet", { name });
    }

    let nodes = $state.raw([
        {
            id: "1",
            position: { x: 0, y: 0 },
            data: { label: "1" },
            type: "node",
        },
        {
            id: "2",
            position: { x: 0, y: 100 },
            data: { label: "2" },
            type: "node",
        },
    ]);

    let edges = $state.raw([
        {
            id: "e1-2",
            source: "1",
            target: "2",
            type: "custom-edge",
            animated: true,
        },
    ]);

    function addGraph() {
        console.log("Clicked");
        invoke("add_graph").then(() => {
            console.log("Success");
        });
    }
</script>

<svelte:head>
    <link rel="stylesheet" href="/style/svelte-flow.css" />
</svelte:head>

<main class="container" style="width: 100vw; height: 100vh">
    <button onclick={addGraph}>HI</button>
    <SvelteFlow
        {edgeTypes}
        {nodeTypes}
        defaultEdgeOptions={{ type: "custom-edge" }}
        bind:nodes
        bind:edges
    >
        <Background />
        <Controls />
    </SvelteFlow>
</main>

<style>
</style>
