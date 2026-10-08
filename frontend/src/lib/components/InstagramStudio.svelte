<script lang="ts">
  import { generateCarousel, generateReels, generateAi } from "$lib/api";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  let activeTab = $state<"carousel" | "reels" | "caption">("carousel");

  // Carousel State
  let topic = $state("7 AI Agent Otomatis untuk Software Engineer");
  let slidesCount = $state(7);
  let currentSlideIndex = $state(0);
  let loading = $state(false);

  let slides = $state([
    {
      title: "7 AI AGENT OTOMATIS",
      subtitle: "Bekerja Mandiri di Laptop Tanpa Bergantung Cloud",
      body: "Bagaimana developer modern membangun armada coding otonom yang berjalan 24/7 di mesin lokal.",
      type: "hook",
    },
    {
      title: "01. ARCHITECTURE AGENT",
      subtitle: "Menyusun PRD & Task Otomatis",
      body: "Agent membaca ide kasar lalu menghasilkan spesifikasi teknis lengkap dengan kriteria acceptance yang ketat.",
      type: "content",
    },
    {
      title: "02. TEST-DRIVEN CODER",
      subtitle: "Menulis Tes Sebelum Implementasi",
      body: "Mencegah kode halusinasi dengan memastikan semua tes hijau sebelum melakukan commit.",
      type: "content",
    },
    {
      title: "03. READ-ONLY QA AUDITOR",
      subtitle: "Pemeriksa Independen Tanpa Kompromi",
      body: "Memverifikasi perintah nyata dan menolak klaim selesai tanpa bukti output terminal asli.",
      type: "content",
    },
    {
      title: "04. CRON WATCHDOG",
      subtitle: "Monitoring & Auto-Commit Latar Belakang",
      body: "Menjaga repositori tetap tersinkron dan mendeteksi kondisi kotor secara otomatis.",
      type: "content",
    },
    {
      title: "05. ASSET & DEPLOY BOT",
      subtitle: "Otomasi Build & Rilis Mandiri",
      body: "Men-scaffold paket, membundel frontend, dan merilis biner ke server Coolify dalam hitungan menit.",
      type: "content",
    },
    {
      title: "SIMPAN & PRAKTIKKAN!",
      subtitle: "Kedaulatan Penuh di Mesin Lokal Anda",
      body: "Ketik 'AGENT' di kolom komentar untuk mendapatkan master template prompt eksekusi lengkap.\n\nFollow @cecepazhar untuk tips sovereign developer harian.",
      type: "cta",
    },
  ]);

  // Reels State
  let reelsTopic = $state("Kenapa Developer Butuh AI Agent Mandiri");
  let reelsScript = $state<string | null>(null);

  // Caption State
  let rawCaption = $state("");
  let cleanCaption = $state("");
  let hashtags = $state("#softwareengineer #developerindonesia #coding #aiagent #rustlang #indiehacker #webdev");

  async function handleGenerateCarousel() {
    loading = true;
    try {
      const res = await generateCarousel(projectId, topic, slidesCount);
      // Parse or set to body
      slides[0].title = topic.toUpperCase();
      slides[0].body = res.slice(0, 200);
      alert("Draf Carousel berhasil di-generate AI!");
    } catch (e: any) {
      alert(`Gagal generate: ${e}`);
    } finally {
      loading = false;
    }
  }

  async function handleGenerateReels() {
    loading = true;
    try {
      const res = await generateReels(projectId, reelsTopic);
      reelsScript = res;
    } catch (e: any) {
      alert(`Gagal generate reels: ${e}`);
    } finally {
      loading = false;
    }
  }

  function downloadSlideAsPng(index: number) {
    const canvas = document.createElement("canvas");
    canvas.width = 1080;
    canvas.height = 1350; // 4:5 IG Ratio
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Dark Obsidian background
    ctx.fillStyle = "#0A0A0C";
    ctx.fillRect(0, 0, 1080, 1350);

    // Outer glow & border
    ctx.strokeStyle = "#8B5CF6";
    ctx.lineWidth = 12;
    ctx.strokeRect(40, 40, 1000, 1270);

    // Header indicator (Slide number)
    ctx.fillStyle = "#9CA3AF";
    ctx.font = "bold 32px sans-serif";
    ctx.fillText(`SLIDE ${index + 1} / ${slides.length}`, 80, 120);

    // Title
    ctx.fillStyle = "#FFFFFF";
    ctx.font = "bold 64px sans-serif";
    const s = slides[index];
    wrapText(ctx, s.title, 80, 320, 920, 80);

    // Subtitle
    ctx.fillStyle = "#A78BFA";
    ctx.font = "bold 40px sans-serif";
    wrapText(ctx, s.subtitle, 80, 480, 920, 56);

    // Body text
    ctx.fillStyle = "#D1D5DB";
    ctx.font = "36px sans-serif";
    wrapText(ctx, s.body, 80, 680, 920, 52);

    // Footer brand
    ctx.fillStyle = "#6B7280";
    ctx.font = "28px sans-serif";
    ctx.fillText("@cecepazhar • Fathforce Sovereign Tech", 80, 1220);

    // Download
    const a = document.createElement("a");
    a.download = `slide_${index + 1}.png`;
    a.href = canvas.toDataURL("image/png");
    a.click();
  }

  function wrapText(ctx: CanvasRenderingContext2D, text: string, x: number, y: number, maxWidth: number, lineHeight: number) {
    const words = text.split(" ");
    let line = "";
    let curY = y;
    for (let n = 0; n < words.length; n++) {
      const testLine = line + words[n] + " ";
      const metrics = ctx.measureText(testLine);
      if (metrics.width > maxWidth && n > 0) {
        ctx.fillText(line, x, curY);
        line = words[n] + " ";
        curY += lineHeight;
      } else {
        line = testLine;
      }
    }
    ctx.fillText(line, x, curY);
  }

  function formatCleanCaption() {
    // Replace double newlines with zero-width clean lines
    cleanCaption = rawCaption
      .split("\n")
      .map((l) => (l.trim() === "" ? "⠀" : l))
      .join("\n");
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Sub Header Tabs -->
  <div class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-1 bg-[#18181F] p-0.5 rounded-lg border border-[#272732] text-xs">
      <button
        onclick={() => (activeTab = "carousel")}
        class="px-3 py-1 rounded-md font-medium transition-all {activeTab === 'carousel' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
      >
        📸 IG Carousel (4:5)
      </button>
      <button
        onclick={() => (activeTab = "reels")}
        class="px-3 py-1 rounded-md font-medium transition-all {activeTab === 'reels' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
      >
        🎬 Reels Script (9:16)
      </button>
      <button
        onclick={() => (activeTab = "caption")}
        class="px-3 py-1 rounded-md font-medium transition-all {activeTab === 'caption' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
      >
        ✍️ Caption & Hashtag
      </button>
    </div>

    <span class="text-xs text-[#9CA3AF]">Instagram Suite (1080×1350 & 1080×1920)</span>
  </div>

  <!-- Content Area -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full">
    {#if activeTab === "carousel"}
      <!-- Carousel Builder Section -->
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
        <!-- Left: Carousel Editor & AI controls -->
        <div class="lg:col-span-6 space-y-5">
          <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
            <h3 class="font-bold text-white text-sm flex items-center gap-2">
              <span>✨ AI Generator Carousel</span>
            </h3>
            <div>
              <label for="carousel-topic-input" class="block text-xs text-[#9CA3AF] mb-1">Topik Utama</label>
              <input
                id="carousel-topic-input"
                type="text"
                bind:value={topic}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs text-[#9CA3AF]">Jumlah Slide: {slidesCount}</span>
              <button
                onclick={handleGenerateCarousel}
                disabled={loading}
                class="px-4 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-md shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
              >
                {loading ? "Menyusun..." : "Generate dengan AI"}
              </button>
            </div>
          </div>

          <!-- Slide Content Inspector -->
          <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
            <div class="flex items-center justify-between">
              <h4 class="font-bold text-white text-xs">Edit Slide #{currentSlideIndex + 1}</h4>
              <button
                onclick={() => downloadSlideAsPng(currentSlideIndex)}
                class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded text-xs font-semibold transition-all shadow-sm"
              >
                Unduh Slide #{currentSlideIndex + 1} (PNG 1080×1350)
              </button>
            </div>

            <div>
              <label for="slide-title-input" class="block text-[11px] text-[#9CA3AF] mb-1">Judul Utama</label>
              <input
                id="slide-title-input"
                type="text"
                bind:value={slides[currentSlideIndex].title}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-1.5 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>

            <div>
              <label for="slide-subtitle-input" class="block text-[11px] text-[#9CA3AF] mb-1">Sub Judul</label>
              <input
                id="slide-subtitle-input"
                type="text"
                bind:value={slides[currentSlideIndex].subtitle}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-1.5 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>

            <div>
              <label for="slide-body-input" class="block text-[11px] text-[#9CA3AF] mb-1">Isi Pesan</label>
              <textarea
                id="slide-body-input"
                bind:value={slides[currentSlideIndex].body}
                rows="4"
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
              ></textarea>
            </div>
          </div>
        </div>

        <!-- Right: Live Canvas Preview (4:5 Ratio) -->
        <div class="lg:col-span-6 flex flex-col items-center">
          <div class="text-xs text-[#9CA3AF] mb-2 font-mono">Pratinjau Asli Rasio 4:5 (Instagram Feed)</div>
          
          <!-- Mock Phone/Frame Card -->
          <div class="w-[340px] h-[425px] bg-[#0A0A0C] border-4 border-[#8B5CF6] rounded-2xl p-6 flex flex-col justify-between shadow-2xl relative overflow-hidden select-none">
            <!-- Top bar indicator -->
            <div class="flex items-center justify-between text-[11px] text-[#9CA3AF] font-bold">
              <span>SLIDE {currentSlideIndex + 1} / {slides.length}</span>
              <span class="text-[#8B5CF6]">@cecepazhar</span>
            </div>

            <!-- Content Area -->
            <div class="my-auto space-y-3">
              <h2 class="text-xl font-extrabold text-white leading-tight tracking-tight">
                {slides[currentSlideIndex].title}
              </h2>
              <h4 class="text-xs font-bold text-[#A78BFA]">
                {slides[currentSlideIndex].subtitle}
              </h4>
              <p class="text-[12px] text-[#D1D5DB] leading-relaxed">
                {slides[currentSlideIndex].body}
              </p>
            </div>

            <!-- Footer Swipe Indicator -->
            <div class="border-t border-[#272732] pt-3 flex items-center justify-between text-[10px] text-[#6B7280]">
              <span>Fathforce Sovereign Studio</span>
              <span class="text-[#8B5CF6] font-bold">Geser Ke Kiri ➔</span>
            </div>
          </div>

          <!-- Slide Thumbnails Strip -->
          <div class="flex items-center gap-2 mt-4 overflow-x-auto max-w-full pb-2">
            {#each slides as _, i}
              <button
                onclick={() => (currentSlideIndex = i)}
                class="w-8 h-10 rounded border text-xs font-bold transition-all flex items-center justify-center {currentSlideIndex === i ? 'border-[#8B5CF6] bg-[#8B5CF6]/30 text-white shadow-md' : 'border-[#272732] bg-[#18181F] text-[#9CA3AF] hover:text-white'}"
              >
                {i + 1}
              </button>
            {/each}
          </div>
        </div>
      </div>
    {:else if activeTab === "reels"}
      <!-- Reels Script Section -->
      <div class="max-w-2xl mx-auto space-y-5">
        <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
          <h3 class="font-bold text-white text-sm">Naskah Video Reels / TikTok (9:16)</h3>
          <div>
            <label for="reels-topic-input" class="block text-xs text-[#9CA3AF] mb-1">Topik Video</label>
            <input
              id="reels-topic-input"
              type="text"
              bind:value={reelsTopic}
              class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
            />
          </div>
          <div class="text-right">
            <button
              onclick={handleGenerateReels}
              disabled={loading}
              class="px-4 py-2 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-md shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
            >
              {loading ? "Menulis Naskah..." : "Generate Naskah Video 60s"}
            </button>
          </div>
        </div>

        {#if reelsScript}
          <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 font-mono text-xs text-[#D1D5DB] whitespace-pre-wrap leading-relaxed">
            {reelsScript}
          </div>
        {/if}
      </div>
    {:else}
      <!-- Caption & Hashtag Section -->
      <div class="max-w-2xl mx-auto space-y-5">
        <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
          <h3 class="font-bold text-white text-sm">Formatter Caption Instagram Anti-Berantakan</h3>
          <p class="text-xs text-[#9CA3AF]">
            Menghilangkan masalah baris kosong yang hilang saat diposting di Instagram dengan zero-width character rapi.
          </p>

          <div>
            <label for="raw-caption-input" class="block text-xs text-[#9CA3AF] mb-1">Ketik / Tempel Caption Mentah</label>
            <textarea
              id="raw-caption-input"
              bind:value={rawCaption}
              oninput={formatCleanCaption}
              rows="6"
              placeholder="Tulis caption dengan enter kosong..."
              class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
            ></textarea>
          </div>

          <div>
            <label for="hashtags-input" class="block text-xs text-[#9CA3AF] mb-1">Kluster Hashtag</label>
            <input
              id="hashtags-input"
              type="text"
              bind:value={hashtags}
              class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-[#A78BFA] focus:border-[#8B5CF6] focus:outline-none"
            />
          </div>

          {#if cleanCaption}
            <div class="border-t border-[#272732] pt-4 space-y-2">
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-[#10B981]">Hasil Siap Salin ke Instagram:</span>
                <button
                  onclick={() => {
                    navigator.clipboard.writeText(`${cleanCaption}\n\n${hashtags}`);
                    alert("Caption tersalin ke clipboard!");
                  }}
                  class="px-3 py-1 bg-[#8B5CF6] text-white rounded text-xs font-semibold"
                >
                  Salin ke Clipboard
                </button>
              </div>
              <div class="bg-[#050507] p-4 rounded-lg text-xs text-white whitespace-pre-wrap font-sans border border-[#272732]">
                {cleanCaption}

                <span class="text-[#A78BFA]">{hashtags}</span>
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>
