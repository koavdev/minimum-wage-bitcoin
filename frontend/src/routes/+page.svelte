<script lang="ts">
  export let data: { wageData: any };

  const BTC_USD = 122000; // assumed bitcoin price in USD

  type Wage = {
    annual: number | null;
    monthly: number | null;
    hourly: number | null;
    currency: string;
  };

  // Minimal mapping from 3-letter code to display name + emoji flag (alpha-2)
  const COUNTRY_MAP: Record<string, { name: string; flag: string; iso2?: string }> = {
    CZE: { name: 'Czech Republic', flag: '🇨🇿', iso2: 'CZ' },
    RUS: { name: 'Russian Federation', flag: '🇷🇺', iso2: 'RU' },
    MEX: { name: 'Mexico', flag: '🇲🇽', iso2: 'MX' },
    IRL: { name: 'Ireland', flag: '🇮🇪', iso2: 'IE' },
    KOR: { name: 'South Korea', flag: '🇰🇷', iso2: 'KR' },
    LTU: { name: 'Lithuania', flag: '🇱🇹', iso2: 'LT' },
    CAN: { name: 'Canada', flag: '🇨🇦', iso2: 'CA' },
    POL: { name: 'Poland', flag: '🇵🇱', iso2: 'PL' },
    ESP: { name: 'Spain', flag: '🇪🇸', iso2: 'ES' },
    CRI: { name: 'Costa Rica', flag: '🇨🇷', iso2: 'CR' },
    BRA: { name: 'Brazil', flag: '🇧🇷', iso2: 'BR' },
    PRT: { name: 'Portugal', flag: '🇵🇹', iso2: 'PT' },
    HRV: { name: 'Croatia', flag: '🇭🇷', iso2: 'HR' },
    LUX: { name: 'Luxembourg', flag: '🇱🇺', iso2: 'LU' },
    GBR: { name: 'United Kingdom', flag: '🇬🇧', iso2: 'GB' },
    ISR: { name: 'Israel', flag: '🇮🇱', iso2: 'IL' },
    LVA: { name: 'Latvia', flag: '🇱🇻', iso2: 'LV' },
    SVK: { name: 'Slovakia', flag: '🇸🇰', iso2: 'SK' },
    DEU: { name: 'Germany', flag: '🇩🇪', iso2: 'DE' },
    GRC: { name: 'Greece', flag: '🇬🇷', iso2: 'GR' },
    HUN: { name: 'Hungary', flag: '🇭🇺', iso2: 'HU' },
    BGR: { name: 'Bulgaria', flag: '🇧🇬', iso2: 'BG' },
    NLD: { name: 'Netherlands', flag: '🇳🇱', iso2: 'NL' },
    AUS: { name: 'Australia', flag: '🇦🇺', iso2: 'AU' },
    PER: { name: 'Peru', flag: '🇵🇪', iso2: 'PE' },
    FRA: { name: 'France', flag: '🇫🇷', iso2: 'FR' },
    USA: { name: 'United States', flag: '🇺🇸', iso2: 'US' },
    NZL: { name: 'New Zealand', flag: '🇳🇿', iso2: 'NZ' },
    MLT: { name: 'Malta', flag: '🇲🇹', iso2: 'MT' },
    ROU: { name: 'Romania', flag: '🇷🇴', iso2: 'RO' },
    BEL: { name: 'Belgium', flag: '🇧🇪', iso2: 'BE' },
    SVN: { name: 'Slovenia', flag: '🇸🇮', iso2: 'SI' },
    EST: { name: 'Estonia', flag: '🇪🇪', iso2: 'EE' },
    COL: { name: 'Colombia', flag: '🇨🇴', iso2: 'CO' },
    TUR: { name: 'Turkey', flag: '🇹🇷', iso2: 'TR' },
    CHL: { name: 'Chile', flag: '🇨🇱', iso2: 'CL' }
  };

  import { writable } from 'svelte/store';
  const selected = writable<string | null>(null);
  // allow undefined so we can clear a rate
  const exchangeRates = writable<Record<string, number | undefined>>({}); // currency -> local per USD
  let rateInput = '';

  function format(n: number | null) {
    if (n === null || n === undefined) return '—';
    // show large numbers with separators
    return new Intl.NumberFormat('en-US', { maximumFractionDigits: 2 }).format(n);
  }

  // strongly type countries and entries for the template
  let countries: Record<string, Wage> = {};
  let metadata: any = {};
  $: countries = (data?.wageData?.countries ?? data?.countries ?? {}) as Record<string, Wage>;
  $: metadata = data?.wageData?.metadata ?? data?.metadata ?? {};
  $: entries = Object.entries(countries) as [string, Wage][];

  // sync local input whenever selected or exchangeRates change
  $: if ($selected) {
    const val = ($exchangeRates && $exchangeRates[$selected]) ?? undefined;
    rateInput = val !== undefined ? String(val) : '';
  }

  // helper to compute BTC equivalents when user provides exchange rate (local per USD)
  function computeBtcEquivalent(localAmount: number | null, rateLocalPerUsd?: number | undefined) {
    if (localAmount === null || localAmount === undefined) return null;
    if (!rateLocalPerUsd || rateLocalPerUsd <= 0) return null;
    const usd = localAmount / rateLocalPerUsd;
    return usd / BTC_USD;
  }

  function openCountry(code: string) {
    selected.set(code);
    // if no exchange rate present, set a sensible default for USD when opening USA
    if (code === 'USA') exchangeRates.update(r => ({ ...r, USA: 1 }));
  }

  function close() {
    selected.set(null);
  }
