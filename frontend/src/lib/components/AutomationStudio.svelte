<script lang="ts">
  import { onMount } from "svelte";
  import {
    listCampaigns,
    listSchedules,
    createDripBatch,
    cancelSchedule,
    triggerInstantDispatch,
    listPublishingChannels,
    savePublishingChannel,
    testChannelConnection,
    type ContentCampaign,
    type ContentSchedule,
    type PublishingChannel,
  } from "$lib/api";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  // Active view: 'scheduler' | 'queue' | 'channels' | 'n8n'
  let activeTab = $state<"scheduler" | "queue" | "channels" | "n8n">("scheduler");

  // Campaigns & Schedules
  let campaigns = $state<ContentCampaign[]>([]);
  let selectedCampaignId = $state<string>("");
  let schedules = $state<ContentSchedule[]>([]);
  let channels = $state<PublishingChannel[]>([]);

  // Drip Scheduler Form State
  let dripPreset = $state<"aggressive" | "balanced" | "extended">("balanced");
  let startDate = $state<string>(new Date().toISOString().slice(0, 10));
  let startTime = $state<string>("09:00");
  let defaultPlatform = $state<string>("wordpress");
  let schedulingBatch = $state(false);

  // Channels Form State
  let showChannelModal = $state(false);
  let editingChannelId = $state<string>("");
  let channelName = $state("WordPress Blog Utama");
  let channelType = $state<"wordpress" | "ghost" | "telegram" | "n8n_webhook" | "custom_webhook">("wordpress");
  let endpointUrl = $state("https://blog.cecepazhar.com");
  let usernameOrToken = $state("");
  let passwordOrSecret = $state("");
  let chatIdOrConfig = $state("");
  let testingConnection = $state(false);
  let testResultMessage = $state<string | null>(null);

  // Filter state
  let statusFilter = $state<string>("");

  onMount(async () => {
    await refreshData();
  });

  async function refreshData() {
    try {
      campaigns = await listCampaigns(projectId);
      if (campaigns.length > 0 && !selectedCampaignId) {
        selectedCampaignId = campaigns[0].id;
      }
      schedules = await listSchedules(
        selectedCampaignId ? selectedCampaignId : undefined,
        statusFilter || undefined
      );
      channels = await listPublishingChannels();
    } catch (e) {
      console.warn("Gagal refresh automation data:", e);
    }
  }

  async function handleCreateDripSchedule() {
    if (!selectedCampaignId) {
      alert("Pilih kampanye konten terlebih dahulu!");
      return;
    }

    schedulingBatch = true;
    try {
      const startDateTime = new Date(`${startDate}T${startTime}:00`);
      const startUnix = Math.floor(startDateTime.getTime() / 1000);

      await createDripBatch({
        campaign_id: selectedCampaignId,
        preset: dripPreset,
        start_time: startUnix,
        default_platform: defaultPlatform,
      });

      await refreshData();
      activeTab = "queue";
      alert("Jadwal rilis bertahap (Drip Release Cadence) berhasil dibuat!");
    } catch (e: any) {
      alert(`Gagal membuat drip schedule: ${e}`);
    } finally {
      schedulingBatch = false;
    }
  }

  async function handleTriggerNow(id: string) {
    if (!confirm("Dispatch konten ini sekarang juga ke platform target?")) return;
    try {
      await triggerInstantDispatch(id);
      await refreshData();
    } catch (e: any) {
      alert(`Gagal dispatch: ${e}`);
    }
  }

  async function handleCancel(id: string) {
    if (!confirm("Batalkan jadwal rilis ini?")) return;
    try {
      await cancelSchedule(id);
      await refreshData();
    } catch (e: any) {
      alert(`Gagal membatalkan: ${e}`);
    }
  }

  async function handleSaveChannel() {
    try {
      let creds: Record<string, string> = {};
      let config: Record<string, string> = {};

      if (channelType === "wordpress") {
        creds = { username: usernameOrToken, app_password: passwordOrSecret };
      } else if (channelType === "ghost") {
        creds = { admin_api_key: usernameOrToken };
      } else if (channelType === "telegram") {
        creds = { bot_token: usernameOrToken };
        config = { chat_id: chatIdOrConfig };
      } else if (channelType === "custom_webhook") {
        creds = { secret: passwordOrSecret };
      }

      await savePublishingChannel({
        id: editingChannelId || `ch_${Date.now()}`,
        name: channelName,
        channel_type: channelType,
        is_active: true,
        endpoint_url: endpointUrl,
        credentials_json: JSON.stringify(creds),
        config_json: JSON.stringify(config),
        created_at: Math.floor(Date.now() / 1000),
        updated_at: Math.floor(Date.now() / 1000),
      });

      showChannelModal = false;
      await refreshData();
      alert("Konfigurasi kanal publikasi tersimpan!");
    } catch (e: any) {
      alert(`Gagal menyimpan channel: ${e}`);
    }
  }

  async function handleTestConnection() {
    testingConnection = true;
    testResultMessage = null;
    try {
      let creds: Record<string, string> = {};
      let config: Record<string, string> = {};

      if (channelType === "wordpress") {
        creds = { username: usernameOrToken, app_password: passwordOrSecret };
      } else if (channelType === "ghost") {
        creds = { admin_api_key: usernameOrToken };
      } else if (channelType === "telegram") {
        creds = { bot_token: usernameOrToken };
        config = { chat_id: chatIdOrConfig };
      } else if (channelType === "custom_webhook") {
        creds = { secret: passwordOrSecret };
      }

      const res = await testChannelConnection(
        channelType,
        endpointUrl,
        JSON.stringify(creds),
        JSON.stringify(config)
      );

      testResultMessage = `Sukses: ${res.message}`;
    } catch (e: any) {
      testResultMessage = `Gagal: ${e}`;
    } finally {
      testingConnection = false;
    }
  }

  function formatUnix(ts: number): string {
    const d = new Date(ts * 1000);
    return d.toLocaleString("id-ID", {
      day: "2-digit",
      month: "short",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden select-none font-sans">
  <!-- Topbar Header -->
  <header class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-6 h-6 rounded-md bg-cyan-500/20 border border-cyan-500/40 flex items-center justify-center text-xs text-cyan-400">
        🛰️
      </div>
      <h2 class="font-bold text-white text-xs tracking-wide">
        Content Automation, Drip Publishing & Webhook Dispatcher
      </h2>
      <!-- Live Listener Status Indicator -->
      <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-emerald-500/15 text-emerald-400 border border-emerald-500/30 flex items-center gap-1.5">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
        <span>Axum Listener :20130 Active</span>
      </span>
    </div>

    <!-- Navigation Subtabs -->
    <div class="flex items-center gap-1">
      <button
        onclick={() => (activeTab = "scheduler")}
        class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all {activeTab === 'scheduler' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
      >
        📅 Drip Scheduler
      </button>
      <button
        onclick={() => {
          activeTab = "queue";
          refreshData();
        }}
        class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all {activeTab === 'queue' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
      >
        📋 Antrean Queue ({schedules.length})
      </button>
      <button
        onclick={() => (activeTab = "channels")}
        class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all {activeTab === 'channels' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
      >
        🔌 Kanal API ({channels.length})
      </button>
      <button
        onclick={() => (activeTab = "n8n")}
        class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all {activeTab === 'n8n' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
      >
        ⚡ n8n Webhook Hub
      </button>
    </div>
  </header>

  <!-- Main View Area -->
  <div class="flex-1 p-6 overflow-y-auto">
    <!-- Tab 1: Drip Content Scheduler -->
    {#if activeTab === "scheduler"}
      <div class="max-w-4xl mx-auto space-y-6">
        <div class="p-5 bg-[#121217] border border-[#272732] rounded-2xl space-y-4">
          <div class="flex items-center justify-between border-b border-[#272732] pb-3">
            <div>
              <h3 class="font-bold text-white text-sm">Pengaturan Rilis Bertahap (Drip Release Cadence)</h3>
              <p class="text-[11px] text-neutral-400">
                Distribusikan bundle konten secara bertahap untuk memaksimalkan reach algoritma lintas platform.
              </p>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4 text-xs">
            <!-- Campaign Selector -->
            <div>
              <label for="campaign-select" class="block font-medium text-neutral-300 mb-1">Pilih Kampanye Sumber:</label>
              <select
                id="campaign-select"
                bind:value={selectedCampaignId}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white text-xs focus:border-[#8B5CF6] focus:outline-none"
              >
                {#each campaigns as c}
                  <option value={c.id}>{c.title} ({c.assets.length} aset)</option>
                {/each}
              </select>
            </div>

            <!-- Cadence Preset -->
            <div>
              <label for="preset-select" class="block font-medium text-neutral-300 mb-1">Pola Distribusi (Cadence Preset):</label>
              <select
                id="preset-select"
                bind:value={dripPreset}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white text-xs focus:border-[#8B5CF6] focus:outline-none"
              >
                <option value="aggressive">🚀 Aggressive (3-Day Sprint Cadence)</option>
                <option value="balanced">⚖️ Balanced Evergreen (7-Day Cadence - Disarankan)</option>
                <option value="extended">📅 Extended Repurposing (14-Day Staggered)</option>
              </select>
            </div>

            <!-- Start Date -->
            <div>
              <label for="start-date-input" class="block font-medium text-neutral-300 mb-1">Tanggal Mulai (D+0):</label>
              <input
                id="start-date-input"
                type="date"
                bind:value={startDate}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white text-xs focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>

            <!-- Start Time -->
            <div>
              <label for="start-time-input" class="block font-medium text-neutral-300 mb-1">Jam Rilis Utama (WIB):</label>
              <input
                id="start-time-input"
                type="time"
                bind:value={startTime}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white text-xs focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>
          </div>

          <!-- Timeline Preview Cards -->
          <div class="mt-4 p-4 rounded-xl bg-[#0A0A0C] border border-[#272732] space-y-3">
            <span class="text-[10px] uppercase font-bold text-neutral-400 block tracking-wider">
              Simulasi Jadwal Eksekusi ({dripPreset.toUpperCase()}):
            </span>
            <div class="grid grid-cols-5 gap-2 text-center text-[11px]">
              <div class="p-2.5 rounded-lg bg-[#18181F] border border-[#272732] space-y-1">
                <span class="block text-cyan-400 font-bold">D+0 (09:00)</span>
                <span class="block text-white font-medium">📝 Blog Teknis</span>
                <span class="text-[10px] text-neutral-500">WordPress / Ghost</span>
              </div>
              <div class="p-2.5 rounded-lg bg-[#18181F] border border-[#272732] space-y-1">
                <span class="block text-cyan-400 font-bold">D+0 (19:30)</span>
                <span class="block text-white font-medium">🐦 Utas X/Threads</span>
                <span class="text-[10px] text-neutral-500">n8n Webhook</span>
              </div>
              <div class="p-2.5 rounded-lg bg-[#18181F] border border-[#272732] space-y-1">
                <span class="block text-cyan-400 font-bold">D+1 (12:00)</span>
                <span class="block text-white font-medium">⚡ Viral Short</span>
                <span class="text-[10px] text-neutral-500">n8n / Reels</span>
              </div>
              <div class="p-2.5 rounded-lg bg-[#18181F] border border-[#272732] space-y-1">
                <span class="block text-cyan-400 font-bold">{dripPreset === 'aggressive' ? 'D+1 (18:00)' : 'D+2 (17:00)'}</span>
                <span class="block text-white font-medium">📱 Carousel IG</span>
                <span class="text-[10px] text-neutral-500">n8n / Carousel</span>
              </div>
              <div class="p-2.5 rounded-lg bg-[#18181F] border border-[#272732] space-y-1">
                <span class="block text-cyan-400 font-bold">{dripPreset === 'aggressive' ? 'D+2 (08:00)' : 'D+3 (08:00)'}</span>
                <span class="block text-white font-medium">✉️ Newsletter</span>
                <span class="text-[10px] text-neutral-500">Telegram / Email</span>
              </div>
            </div>
          </div>

          <button
            onclick={handleCreateDripSchedule}
            disabled={schedulingBatch || !selectedCampaignId}
            class="w-full py-3 rounded-xl bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white text-xs font-bold shadow-lg shadow-emerald-600/30 transition-all flex items-center justify-center gap-2"
          >
            <span>📅</span>
            <span>{schedulingBatch ? "Menjadwalkan Drip Release..." : "Buat Jadwal Drip Cadence Otomatis"}</span>
          </button>
        </div>
      </div>

    <!-- Tab 2: Queue Monitor -->
    {:else if activeTab === "queue"}
      <div class="max-w-5xl mx-auto space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <span class="text-xs font-medium text-neutral-400">Filter Status:</span>
            <select
              bind:value={statusFilter}
              onchange={refreshData}
              class="bg-[#18181F] border border-[#272732] text-xs text-white rounded px-2.5 py-1 focus:outline-none"
            >
              <option value="">Semua Status</option>
              <option value="queued">Queued (Menunggu)</option>
              <option value="dispatching">Dispatching</option>
              <option value="published">Published</option>
              <option value="failed">Failed (Gagal)</option>
            </select>
          </div>
          <button
            onclick={refreshData}
            class="px-3 py-1 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-xs text-white rounded transition-colors"
          >
            🔄 Refresh Antrean
          </button>
        </div>

        {#if schedules.length === 0}
          <div class="p-12 text-center text-neutral-500 bg-[#121217] rounded-xl border border-[#272732]">
            Belum ada jadwal yang terdaftar. Buat jadwal di tab Drip Scheduler.
          </div>
        {:else}
          <div class="bg-[#121217] border border-[#272732] rounded-xl overflow-hidden text-xs">
            <table class="w-full text-left">
              <thead class="bg-[#18181F] text-neutral-400 uppercase font-mono text-[10px] border-b border-[#272732]">
                <tr>
                  <th class="p-3">Waktu Rilis</th>
                  <th class="p-3">Kanal</th>
                  <th class="p-3">Platform</th>
                  <th class="p-3">Status</th>
                  <th class="p-3">Tracking / URL</th>
                  <th class="p-3 text-right">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-[#272732]">
                {#each schedules as s}
                  <tr class="hover:bg-[#18181F]/40 transition-colors">
                    <td class="p-3 font-mono text-neutral-300">
                      {formatUnix(s.scheduled_at)}
                    </td>
                    <td class="p-3 uppercase font-bold text-white">
                      {s.channel}
                    </td>
                    <td class="p-3 font-mono text-cyan-400">
                      {s.target_platform}
                    </td>
                    <td class="p-3">
                      <span class="px-2 py-0.5 rounded text-[10px] font-mono font-bold uppercase {s.status === 'published' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : s.status === 'dispatching' ? 'bg-amber-500/20 text-amber-400' : s.status === 'failed' ? 'bg-red-500/20 text-red-400' : 'bg-blue-500/20 text-blue-400'}">
                        {s.status}
                        {#if s.retry_count > 0 && s.status !== 'published'}
                          ({s.retry_count}/{s.max_retries})
                        {/if}
                      </span>
                    </td>
                    <td class="p-3 font-mono text-[10px] text-neutral-400 truncate max-w-xs">
                      {#if s.published_url}
                        <a href={s.published_url} target="_blank" class="text-cyan-400 hover:underline">
                          🔗 {s.published_url}
                        </a>
                      {:else if s.last_error}
                        <span class="text-red-400" title={s.last_error}>⚠️ {s.last_error}</span>
                      {:else}
                        <span>{s.tracking_token}</span>
                      {/if}
                    </td>
                    <td class="p-3 text-right space-x-1">
                      {#if s.status === 'queued' || s.status === 'failed'}
                        <button
                          onclick={() => handleTriggerNow(s.id)}
                          class="px-2 py-1 bg-cyan-600 hover:bg-cyan-500 text-white rounded text-[10px] font-bold"
                          title="Kirim sekarang juga tanpa menunggu timer"
                        >
                          ⚡ Push
                        </button>
                        <button
                          onclick={() => handleCancel(s.id)}
                          class="px-2 py-1 bg-neutral-800 hover:bg-red-900/50 text-neutral-300 hover:text-red-300 rounded text-[10px]"
                          title="Batalkan jadwal"
                        >
                          ✕
                        </button>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

    <!-- Tab 3: Publishing Channels Management -->
    {:else if activeTab === "channels"}
      <div class="max-w-4xl mx-auto space-y-4">
        <div class="flex items-center justify-between">
          <h3 class="font-bold text-white text-sm">Kanal Destinasi Publikasi Terintegrasi</h3>
          <button
            onclick={() => {
              editingChannelId = "";
              channelName = "WordPress Baru";
              channelType = "wordpress";
              endpointUrl = "https://example.com";
              usernameOrToken = "";
              passwordOrSecret = "";
              chatIdOrConfig = "";
              showChannelModal = true;
            }}
            class="px-3 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold rounded-lg"
          >
            + Tambah Kanal API
          </button>
        </div>

        <div class="grid grid-cols-2 gap-4">
          {#each channels as ch}
            <div class="p-4 bg-[#121217] border border-[#272732] rounded-xl space-y-2">
              <div class="flex items-center justify-between">
                <span class="font-bold text-white text-xs">{ch.name}</span>
                <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-cyan-500/20 text-cyan-400 uppercase">
                  {ch.channel_type}
                </span>
              </div>
              <p class="text-[11px] text-neutral-400 font-mono truncate">{ch.endpoint_url || 'N/A'}</p>
              <div class="pt-2 border-t border-[#272732] flex items-center justify-between text-xs">
                <span class="text-[10px] text-emerald-400">● Terhubung</span>
                <button
                  onclick={() => {
                    editingChannelId = ch.id;
                    channelName = ch.name;
                    channelType = ch.channel_type as any;
                    endpointUrl = ch.endpoint_url || "";
                    showChannelModal = true;
                  }}
                  class="text-[#8B5CF6] hover:underline text-[11px]"
                >
                  Edit Konfigurasi
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>

    <!-- Tab 4: Bidirectional n8n Hub -->
    {:else if activeTab === "n8n"}
      <div class="max-w-4xl mx-auto space-y-6">
        <div class="p-6 bg-[#121217] border border-[#272732] rounded-2xl space-y-4">
          <div class="flex items-center justify-between border-b border-[#272732] pb-3">
            <div>
              <h3 class="font-bold text-white text-sm">Hub Integrasi Dua Arah n8n (Bidirectional Workflow)</h3>
              <p class="text-[11px] text-neutral-400">
                Picu eksekusi multi-platform di n8n dan terima status konfirmasi langsung ke CAStudio.
              </p>
            </div>
            <span class="px-2.5 py-1 rounded text-xs font-mono font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
              ● Listener Online :20130
            </span>
          </div>

          <div class="space-y-3 text-xs">
            <div>
              <span class="font-bold text-[#A78BFA] block mb-1">1. Outbound Webhook Trigger (CAStudio ➔ n8n):</span>
              <p class="text-neutral-400 mb-2">
                CAStudio mengirim payload JSON terstandarisasi ke webhook trigger URL n8n Anda saat jadwal tercapai:
              </p>
              <pre class="p-3 bg-[#0A0A0C] border border-[#272732] rounded-lg font-mono text-[10px] text-neutral-300 overflow-x-auto">{JSON.stringify({
  campaign_id: "cmp_7f8a9...",
  channel: "tweets",
  title: "Peluncuran Sovereign Developer",
  raw_content: "1/5 Arsitektur Sovereign Developer...",
  media_urls: [],
  scheduled_at: 1728412800,
  tracking_token: "trk_01J9X8K...",
  callback_url: "http://127.0.0.1:20130/api/v1/webhook/n8n-status"
}, null, 2)}</pre>
            </div>

            <div class="pt-3 border-t border-[#272732]">
              <span class="font-bold text-[#A78BFA] block mb-1">2. Inbound Callback Endpoint (n8n ➔ CAStudio):</span>
              <p class="text-neutral-400 mb-1">
                Kirim HTTP POST dari n8n HTTP Request Node di akhir workflow Anda ke:
              </p>
              <div class="p-2 bg-[#0A0A0C] border border-[#272732] rounded font-mono text-cyan-400 text-xs">
                POST http://127.0.0.1:20130/api/v1/webhook/n8n-status
              </div>
              <p class="text-[11px] text-neutral-500 mt-1">
                Body format: <code>{"{ tracking_token, status: 'COMPLETED', live_url: 'https://...' }"}</code>
              </p>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Modal Edit / Add Channel -->
{#if showChannelModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-xs">
    <div class="w-full max-w-md bg-[#121217] border border-[#272732] rounded-2xl p-6 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Konfigurasi Kanal Publikasi</h3>
        <button onclick={() => (showChannelModal = false)} class="text-neutral-400 hover:text-white">✕</button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label for="ch-name-input" class="block font-medium text-neutral-300 mb-1">Nama Kanal:</label>
          <input
            id="ch-name-input"
            type="text"
            bind:value={channelName}
            class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
          />
        </div>

        <div>
          <label for="ch-type-select" class="block font-medium text-neutral-300 mb-1">Tipe Platform:</label>
          <select
            id="ch-type-select"
            bind:value={channelType}
            class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
          >
            <option value="wordpress">WordPress REST API</option>
            <option value="ghost">Ghost Admin API</option>
            <option value="telegram">Telegram Bot API</option>
            <option value="n8n_webhook">n8n Webhook Trigger</option>
            <option value="custom_webhook">Custom Webhook POST</option>
          </select>
        </div>

        <div>
          <label for="ch-url-input" class="block font-medium text-neutral-300 mb-1">Endpoint URL / Webhook URL:</label>
          <input
            id="ch-url-input"
            type="text"
            bind:value={endpointUrl}
            placeholder="https://..."
            class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
          />
        </div>

        {#if channelType === "wordpress"}
          <div>
            <label for="wp-username-input" class="block font-medium text-neutral-300 mb-1">WordPress Username:</label>
            <input
              id="wp-username-input"
              type="text"
              bind:value={usernameOrToken}
              class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
            />
          </div>
          <div>
            <label for="wp-password-input" class="block font-medium text-neutral-300 mb-1">Application Password:</label>
            <input
              id="wp-password-input"
              type="password"
              bind:value={passwordOrSecret}
              class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
            />
          </div>
        {:else if channelType === "telegram"}
          <div>
            <label for="tg-token-input" class="block font-medium text-neutral-300 mb-1">Bot Token:</label>
            <input
              id="tg-token-input"
              type="password"
              bind:value={usernameOrToken}
              placeholder="123456:ABC-DEF..."
              class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
            />
          </div>
          <div>
            <label for="tg-chat-input" class="block font-medium text-neutral-300 mb-1">Chat ID / Channel Username:</label>
            <input
              id="tg-chat-input"
              type="text"
              bind:value={chatIdOrConfig}
              placeholder="-100123456789 atau @channelname"
              class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white focus:outline-none"
            />
          </div>
        {/if}

        {#if testResultMessage}
          <div class="p-2 rounded bg-[#0A0A0C] border border-[#272732] text-[11px] {testResultMessage.startsWith('Sukses') ? 'text-emerald-400' : 'text-red-400'}">
            {testResultMessage}
          </div>
        {/if}
      </div>

      <div class="pt-3 border-t border-[#272732] flex items-center justify-between">
        <button
          onclick={handleTestConnection}
          disabled={testingConnection}
          class="px-3 py-1.5 bg-[#18181F] hover:bg-[#272732] text-xs text-neutral-300 rounded border border-[#272732]"
        >
          {testingConnection ? "Menguji..." : "⚡ Test Ping"}
        </button>
        <div class="flex items-center gap-2">
          <button onclick={() => (showChannelModal = false)} class="px-3 py-1.5 text-xs text-neutral-400">Batal</button>
          <button onclick={handleSaveChannel} class="px-4 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold rounded">
            Simpan
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
