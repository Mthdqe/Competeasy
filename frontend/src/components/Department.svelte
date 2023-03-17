<!-- 
  -- \file Department.svelte
  -- 
  -- \brief This file describe the Department component. It allows to spawn a
  --        container with name of a department which can be clicked to get 
  --        the different pools in this department
  --
  -- \author Mathieu dique
  -->
<script>
    /* Component properties */
    export let name = "";
    export let url = "";
</script>

<!----------------------------------------------------------------------------->
<div>
    <button>
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
