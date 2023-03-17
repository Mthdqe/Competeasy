<!-- 
  -- \file App.svelte
  --
  -- \brief This filte is the main component of the website, it regroups the
  --        different components of the application to make it works.
  --
  -- \author Mathieu Dique
  -->
<script>
    /* Requests */
    import { get_competitions } from "./requests";

    /* Svelte components */
    import Competition from "./components/Competition.svelte";
    import Region from "./components/Region.svelte";
    import { onMount } from "svelte";
    import Department from "./components/Department.svelte";

    /* Hold the elements sent by the backend */
    let competitions = [];
    let regions = [];
    let departments = [];

    /* Once the page pop-up, we get the different available competitions */
    onMount(async () => {
        competitions = await get_competitions();
    });

    /* Reactive code to empty the lists so that they don't display if required */
    $: if (regions.length != 0) competitions = [];
    $: if (departments.length != 0) regions = [];
</script>

<!----------------------------------------------------------------------------->
<main>
    <div class="list">
        {#each competitions as competition}
            <Competition
                name={competition.name}
                url={competition.url}
                bind:regions
            />
        {/each}
    </div>

    <div class="list">
        {#each regions as region}
            <Region name={region.name} url={region.url} bind:departments />
        {/each}
    </div>

    <div class="list">
        {#each departments as department}
            <Department name={department.name} url={department.url} />
        {/each}
    </div>
</main>

<!----------------------------------------------------------------------------->
<style>
    main {
        width: 1200px;
        margin: auto;
    }

    .list {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
    }

    #input {
        display: flex;
        justify-content: center;
        width: 300px;
        margin: auto;
    }
</style>
