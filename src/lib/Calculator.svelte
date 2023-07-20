<script lang="ts">

  import { invoke } from "@tauri-apps/api/tauri"

  let input = "2 3 +"

  let result = ""

  const parse = (input: string): [string, number, number | undefined] => {
    let parts = input.split(' ');
    parts = parts.filter((part: any): boolean => !!part)
    parts = parts.map((part: any) => part.trim())
    if (parts.length === 3) {
      return [ parts[2], +parts[0], +parts[1]]
    }
    else {
      return [ parts[1], +parts[0], undefined]
    }
  }

  async function calculate() {

    const operator_fuctions = {
      '+': "add",
      '-': "sub",
      '*': "mul",
      '/': "div",
      '**': 'power',
      'sin': 'sin',
    }

    const tokens = parse(input)
    if (!!operator_fuctions[tokens[0]]) {
      result = await invoke(operator_fuctions[tokens[0]], { a: tokens[1], b: tokens[2] });
    }
    else {
      result = "invalid operation"
    }
}
</script>

<div>
  <p>please input reverse polish notation</p>
  <form class="row" on:submit|preventDefault={calculate}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={input} />
    <button type="submit">Calculate Result</button>
  </form>
  <p class="result">{ result }</p>
</div>

<style>
  * {
    font-size: 2rem;
  }
  .result {
    margin-top: 3rem;
    font-size: 5rem;
  }
</style>