</script>

{#if !data?.wageData}
  <div class="p-8">
    <p>Carregando ou sem dados</p>
  </div>
{:else}
  <div class="min-h-screen p-6 bg-slate-50 text-slate-900">
    <header class="mb-6 flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-semibold">Comparador: Salário Mínimo × Bitcoin</h1>
        <p class="text-sm text-slate-600 mt-1">Assumindo preço do Bitcoin: <strong>${BTC_USD.toLocaleString()}</strong> USD</p>
        {#if metadata?.name}
          <p class="text-xs text-slate-500 mt-2">{metadata.name}</p>
        {/if}
      </div>
      <div class="text-right text-xs text-slate-500">
        <div>Dados preparados: {new Date(metadata.prepared).toLocaleString()}</div>
      </div>
    </header>

    <main>
      <section class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        {#each Object.entries(countries) as [code, wageRaw]}
          {#key code}
            <button
              on:click={() => openCountry(code)}
              class="bg-white hover:shadow-lg focus:shadow-outline flex flex-col items-start gap-3 p-3 rounded-lg border border-slate-200 text-left transition"
              aria-label={`Abrir ${code}`}
            >
              <div class="flex items-center gap-3 w-full">
                <div class="text-3xl">{COUNTRY_MAP[code]?.flag ?? '🏳️'}</div>
                <div>
                  <div class="text-sm font-medium">{COUNTRY_MAP[code]?.name ?? code}</div>
                  <div class="text-xs text-slate-500">{wageRaw?.currency}</div>
                </div>
              </div>

              <div class="mt-2 text-sm text-slate-700">
                <div>Annual: <strong>{format(wageRaw?.annual)}</strong></div>
                <div>Monthly: <strong>{format(wageRaw?.monthly)}</strong></div>
                <div>Hourly: <strong>{format(wageRaw?.hourly)}</strong></div>
              </div>

              <div class="mt-3 w-full flex items-center justify-between">
                <span class="text-xs text-slate-500">Clique para comparar</span>
                <span class="text-xs bg-slate-100 px-2 py-1 rounded text-slate-700">{wageRaw?.currency}</span>
              </div>
            </button>
          {/key}
        {/each}
      </section>

      <!-- Drawer / Panel -->
      {#if $selected}
        <div class="fixed inset-0 z-40">
          <button
            type="button"
            class="absolute inset-0 bg-black/40 focus:outline-none"
            aria-label="Fechar painel"
            on:click={close}
          ></button>
          <aside class="absolute right-0 top-0 h-full w-full sm:w-96 bg-white p-6 overflow-auto shadow-2xl">
            <div class="flex items-start justify-between">
              <div class="flex items-center gap-3">
                <div class="text-4xl">{COUNTRY_MAP[$selected]?.flag ?? '🏳️'}</div>
                <div>
                  <div class="text-lg font-semibold">{COUNTRY_MAP[$selected]?.name ?? $selected}</div>
                  <div class="text-sm text-slate-500">{$selected} • {countries[$selected]?.currency}</div>
                </div>
              </div>
              <div>
                <button class="px-3 py-1 rounded-md text-sm bg-slate-100 hover:bg-slate-200" on:click={close}>Fechar</button>
              </div>
            </div>

            <div class="mt-4 space-y-4">
              <div class="bg-slate-50 p-4 rounded border border-slate-100">
                <div class="text-sm text-slate-600">Valores declarados</div>
                <div class="mt-2 grid grid-cols-2 gap-2">
                  <div class="text-xs text-slate-500">Anual</div>
                  <div class="font-medium">{format(countries[$selected]?.annual)} {countries[$selected]?.currency}</div>

                  <div class="text-xs text-slate-500">Mensal</div>
                  <div class="font-medium">{format(countries[$selected]?.monthly)} {countries[$selected]?.currency}</div>

                  <div class="text-xs text-slate-500">Horário</div>
                  <div class="font-medium">{format(countries[$selected]?.hourly)} {countries[$selected]?.currency}</div>
                </div>
              </div>

              <div class="p-4 rounded border border-slate-100">
                <div class="flex items-center justify-between">
                  <div>
                    <div class="text-sm text-slate-600">Comparar com Bitcoin</div>
                    <div class="text-xs text-slate-500">Preço BTC: <strong>${BTC_USD.toLocaleString()} USD</strong></div>
                  </div>
                </div>

                <div class="mt-4 grid grid-cols-1 gap-3">
                  <label class="text-xs text-slate-500">Taxa de câmbio: 1 USD =</label>
                  <div class="flex gap-2">
                    <input
                      id={`rate-input-${$selected}`}
                      bind:value={rateInput}
                      class="flex-1 border rounded px-3 py-2 text-sm"
                      placeholder="Digite quantas unidades da moeda local equivalem a 1 USD"
                      on:input={(e) => {
                        const v = parseFloat((e.target as HTMLInputElement).value);
                        const parsed = isFinite(v) ? v : undefined;
                        exchangeRates.update(r => ({ ...r, [$selected!]: parsed }));
                      }}
                    />
                    <button
                      class="px-3 py-2 bg-slate-800 text-white rounded text-sm"
                      on:click={() => exchangeRates.update(r => ({ ...r, [$selected]: 1 }))}
                      title="Assume 1:1 (useful for USD)"
                    >
                      Paridade 1
                    </button>
                  </div>

                  {#if exchangeRates && exchangeRates[$selected]}
                    <div class="mt-2 text-sm text-slate-700">
                      <div>Anual em BTC:
                        {#if computeBtcEquivalent(countries[$selected]?.annual, exchangeRates[$selected]) !== null}
                          <strong>
                            {computeBtcEquivalent(countries[$selected]?.annual, exchangeRates[$selected])!.toFixed(6)} BTC
                          </strong>
                        {:else}
                          <span class="text-slate-500">Não aplicável</span>
                        {/if}
                      </div>

                      <div class="mt-1">Mensal em BTC:
                        {#if computeBtcEquivalent(countries[$selected]?.monthly, exchangeRates[$selected]) !== null}
                          <strong>
                            {computeBtcEquivalent(countries[$selected]?.monthly, exchangeRates[$selected])!.toFixed(6)} BTC
                          </strong>
                        {:else}
                          <span class="text-slate-500">Não aplicável</span>
                        {/if}
                      </div>

                      <div class="mt-1">Hourly em BTC:
                        {#if computeBtcEquivalent(countries[$selected]?.hourly, exchangeRates[$selected]) !== null}
                          <strong>
                            {computeBtcEquivalent(countries[$selected]?.hourly, exchangeRates[$selected])!.toFixed(6)} BTC
                          </strong>
                        {:else}
                          <span class="text-slate-500">Não aplicável</span>
                        {/if}
                      </div>

                      <div class="mt-3 text-xs text-slate-500">
                        Observação: Sem uma taxa de câmbio, a conversão para BTC não é possível. Insira quantas unidades da moeda local equivalem a 1 USD. Ex: para USD, use 1.
                      </div>
                    </div>
                  {:else}
                    <div class="text-sm text-slate-500">Insira a taxa de câmbio para obter equivalentes em BTC.</div>
                  {/if}
                </div>
              </div>

            </div>

          </aside>
        </div>
      {/if}

    </main>
  </div>
{/if}

<style>
  /* small helpers to mimic shadcn spacing & behavior if needed */
  .shadow-2xl { box-shadow: 0 25px 50px -12px rgba(0,0,0,0.25); }
</style>
