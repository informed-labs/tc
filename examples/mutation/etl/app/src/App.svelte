<script>
  import { onMount } from 'svelte';
  import { generateClient } from 'aws-amplify/api';
  import { initialize, subscribeInitialize } from './graphql';
  import { subscribeEnhance, subscribeTransform } from './graphql';
  import { subscribeLoad, subscribeComplete } from './graphql';
  import { Amplify } from 'aws-amplify';
  import config from './config.json';

  let value = 0;
  let job_id;
  let status = '';
  let error = '';

  let auth_token = 'auth123';
  let client;

  onMount(async () => {
    let a = Amplify.configure(config);
    console.log("conf", a);
    client = generateClient();
  })

  async function start() {

    console.log("subscribing initalize: ", job_id);
    await client.graphql({
      query: subscribeInitialize,
      variables: {
	id: job_id
      },
      authToken: auth_token
    }).subscribe({
      next: ({ data }) => {
	console.log("got data: ", data)
	value = data.subscribeInitialize.percentage;
	status = data.subscribeInitialize.status;
      },
      error: (error) => console.error(error)
    });

    console.log("subscribing enhance: ", job_id);
    await client.graphql({
      query: subscribeEnhance,
      variables: {
	id: job_id
      },
      authToken: auth_token
    }).subscribe({
      next: ({ data }) => {
	console.log("got data: ", data)
	value = data.subscribeEnhance.percentage;
	status = data.subscribeEnhance.status;
      },
      error: (error) => console.error(error)
    });

    console.log("subscribing transform: ", job_id);
    await client.graphql({
      query: subscribeTransform,
      variables: {
	id: job_id
      },
      authToken: auth_token
    }).subscribe({
      next: ({ data }) => {
	console.log("got data: ", data)
	value = data.subscribeTransform.percentage;
	status = data.subscribeTransform.status;
      },
      error: (error) => console.error(error)
    });

    console.log("subscribing loader: ", job_id);
    await client.graphql({
      query: subscribeLoad,
      variables: {
	id: job_id
      },
      authToken: auth_token
    }).subscribe({
      next: ({ data }) => {
	console.log("got data: ", data)
	value = data.subscribeLoad.percentage;
	status = data.subscribeLoad.status;
      },
      error: (error) => console.error(error)
    });

    console.log("subscribing complete: ", job_id);
    await client.graphql({
      query: subscribeComplete,
      variables: {
	id: job_id
      },
      authToken: auth_token
    }).subscribe({
      next: ({ data }) => {
	console.log("got data: ", data)
	value = data.subscribeComplete.percentage;
	status = data.subscribeComplete.status;
      },
      error: (error) => console.error(error)
    });

    console.log("Initialize: ", job_id)
    const start_response = await client.graphql({
      query: initialize,
      variables: {
	id: job_id
      },
      authToken: auth_token
    });
    console.log(start_response);

  }

</script>


<main>
  <div>
    <h1>ETL</h1>

    <form on:submit|preventDefault={start}>
      <input type="text" bind:value={job_id} id="job_id" />
      <button type="submit">Initialize ETL</button>
      {#if error}
	{error}
      {/if}
    </form>


    <br/>


    {#if value > 0 }
      <p> {status} - {value}% </p>
      <progress max="100" {value}>{value}%</progress>
    {/if}

    </div>

</main>


<style>
  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    width: 100%;
    max-width: 80rem;
    margin: 0 auto;
    box-sizing: border-box;
  }
  progress[value] {
    --w: 600px; /* The width */

    /* The background property */
    --b:/* highlight */
      linear-gradient(#fff8, #fff0),
      /* stripes */
      repeating-linear-gradient(135deg, #0003 0 10px, #0000 0 20px),
      /* dynamic layers */
      /* if < 30% "red" */
      linear-gradient(red 0 0) 0 / calc(var(--w) * .3 - 100%) 1px,
      /* if < 60% "orange" */
      linear-gradient(orange 0 0) 0 / calc(var(--w) * .6 - 100%) 1px,
      /* else "green" */
      green;

    appearance: none;
    border: none;
    width: var(--w);
    height: 20px;
    display: block;
    margin: 10px;
    background-color: lightgrey;
    border-radius: 50px;
  }
  progress[value]::-webkit-progress-bar {
    background-color: lightgrey;
    border-radius: 50px;
  }
  progress[value]::-webkit-progress-value {
    border-radius: 50px;
    background: var(--b);
  }
  progress[value]::-moz-progress-bar {
    border-radius: 50px;
    background: var(--b);
  }
</style>
