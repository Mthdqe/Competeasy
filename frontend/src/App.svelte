<script>
  import Match from "./components/Match.svelte";

  class Match_ {
    constructor(first_team, second_team, date, hour, place) {
      this.first_team = first_team;
      this.second_team = second_team;
      this.date = date;
      this.hour = hour;
      this.place = place;
    }
  }

  let matchs = [];

  let team = "";

  function getMatchs(team) {
    let uri = "http://localhost:8000/matchs/" + team;
    fetch(uri).then((response) => {
      response.json().then((json) => {
        matchs = json;
      });
    });
  }

  function onClick() {
    getMatchs(team);
  }
</script>

<main>
  <div id="input">
    <input type="value" bind:value={team} />
    <button on:click={onClick}>Get</button>
  </div>
  <div id="list">
    {#each matchs as match}
      <Match
        first_team={match.first_team}
        second_team={match.second_team}
        date={match.date}
        hour={match.hour}
        place={match.place}
      />
    {/each}
  </div>
</main>

<style>
  #list {
    display: flex;
    flex-wrap: wrap;
  }
</style>
