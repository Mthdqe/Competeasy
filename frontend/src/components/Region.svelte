<!--
  -- \file Region.svelte
  --
  -- \brief This filte describe the Region component , it allows to spawn
  --        a button with the name of the Region which can be clicked to
  --        retrieve the departments of this region.
  --
  -- \author Mathieu Dique
  -->
<script>
    /* Requests */
    import { get_departments } from "../requests";

    /* Component properties */
    export let name = "";
    export let url = "";
    export let departments = [];

    /* Comportment of the component */
    async function get_departments_wrapper() {
        departments = await get_departments(url, name);
    }
</script>

<!----------------------------------------------------------------------------->
<div>
    <button on:click={get_departments_wrapper}>
        <p>{name}</p>
    </button>
</div>

<!----------------------------------------------------------------------------->
<style>
    div {
        display: inline-block;
        width: 400px;
        background-color: var(--blue);
        margin: 15px;
        padding: 10px;
        border-radius: 10px;
    }

    div button {
        background-color: var(--blue);
    }

    div button p {
        color: var(--fg);
    }
</style>
