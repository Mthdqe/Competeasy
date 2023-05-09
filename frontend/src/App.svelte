<!-- 
  -- \file App.svelte
  --
  -- \brief This filte is the main component of the website, it regroups the
  --        different components of the application to make it works.
  --
  -- \author Mathieu Dique
  -->
<script>
    /*---------------------------- Constants ---------------------------------*/
    import { Competition, State } from "./constant";

    /*----------------------------- Requests ---------------------------------*/
    import { get_competitions, get_departments, get_regions } from "./requests";

    /*------------------------ Svelte components -----------------------------*/
    import { onMount } from "svelte";
    import SimpleButton from "./components/SimpleButton.svelte";

    /*--------------- Hold the elements sent by the backend ------------------*/
    let competitions = [];
    let regions = [];
    let departments = [];

    /*---------- Actual selected competition/region/departement --------------*/
    let competition = "";
    let region = "";
    let department = "";

    /*------------------------------ State -----------------------------------*/
    let state = State.Competition;

    /*------------------------------------------------------------------------*/
    /* Once the page pop-up, we get the different available competitions */
    onMount(async () => {
        competitions = await get_competitions();
    });

    /*------------------------------------------------------------------------*/
    /* Reactive code to empty the lists so that they don't display if needed */
    $: if (regions.length != 0) competitions = [];
    $: if (departments.length != 0) regions = [];

    /*----------------------- Private functions ------------------------------*/
    /**
     * @brief Get back button clicked function, it allows to get the competition
     *        and to clear the actual regions and departments.
     */
    async function get_back() {
        competitions = await get_competitions();
        state = State.Competition;
        regions = [];
        departments = [];
    }

    /**
     * @brief Competition clicked button function, it allows to set the actual
     *        competition and perform the next displays depending on the
     *        selected competition.
     * @param comp The clicked competition object.
     */
    async function click_competition(comp) {
        competition = comp.name;
        if (
            competition == Competition.Departemental ||
            competition == Competition.Regional
        ) {
            regions = await get_regions(comp.url);
            state = State.Region;
        }
    }

    /**
     * @brief Region clicked button function, it allows to set the actual
     *        region and perform the next displays depending on the selected
     *        competition.
     * @param comp The clicked competition object.
     */
    async function click_region(reg) {
        region = reg.name;

        if (competition == Competition.Departemental) {
            departments = await get_departments(reg.url, reg.name);
            state = State.Department;
        }
    }

    /**
     * @brief Department clicked button function, it allows to set the actual
     *        department perform the next displays depending on the selected
     *        competition.
     * @param comp The clicked competition object.
     */
    async function click_department(dep) {
        department = dep.name;
    }
</script>

<!----------------------------------------------------------------------------->
<main>
    <!-- Get back button-->
    {#if state != State.Competition}
        <SimpleButton text="Retour" fct={get_back} />
    {/if}

    <!-- Displays the list of competitions -->
    <div class="list">
        {#each competitions as competition}
            <SimpleButton
                text={competition.name}
                fct={() => click_competition(competition)}
            />
        {/each}
    </div>

    <!-- Display the list of regions -->
    <div class="list">
        {#each regions as region}
            <SimpleButton text={region.name} fct={() => click_region(region)} />
        {/each}
    </div>

    <!-- Display the list of departments -->
    <div class="list">
        {#each departments as department}
            <SimpleButton
                text={department.name}
                fct={() => click_department(department)}
            />
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
</style>
