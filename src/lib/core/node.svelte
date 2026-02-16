<script lang="ts">
    import {
        Handle,
        Position,
        useSvelteFlow,
        type NodeProps,
    } from "@xyflow/svelte";
    import { DataType } from "$lib/types/data_types";
    import { type DataValue } from "$lib/types/data_types";

    import InputField from "./input_field.svelte";
    import OutputPin from "./output_pin.svelte";

    let { id, data }: NodeProps = $props();

    let { updateNodeData } = useSvelteFlow();

    interface InputData {
        name: String;
        data_type: DataType;
        default: DataValue;
    }
</script>

<div class="node">
    {#each data.inputs as input}{/each}
    <h1>hi</h1>
    <InputField></InputField>
</div>
