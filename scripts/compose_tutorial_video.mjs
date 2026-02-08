#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { access, cp, mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";

function parseArgs(argv) {
  let outDir = process.env.TUTORIAL_VIDEO_OUT_DIR || "artifacts/tutorial-video";
  let voiceProvider = null;
  let strictVoice = envBool("TUTORIAL_VOICE_STRICT", false);
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === "--out-dir" && argv[i + 1]) {
      outDir = argv[i + 1];
      i += 1;
      continue;
    }
    if (arg === "--voice-provider" && argv[i + 1]) {
      voiceProvider = normalizeProviderName(argv[i + 1]);
      i += 1;
      continue;
    }
    if (arg === "--strict-voice") {
      strictVoice = true;
      continue;
    }
    if (arg === "--no-strict-voice") {
      strictVoice = false;
    }
  }
  return { outDir, voiceProvider, strictVoice };
}

function run(cmd, args, options = {}) {
  const result = spawnSync(cmd, args, {
    cwd: options.cwd,
    stdio: ["ignore", "pipe", "pipe"],
    encoding: "utf8",
  });
  if (result.status !== 0) {
    const message = [
      `${cmd} ${args.join(" ")}`,
      result.stdout?.trim(),
      result.stderr?.trim(),
    ]
      .filter(Boolean)
      .join("\n");
    throw new Error(`Command failed:\n${message}`);
  }
  return result.stdout.trim();
}

function tryRun(cmd, args, options = {}) {
  const result = spawnSync(cmd, args, {
    cwd: options.cwd,
    stdio: ["ignore", "pipe", "pipe"],
    encoding: "utf8",
  });
  return {
    ok: result.status === 0,
    stdout: result.stdout?.trim() || "",
    stderr: result.stderr?.trim() || "",
  };
}

function hasCommand(cmd) {
  const probe = spawnSync("which", [cmd], { stdio: "ignore" });
  return probe.status === 0;
}

function clamp(value, min, max) {
  return Math.max(min, Math.min(max, value));
}

function envFloat(name, fallback) {
  const raw = process.env[name];
  if (raw === undefined) return fallback;
  const parsed = Number.parseFloat(raw);
  return Number.isFinite(parsed) ? parsed : fallback;
}

function envInt(name, fallback) {
  const raw = process.env[name];
  if (raw === undefined) return fallback;
  const parsed = Number.parseInt(raw, 10);
  return Number.isInteger(parsed) ? parsed : fallback;
}

function envBool(name, fallback) {
  const raw = process.env[name];
  if (raw === undefined) return fallback;
  return ["1", "true", "yes", "on"].includes(raw.toLowerCase());
}

function normalizeCaption(text) {
  return String(text || "").replace(/\s+/g, " ").trim();
}

function normalizeCueText(text) {
  return normalizeCaption(text).replace(/\s+([,.;:!?])/g, "$1");
}

async function loadDotEnv(filePath) {
  let raw = "";
  try {
    raw = await readFile(filePath, "utf8");
  } catch (_err) {
    return false;
  }

  for (const line of raw.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;

    const eq = trimmed.indexOf("=");
    if (eq <= 0) continue;

    const key = trimmed.slice(0, eq).trim();
    if (!key || process.env[key] !== undefined) continue;

    let value = trimmed.slice(eq + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    process.env[key] = value;
  }
  return true;
}

function toSrtTimestamp(seconds) {
  const safe = Math.max(0, seconds);
  const totalMs = Math.round(safe * 1000);
  const ms = totalMs % 1000;
  const totalSec = Math.floor(totalMs / 1000);
  const sec = totalSec % 60;
  const totalMin = Math.floor(totalSec / 60);
  const min = totalMin % 60;
  const hour = Math.floor(totalMin / 60);
  return `${String(hour).padStart(2, "0")}:${String(min).padStart(2, "0")}:${String(sec).padStart(
    2,
    "0",
  )},${String(ms).padStart(3, "0")}`;
}

function durationOf(file) {
  const out = run("ffprobe", [
    "-v",
    "error",
    "-show_entries",
    "format=duration",
    "-of",
    "default=nokey=1:noprint_wrappers=1",
    file,
  ]);
  const value = Number.parseFloat(out);
  if (!Number.isFinite(value) || value <= 0) {
    throw new Error(`Could not detect duration for ${file}`);
  }
  return value;
}

async function fileExists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch (_err) {
    return false;
  }
}

function elevenLabsSettings() {
  return {
    voice_id: process.env.TUTORIAL_ELEVENLABS_VOICE_ID || "pqHfZKP75CvOlQylNhV4",
    model_id: process.env.TUTORIAL_ELEVENLABS_MODEL_ID || "eleven_turbo_v2_5",
    output_format: process.env.TUTORIAL_ELEVENLABS_OUTPUT_FORMAT || "mp3_44100_128",
    endpoint: process.env.TUTORIAL_ELEVENLABS_ENDPOINT || "https://api.elevenlabs.io",
    stability: clamp(envFloat("TUTORIAL_ELEVENLABS_STABILITY", 0.4), 0, 1),
    similarity_boost: clamp(envFloat("TUTORIAL_ELEVENLABS_SIMILARITY", 0.7), 0, 1),
    style: clamp(envFloat("TUTORIAL_ELEVENLABS_STYLE", 0.25), 0, 1),
    use_speaker_boost: envBool("TUTORIAL_ELEVENLABS_SPEAKER_BOOST", true),
    speed: clamp(envFloat("TUTORIAL_ELEVENLABS_SPEED", 1.0), 0.7, 1.2),
  };
}

function voiceCacheSettings(provider) {
  if (provider === "elevenlabs") {
    return elevenLabsSettings();
  }
  if (provider === "edge") {
    return {
      voice: process.env.TUTORIAL_VOICE_NAME || "en-US-JennyNeural",
      rate: process.env.TUTORIAL_VOICE_RATE || "+0%",
      pitch: process.env.TUTORIAL_VOICE_PITCH || "+0Hz",
      volume: process.env.TUTORIAL_VOICE_VOLUME || "+0%",
    };
  }
  if (provider === "say") {
    return {
      voice: process.env.TUTORIAL_VOICE_NAME || "Samantha",
      rate_wpm: process.env.TUTORIAL_VOICE_RATE_WPM || "168",
    };
  }
  if (provider === "espeak-ng" || provider === "espeak") {
    return {
      voice: process.env.TUTORIAL_VOICE_NAME || "en-us+f3",
      rate_wpm: process.env.TUTORIAL_VOICE_RATE_WPM || "160",
    };
  }
  return {};
}

function voiceCacheKey(provider, text) {
  const descriptor = {
    provider,
    text: normalizeCaption(text),
    settings: voiceCacheSettings(provider),
  };
  return createHash("sha256").update(JSON.stringify(descriptor)).digest("hex");
}

function sceneVoiceDuration(timeline, index, fade) {
  const segment = timeline[index];
  if (!segment) return 0.1;
  if (index >= timeline.length - 1) return segment.duration;
  return Math.max(0.1, segment.duration - fade);
}

function timelineFor(durations, fade) {
  const segments = [];
  let cursor = 0;
  for (let i = 0; i < durations.length; i += 1) {
    const start = cursor;
    const end = start + durations[i];
    segments.push({ index: i, start, end, duration: durations[i] });
    cursor = end - (i < durations.length - 1 ? fade : 0);
  }
  return segments;
}

function transitionSequence(count) {
  const defaults = [
    "fade",
    "smoothleft",
    "fadeblack",
    "wipeleft",
    "circleopen",
    "smoothup",
    "slideright",
    "fade",
  ];
  const configured = String(process.env.TUTORIAL_SCENE_TRANSITIONS || "")
    .split(",")
    .map((value) => value.trim())
    .filter(Boolean);
  const source = configured.length > 0 ? configured : defaults;
  const out = [];
  for (let i = 0; i < Math.max(0, count - 1); i += 1) {
    out.push(source[i % source.length]);
  }
  return out;
}

function normalizeProviderName(value) {
  const lowered = String(value || "").toLowerCase();
  if (lowered === "11labs") return "elevenlabs";
  return lowered;
}

function availableVoiceProviders() {
  const elevenlabsKey = process.env.ELEVENLABS_API_KEY || process.env.TUTORIAL_ELEVENLABS_API_KEY;
  return {
    elevenlabs: Boolean(elevenlabsKey),
    edge: hasCommand("edge-tts"),
    say: hasCommand("say"),
    espeakng: hasCommand("espeak-ng"),
    espeak: hasCommand("espeak"),
  };
}

function pickVoiceProviders(preferredOverride = null) {
  const preferred = normalizeProviderName(preferredOverride || process.env.TUTORIAL_VOICE_PROVIDER || "auto");
  const avail = availableVoiceProviders();
  const byName = {
    elevenlabs: avail.elevenlabs,
    edge: avail.edge,
    say: avail.say,
    "espeak-ng": avail.espeakng,
    espeak: avail.espeak,
  };

  const autoOrder = [];
  if (avail.elevenlabs) autoOrder.push("elevenlabs");
  if (avail.edge) autoOrder.push("edge");
  if (avail.say) autoOrder.push("say");
  if (avail.espeakng) autoOrder.push("espeak-ng");
  if (avail.espeak) autoOrder.push("espeak");

  if (preferred === "none") {
    return { preferred, candidates: ["none"], available: avail };
  }

  if (preferred !== "auto") {
    const ordered = [];
    if (byName[preferred]) {
      ordered.push(preferred);
    }
    for (const provider of autoOrder) {
      if (provider !== preferred) {
        ordered.push(provider);
      }
    }
    if (ordered.length === 0) ordered.push("none");
    return { preferred, candidates: ordered, available: avail };
  }

  if (autoOrder.length === 0) autoOrder.push("none");
  return { preferred, candidates: autoOrder, available: avail };
}

function firstNumber(value, fallback) {
  const parsed = Number.parseFloat(String(value));
  return Number.isFinite(parsed) ? parsed : fallback;
}

function alignmentFromPayload(raw) {
  if (!raw || !Array.isArray(raw.characters) || raw.characters.length === 0) {
    return null;
  }
  const chars = raw.characters.map((ch) => String(ch ?? ""));
  const startsSec =
    raw.character_start_times_seconds ||
    raw.char_start_times_seconds ||
    (Array.isArray(raw.character_start_times_ms)
      ? raw.character_start_times_ms.map((value) => firstNumber(value, 0) / 1000)
      : null) ||
    (Array.isArray(raw.char_start_times_ms) ? raw.char_start_times_ms.map((value) => firstNumber(value, 0) / 1000) : null);
  const endsSec =
    raw.character_end_times_seconds ||
    raw.char_end_times_seconds ||
    (Array.isArray(raw.character_end_times_ms)
      ? raw.character_end_times_ms.map((value) => firstNumber(value, 0) / 1000)
      : null) ||
    (Array.isArray(raw.char_end_times_ms) ? raw.char_end_times_ms.map((value) => firstNumber(value, 0) / 1000) : null);

  if (!Array.isArray(startsSec) || !Array.isArray(endsSec)) {
    return null;
  }
  return {
    characters: chars,
    starts: startsSec.map((value) => firstNumber(value, 0)),
    ends: endsSec.map((value) => firstNumber(value, 0)),
  };
}

function wordsFromAlignment(alignment) {
  if (!alignment) return [];

  const out = [];
  let current = "";
  let start = null;
  let end = null;

  const flush = () => {
    const text = normalizeCueText(current);
    if (text && start !== null && end !== null && end > start) {
      out.push({ text, start, end });
    }
    current = "";
    start = null;
    end = null;
  };

  for (let i = 0; i < alignment.characters.length; i += 1) {
    const ch = alignment.characters[i] || "";
    const s = firstNumber(alignment.starts[i], end ?? 0);
    const e = Math.max(s + 0.02, firstNumber(alignment.ends[i], s + 0.04));

    if (/\s/.test(ch)) {
      flush();
      continue;
    }

    if (start === null) start = s;
    current += ch;
    end = e;

    if (/[.!?;:]$/.test(ch)) {
      flush();
    }
  }
  flush();
  return out;
}

function estimateWordTimings(text, duration) {
  const words = normalizeCaption(text)
    .split(" ")
    .map((w) => w.trim())
    .filter(Boolean);
  if (words.length === 0 || duration <= 0) return [];

  const weights = words.map((word) => {
    const core = word.replace(/[^\w]/g, "");
    return clamp(core.length, 1, 12);
  });
  const total = weights.reduce((sum, value) => sum + value, 0);
  let cursor = 0;

  return words.map((word, index) => {
    const share = duration * (weights[index] / total);
    const start = cursor;
    cursor += share;
    const end = Math.min(duration, cursor);
    return { text: word, start, end };
  });
}

function chunkWordsToCaptionCues(words, sceneStart, sceneDuration) {
  if (!Array.isArray(words) || words.length === 0) return [];

  const maxWords = clamp(envInt("TUTORIAL_CAPTION_MAX_WORDS", 8), 3, 20);
  const maxSeconds = clamp(envFloat("TUTORIAL_CAPTION_MAX_SECONDS", 2.6), 0.8, 6);
  const minSeconds = clamp(envFloat("TUTORIAL_CAPTION_MIN_SECONDS", 0.6), 0.2, 2.5);

  const out = [];
  let chunk = [];

  const flush = () => {
    if (chunk.length === 0) return;
    const rawStart = clamp(chunk[0].start, 0, sceneDuration);
    const rawEnd = clamp(chunk[chunk.length - 1].end, 0, sceneDuration);
    const minEnd = Math.min(sceneDuration, rawStart + minSeconds);
    const localEnd = Math.max(rawEnd, minEnd);
    const text = normalizeCueText(chunk.map((part) => part.text).join(" "));
    if (text && localEnd - rawStart > 0.05) {
      out.push({
        start: sceneStart + rawStart,
        end: sceneStart + localEnd,
        text,
      });
    }
    chunk = [];
  };

  for (const word of words) {
    chunk.push(word);
    const start = chunk[0].start;
    const end = word.end;
    const duration = end - start;
    const overWordBudget = chunk.length >= maxWords;
    const overTimeBudget = duration >= maxSeconds;
    const punctuationBreak = /[.!?]$/.test(word.text);
    if (overWordBudget || overTimeBudget || punctuationBreak) {
      flush();
    }
  }
  flush();

  return out;
}

function buildFallbackCaptionCues(scenes, timeline) {
  const cues = [];
  for (const [index, segment] of timeline.entries()) {
    const scene = scenes[index];
    const text = normalizeCaption(scene.voiceover || scene.caption || scene.title || `Scene ${index + 1}`);
    const estimated = estimateWordTimings(text, segment.duration);
    const chunked = chunkWordsToCaptionCues(estimated, segment.start, segment.duration);
    if (chunked.length > 0) {
      cues.push(...chunked);
    } else {
      cues.push({ start: segment.start, end: segment.end, text });
    }
  }
  return cues;
}

async function writeCaptionCues(cues, outPath, totalDuration = null) {
  const sorted = [...(Array.isArray(cues) ? cues : [])]
    .filter((cue) => cue && normalizeCueText(cue.text).length > 0)
    .map((cue) => {
      const start = Math.max(0, firstNumber(cue.start, 0));
      const end = Math.max(start + 0.05, firstNumber(cue.end, start + 0.75));
      return { start, end, text: normalizeCueText(cue.text) };
    })
    .sort((a, b) => a.start - b.start);

  if (sorted.length > 0 && Number.isFinite(totalDuration) && totalDuration > 0) {
    const last = sorted[sorted.length - 1];
    last.end = Math.max(last.end, totalDuration);
  }

  let srt = "";
  for (const [i, cue] of sorted.entries()) {
    srt += `${i + 1}\n`;
    srt += `${toSrtTimestamp(cue.start)} --> ${toSrtTimestamp(cue.end)}\n`;
    srt += `${cue.text}\n\n`;
  }
  await writeFile(outPath, srt, "utf8");
}

async function synthVoiceEdge(text, outAudioPath) {
  const voice = process.env.TUTORIAL_VOICE_NAME || "en-US-JennyNeural";
  const rate = process.env.TUTORIAL_VOICE_RATE || "+0%";
  const pitch = process.env.TUTORIAL_VOICE_PITCH || "+0Hz";
  const volume = process.env.TUTORIAL_VOICE_VOLUME || "+0%";
  const tmpMedia = outAudioPath.replace(/\.wav$/, ".edge.mp3");

  const result = tryRun("edge-tts", [
    "--voice",
    voice,
    "--rate",
    rate,
    "--pitch",
    pitch,
    "--volume",
    volume,
    "--text",
    text,
    "--write-media",
    tmpMedia,
  ]);
  if (!result.ok) {
    await rm(tmpMedia, { force: true });
    return { ok: false, error: result.stderr || "edge-tts failed" };
  }

  run("ffmpeg", ["-y", "-i", tmpMedia, "-ar", "48000", "-ac", "1", outAudioPath]);
  let duration = 0;
  try {
    duration = durationOf(outAudioPath);
  } catch (_err) {
    await rm(tmpMedia, { force: true });
    return { ok: false, error: "edge-tts produced invalid audio" };
  }
  await rm(tmpMedia, { force: true });
  return { ok: true, duration };
}

async function synthVoiceSay(text, outAudioPath, outDir) {
  const preferredVoice = process.env.TUTORIAL_VOICE_NAME || "Samantha";
  const rate = process.env.TUTORIAL_VOICE_RATE_WPM || "168";
  const tmpId = `${Date.now()}-${Math.random().toString(36).slice(2)}`;
  const txtFile = path.join(outDir, `say-${tmpId}.txt`);
  const aiffFile = outAudioPath.replace(/\.wav$/, ".aiff");

  await writeFile(txtFile, `${text}\n`, "utf8");

  const attempts = [
    ["-v", preferredVoice, "-r", rate, "-f", txtFile, "-o", aiffFile],
    ["-v", "Samantha", "-r", rate, "-f", txtFile, "-o", aiffFile],
    ["-r", rate, "-f", txtFile, "-o", aiffFile],
  ];

  let lastError = "say failed";
  for (const args of attempts) {
    await rm(aiffFile, { force: true });
    const result = tryRun("say", args);
    if (!result.ok) {
      lastError = result.stderr || lastError;
      continue;
    }

    try {
      run("ffmpeg", ["-y", "-i", aiffFile, "-ar", "48000", "-ac", "1", outAudioPath]);
      const dur = durationOf(outAudioPath);
      if (dur > 0.08) {
        await rm(txtFile, { force: true });
        await rm(aiffFile, { force: true });
        return { ok: true, duration: dur };
      }
      lastError = "say produced empty audio";
    } catch (err) {
      lastError = err instanceof Error ? err.message : "say conversion failed";
    }
  }

  await rm(txtFile, { force: true });
  await rm(aiffFile, { force: true });
  await rm(outAudioPath, { force: true });
  return { ok: false, error: lastError };
}

async function synthVoiceEspeak(cmd, text, outAudioPath) {
  const rate = process.env.TUTORIAL_VOICE_RATE_WPM || "160";
  const voice = process.env.TUTORIAL_VOICE_NAME || "en-us+f3";
  const result = tryRun(cmd, ["-s", rate, "-v", voice, text, "-w", outAudioPath]);
  if (!result.ok) {
    return { ok: false, error: result.stderr || `${cmd} failed` };
  }
  try {
    const duration = durationOf(outAudioPath);
    return { ok: true, duration };
  } catch (_err) {
    return { ok: false, error: `${cmd} produced invalid audio` };
  }
}

async function synthVoiceElevenLabs(text, outAudioPath) {
  const apiKey = process.env.ELEVENLABS_API_KEY || process.env.TUTORIAL_ELEVENLABS_API_KEY;
  if (!apiKey) {
    return { ok: false, error: "Missing ELEVENLABS_API_KEY" };
  }

  const settings = elevenLabsSettings();
  const timeoutMs = clamp(envInt("TUTORIAL_ELEVENLABS_TIMEOUT_MS", 90000), 2000, 180000);
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${settings.endpoint}/v1/text-to-speech/${settings.voice_id}/with-timestamps`, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        "xi-api-key": apiKey,
      },
      signal: controller.signal,
      body: JSON.stringify({
        text,
        model_id: settings.model_id,
        output_format: settings.output_format,
        voice_settings: {
          stability: settings.stability,
          similarity_boost: settings.similarity_boost,
          style: settings.style,
          use_speaker_boost: settings.use_speaker_boost,
          speed: settings.speed,
        },
      }),
    });

    if (!response.ok) {
      const details = (await response.text()).slice(0, 400);
      return { ok: false, error: `ElevenLabs ${response.status}: ${details}` };
    }

    const payload = await response.json();
    if (!payload?.audio_base64) {
      return { ok: false, error: "ElevenLabs did not return audio_base64" };
    }

    const rawAudio = outAudioPath.replace(/\.wav$/, ".elevenlabs.mp3");
    await writeFile(rawAudio, Buffer.from(payload.audio_base64, "base64"));
    run("ffmpeg", ["-y", "-i", rawAudio, "-ar", "48000", "-ac", "1", outAudioPath]);
    const duration = durationOf(outAudioPath);
    await rm(rawAudio, { force: true });

    const alignment = alignmentFromPayload(payload.normalized_alignment || payload.alignment);
    return {
      ok: true,
      duration,
      alignment,
      meta: {
        voice_id: settings.voice_id,
        model_id: settings.model_id,
        output_format: settings.output_format,
      },
    };
  } catch (err) {
    return { ok: false, error: err instanceof Error ? err.message : String(err) };
  } finally {
    clearTimeout(timer);
  }
}

async function synthSceneVoice(provider, text, outAudioPath, outDir, cacheDir) {
  const cleaned = normalizeCaption(text);
  if (!cleaned) return { ok: false, error: "Empty scene voice text" };

  const key = voiceCacheKey(provider, cleaned);
  const providerCacheDir = path.join(cacheDir, provider);
  const cacheAudioPath = path.join(providerCacheDir, `${key}.wav`);
  const cacheMetaPath = path.join(providerCacheDir, `${key}.json`);

  if ((await fileExists(cacheAudioPath)) && (await fileExists(cacheMetaPath))) {
    await cp(cacheAudioPath, outAudioPath);
    let cachePayload = null;
    try {
      cachePayload = JSON.parse(await readFile(cacheMetaPath, "utf8"));
    } catch (_err) {
      cachePayload = null;
    }
    const duration = firstNumber(cachePayload?.duration, 0) > 0 ? firstNumber(cachePayload.duration, 0) : durationOf(outAudioPath);
    return {
      ok: true,
      duration,
      alignment: cachePayload?.alignment || null,
      meta: cachePayload?.meta || null,
      cacheHit: true,
      cacheKey: key,
    };
  }

  let result = { ok: false, error: `Unsupported provider: ${provider}` };
  if (provider === "elevenlabs") {
    result = await synthVoiceElevenLabs(cleaned, outAudioPath);
  } else if (provider === "edge") {
    result = await synthVoiceEdge(cleaned, outAudioPath);
  } else if (provider === "say") {
    result = await synthVoiceSay(cleaned, outAudioPath, outDir);
  } else if (provider === "espeak-ng") {
    result = await synthVoiceEspeak("espeak-ng", cleaned, outAudioPath);
  } else if (provider === "espeak") {
    result = await synthVoiceEspeak("espeak", cleaned, outAudioPath);
  }
  if (!result.ok) {
    return result;
  }

  const duration = firstNumber(result.duration, 0) > 0 ? firstNumber(result.duration, 0) : durationOf(outAudioPath);
  await mkdir(providerCacheDir, { recursive: true });
  await cp(outAudioPath, cacheAudioPath);
  await writeFile(
    cacheMetaPath,
    `${JSON.stringify(
      {
        provider,
        cache_key: key,
        text: cleaned,
        settings: voiceCacheSettings(provider),
        duration,
        alignment: result.alignment || null,
        meta: result.meta || null,
        created_at: new Date().toISOString(),
      },
      null,
      2,
    )}\n`,
    "utf8",
  );

  return {
    ...result,
    duration,
    cacheHit: false,
    cacheKey: key,
  };
}

function concatSceneVoiceTracks(alignedTracks, voiceoverPath) {
  if (alignedTracks.length === 1) {
    return cp(alignedTracks[0], voiceoverPath);
  }

  const ffArgs = ["-y"];
  for (const track of alignedTracks) {
    ffArgs.push("-i", track);
  }

  const concatInputs = alignedTracks.map((_track, index) => `[${index}:a]`).join("");
  ffArgs.push(
    "-filter_complex",
    `${concatInputs}concat=n=${alignedTracks.length}:v=0:a=1[a]`,
    "-map",
    "[a]",
    "-ar",
    "48000",
    "-ac",
    "1",
    voiceoverPath,
  );
  run("ffmpeg", ffArgs);
}

function voiceSceneGapSeconds() {
  return clamp(envFloat("TUTORIAL_SCENE_AUDIO_GAP", 0.12), 0, 0.8);
}

function voiceClipFadeOutSeconds() {
  return clamp(envFloat("TUTORIAL_SCENE_AUDIO_CLIP_FADE", 0.09), 0.02, 0.4);
}

function fitVoiceToScene(inputPath, outPath, sceneDuration, desiredGap, clipFadeSeconds, rawDuration) {
  const boundedGap = Math.min(desiredGap, Math.max(0, sceneDuration - 0.08));
  const keepConstantGap = rawDuration <= sceneDuration - boundedGap + 0.01;
  const appliedGap = keepConstantGap ? boundedGap : 0;
  const speechBudget = Math.max(0.08, sceneDuration - appliedGap);
  const clipped = rawDuration > speechBudget + 0.01;
  const spokenDuration = Math.min(rawDuration, speechBudget);

  const filters = [];
  if (clipped) {
    filters.push(`atrim=end=${spokenDuration.toFixed(3)}`);
    const fadeDuration = Math.min(clipFadeSeconds, Math.max(0.03, spokenDuration * 0.45));
    const fadeStart = Math.max(0, spokenDuration - fadeDuration);
    filters.push(`afade=t=out:st=${fadeStart.toFixed(3)}:d=${fadeDuration.toFixed(3)}`);
  }
  filters.push("apad");
  run("ffmpeg", [
    "-y",
    "-i",
    inputPath,
    "-af",
    filters.join(","),
    "-t",
    sceneDuration.toFixed(3),
    "-ar",
    "48000",
    "-ac",
    "1",
    outPath,
  ]);

  const cueDuration = Math.max(0.05, Math.min(spokenDuration, sceneDuration));
  const trailingGap = Math.max(0, sceneDuration - cueDuration);
  return {
    cueStartOffset: 0,
    cueDuration,
    clipped,
    desiredGap,
    appliedGap,
    trailingGap,
  };
}

function normalizeVideoClips(scenes, outDir, workDir, width, height, trimStart, firstSceneTrimStart, settlePad) {
  const outputs = [];
  for (const [idx, scene] of scenes.entries()) {
    const src = path.resolve(outDir, scene.clip);
    const out = path.join(workDir, `${String(idx + 1).padStart(2, "0")}-${scene.id}.mp4`);
    const startTrim = Math.max(0, idx === 0 ? firstSceneTrimStart : trimStart);
    const padPrefix =
      settlePad > 0 ? `,tpad=start_duration=${settlePad.toFixed(3)}:start_mode=clone` : "";
    run("ffmpeg", [
      "-y",
      "-i",
      src,
      "-an",
      "-vf",
      `fps=30,trim=start=${startTrim.toFixed(3)},setpts=PTS-STARTPTS${padPrefix},scale=${width}:${height}:force_original_aspect_ratio=decrease,pad=${width}:${height}:(ow-iw)/2:(oh-ih)/2,format=yuv420p`,
      "-c:v",
      "libx264",
      "-preset",
      "veryfast",
      "-crf",
      "20",
      out,
    ]);
    outputs.push(out);
  }
  return outputs;
}

function composeVideo(normalized, fade, outPath) {
  if (normalized.length === 1) {
    return cp(normalized[0], outPath);
  }

  const ffArgs = ["-y"];
  for (const clip of normalized) {
    ffArgs.push("-i", clip);
  }

  const durations = normalized.map(durationOf);
  const chains = [];
  const transitions = transitionSequence(normalized.length);
  let cumulative = durations[0];
  chains.push(
    `[0:v][1:v]xfade=transition=${transitions[0]}:duration=${fade}:offset=${(durations[0] - fade).toFixed(3)}[v1]`,
  );
  for (let i = 2; i < normalized.length; i += 1) {
    const offset = cumulative + durations[i - 1] - fade * i;
    chains.push(
      `[v${i - 1}][${i}:v]xfade=transition=${transitions[i - 1]}:duration=${fade}:offset=${offset.toFixed(3)}[v${i}]`,
    );
    cumulative += durations[i - 1];
  }

  ffArgs.push(
    "-filter_complex",
    chains.join(";"),
    "-map",
    `[v${normalized.length - 1}]`,
    "-r",
    "30",
    "-pix_fmt",
    "yuv420p",
    "-c:v",
    "libx264",
    "-preset",
    "veryfast",
    "-crf",
    "20",
    outPath,
  );
  run("ffmpeg", ffArgs);
}

function createSilenceTrack(outAudioPath, durationSeconds) {
  run("ffmpeg", [
    "-y",
    "-f",
    "lavfi",
    "-i",
    "anullsrc=channel_layout=mono:sample_rate=48000",
    "-t",
    durationSeconds.toFixed(3),
    "-ar",
    "48000",
    "-ac",
    "1",
    outAudioPath,
  ]);
}

async function renderSceneVoiceover(scenes, timeline, fade, outDir, workDir, voiceProviderOverride) {
  const providerPick = pickVoiceProviders(voiceProviderOverride);
  const attemptedProviders = [];
  const sceneGap = voiceSceneGapSeconds();
  const clipFadeSeconds = voiceClipFadeOutSeconds();
  const cacheDir = path.join(outDir, "cache", "voice");
  await mkdir(cacheDir, { recursive: true });

  for (const provider of providerPick.candidates) {
    if (provider === "none") break;

    const sceneVoiceDir = path.join(workDir, `scene-voice-${provider}`);
    await rm(sceneVoiceDir, { recursive: true, force: true });
    await mkdir(sceneVoiceDir, { recursive: true });

    const alignedTracks = [];
    const captionCues = [];
    let providerMeta = null;
    let alignedSceneCount = 0;
    let failed = false;
    let failureReason = "";
    let cacheHits = 0;
    let cacheMisses = 0;

    for (const [i, scene] of scenes.entries()) {
      const sceneId = `${String(i + 1).padStart(2, "0")}-${scene.id}`;
      const raw = path.join(sceneVoiceDir, `${sceneId}-raw.wav`);
      const aligned = path.join(sceneVoiceDir, `${sceneId}-fit.wav`);
      const text = normalizeCaption(scene.voiceover || scene.caption || scene.title || "");
      const slotDuration = sceneVoiceDuration(timeline, i, fade);

      if (!text) {
        createSilenceTrack(aligned, slotDuration);
        alignedTracks.push(aligned);
        continue;
      }

      const synth = await synthSceneVoice(provider, text, raw, outDir, cacheDir);
      if (!synth.ok) {
        failed = true;
        failureReason = synth.error || "voice synthesis failed";
        break;
      }
      if (synth.cacheHit) {
        cacheHits += 1;
      } else {
        cacheMisses += 1;
      }
      if (!providerMeta && synth.meta) {
        providerMeta = synth.meta;
      }

      let rawDuration = 0;
      try {
        rawDuration = durationOf(raw);
      } catch (_err) {
        rawDuration = firstNumber(synth.duration, 0);
      }
      if (rawDuration <= 0) {
        try {
          rawDuration = durationOf(raw);
        } catch (_err) {
          failed = true;
          failureReason = "failed to read raw voice duration";
          break;
        }
      }
      if (rawDuration < 0.08) {
        failed = true;
        failureReason = "raw voice track too short";
        break;
      }

      let fit = null;
      try {
        fit = fitVoiceToScene(raw, aligned, slotDuration, sceneGap, clipFadeSeconds, rawDuration);
      } catch (_err) {
        failed = true;
        failureReason = "failed to fit voice track to scene duration";
        break;
      }
      alignedTracks.push(aligned);

      let words = wordsFromAlignment(synth.alignment);
      const usedAlignment = words.length > 0;
      if (!usedAlignment) {
        words = estimateWordTimings(text, Math.min(rawDuration, fit.cueDuration));
      } else {
        alignedSceneCount += 1;
      }

      const scaledWords = words
        .map((word) => {
          const localStart = clamp(
            word.start + fit.cueStartOffset,
            fit.cueStartOffset,
            fit.cueStartOffset + fit.cueDuration,
          );
          const localEnd = clamp(
            word.end + fit.cueStartOffset,
            localStart + 0.02,
            fit.cueStartOffset + fit.cueDuration,
          );
          return { text: word.text, start: localStart, end: localEnd };
        })
        .filter((word) => word.end - word.start > 0.02);

      const cueGroup = chunkWordsToCaptionCues(scaledWords, timeline[i].start, slotDuration);
      if (cueGroup.length > 0) {
        captionCues.push(...cueGroup);
      } else {
        captionCues.push({
          start: timeline[i].start,
          end: timeline[i].start + slotDuration,
          text: normalizeCueText(text),
        });
      }
    }

    if (failed || alignedTracks.length === 0) {
      attemptedProviders.push({
        provider,
        ok: false,
        reason: failureReason || "provider failed",
        cache: { hits: cacheHits, misses: cacheMisses },
      });
      continue;
    }

    const voiceoverPath = path.join(outDir, "voiceover.wav");
    await concatSceneVoiceTracks(alignedTracks, voiceoverPath);

    attemptedProviders.push({ provider, ok: true, cache: { hits: cacheHits, misses: cacheMisses } });
    const captionMode =
      alignedSceneCount === scenes.length
        ? "aligned"
        : alignedSceneCount > 0
          ? "hybrid"
          : "estimated";
    return {
      provider,
      voiceoverPath,
      sceneAudio: alignedTracks.map((p) => path.basename(p)),
      captionCues,
      captionMode,
      attemptedProviders,
      providerMeta,
      providerAvailability: providerPick.available,
      requestedProvider: providerPick.preferred,
      cacheStats: {
        hits: cacheHits,
        misses: cacheMisses,
        path: path.relative(outDir, cacheDir) || ".",
      },
      audioConfig: {
        target_scene_gap_seconds: sceneGap,
        clip_fade_out_seconds: clipFadeSeconds,
        silence_trimming: false,
        join_mode: "concat",
        speed_warping: false,
      },
    };
  }

  return {
    provider: "none",
    voiceoverPath: null,
    sceneAudio: [],
    captionCues: [],
    captionMode: "scene",
    attemptedProviders,
    providerMeta: null,
    providerAvailability: providerPick.available,
    requestedProvider: providerPick.preferred,
    cacheStats: {
      hits: 0,
      misses: 0,
      path: path.relative(outDir, cacheDir) || ".",
    },
    audioConfig: {
      target_scene_gap_seconds: sceneGap,
      clip_fade_out_seconds: clipFadeSeconds,
      silence_trimming: false,
      join_mode: "concat",
      speed_warping: false,
    },
  };
}

async function main() {
  await loadDotEnv(path.resolve(process.cwd(), ".env"));
  const { outDir, voiceProvider, strictVoice } = parseArgs(process.argv.slice(2));
  const manifestPath = path.join(outDir, "scene-manifest.json");
  const manifestRaw = await readFile(manifestPath, "utf8");
  const manifest = JSON.parse(manifestRaw);
  const scenes = Array.isArray(manifest?.scenes) ? manifest.scenes : [];

  if (!hasCommand("ffmpeg") || !hasCommand("ffprobe")) {
    throw new Error("ffmpeg and ffprobe are required for tutorial video composition.");
  }
  if (scenes.length === 0) {
    throw new Error("No scenes found in scene-manifest.json");
  }

  const width = manifest?.size?.width || 1280;
  const height = manifest?.size?.height || 720;
  const fade = clamp(envFloat("TUTORIAL_SCENE_FADE", 0.35), 0.05, 1.2);
  const trimStart = clamp(envFloat("TUTORIAL_SCENE_TRIM_START", 0.18), 0, 1.2);
  const firstSceneTrimStart = clamp(
    envFloat("TUTORIAL_SCENE_FIRST_TRIM_START", Math.max(trimStart, 0.95)),
    trimStart,
    2.5,
  );
  const settlePad = clamp(envFloat("TUTORIAL_SCENE_SETTLE_PAD", 0.06), 0, 0.3);

  const workDir = path.join(outDir, "work");
  await rm(workDir, { recursive: true, force: true });
  await mkdir(workDir, { recursive: true });
  await rm(path.join(outDir, "voiceover.aiff"), { force: true });

  const stale = await readdir(outDir, { withFileTypes: true }).catch(() => []);
  for (const ent of stale) {
    if (ent.isFile() && ent.name.startsWith("say-") && ent.name.endsWith(".txt")) {
      await rm(path.join(outDir, ent.name), { force: true });
    }
  }

  const normalized = normalizeVideoClips(
    scenes,
    outDir,
    workDir,
    width,
    height,
    trimStart,
    firstSceneTrimStart,
    settlePad,
  );
  const durations = normalized.map(durationOf);
  const timeline = timelineFor(durations, fade);

  const joinedVideo = path.join(workDir, "video-joined.mp4");
  await composeVideo(normalized, fade, joinedVideo);

  const narrationText = scenes
    .map((s, i) => `Scene ${i + 1} (${s.id}): ${normalizeCaption(s.voiceover || s.caption || s.title || "")}`)
    .join("\n");
  await writeFile(path.join(outDir, "narration.txt"), `${narrationText}\n`, "utf8");

  const voiceInfo = await renderSceneVoiceover(scenes, timeline, fade, outDir, workDir, voiceProvider);
  const requestedProvider = voiceInfo.requestedProvider;
  const strictRequested = requestedProvider !== "auto" && requestedProvider !== "none";
  if (strictVoice) {
    if (voiceInfo.provider === "none") {
      const reason = JSON.stringify(voiceInfo.attemptedProviders, null, 2);
      throw new Error(`Strict voice mode failed: no provider succeeded.\nAttempts: ${reason}`);
    }
    if (strictRequested && voiceInfo.provider !== requestedProvider) {
      const reason = JSON.stringify(voiceInfo.attemptedProviders, null, 2);
      throw new Error(
        `Strict voice mode failed: requested '${requestedProvider}', produced '${voiceInfo.provider}'.\nAttempts: ${reason}`,
      );
    }
  }

  let captionCues = voiceInfo.captionCues;
  let captionMode = voiceInfo.captionMode;
  if (!Array.isArray(captionCues) || captionCues.length === 0) {
    captionCues = buildFallbackCaptionCues(scenes, timeline);
    captionMode = "estimated";
  }

  const captionPath = path.join(outDir, "captions.srt");
  const outputDuration = timeline[timeline.length - 1]?.end || 0;
  await writeCaptionCues(captionCues, captionPath, outputDuration);
  await writeFile(path.join(outDir, "caption-cues.json"), `${JSON.stringify(captionCues, null, 2)}\n`, "utf8");

  const finalVideo = path.join(outDir, "tutorial.mp4");
  if (voiceInfo.voiceoverPath) {
    const masteringFilter =
      process.env.TUTORIAL_VOICE_MASTERING_FILTER ||
      "highpass=f=65,lowpass=f=12000,acompressor=threshold=-18dB:ratio=2.6:attack=18:release=220:makeup=2,loudnorm=I=-16:TP=-1.5:LRA=7";
    run("ffmpeg", [
      "-y",
      "-i",
      joinedVideo,
      "-i",
      voiceInfo.voiceoverPath,
      "-i",
      captionPath,
      "-filter:a",
      masteringFilter,
      "-map",
      "0:v:0",
      "-map",
      "1:a:0",
      "-map",
      "2:0",
      "-c:v",
      "copy",
      "-c:a",
      "aac",
      "-b:a",
      "192k",
      "-c:s",
      "mov_text",
      "-metadata:s:s:0",
      "language=eng",
      finalVideo,
    ]);
  } else {
    run("ffmpeg", [
      "-y",
      "-i",
      joinedVideo,
      "-i",
      captionPath,
      "-map",
      "0:v:0",
      "-map",
      "1:0",
      "-c:v",
      "copy",
      "-c:s",
      "mov_text",
      "-metadata:s:s:0",
      "language=eng",
      finalVideo,
    ]);
  }

  const metaPath = path.join(outDir, "tutorial-metadata.json");
  await writeFile(
    metaPath,
    `${JSON.stringify(
      {
        generated_at: new Date().toISOString(),
        output: "tutorial.mp4",
        scene_count: scenes.length,
        duration_seconds: Number(durationOf(finalVideo).toFixed(2)),
        voice_provider: voiceInfo.provider,
        requested_voice_provider: voiceInfo.requestedProvider,
        voice_provider_availability: voiceInfo.providerAvailability,
        attempted_voice_providers: voiceInfo.attemptedProviders,
        strict_voice: strictVoice,
        voice_provider_meta: voiceInfo.providerMeta,
        voiceover: voiceInfo.voiceoverPath ? path.basename(voiceInfo.voiceoverPath) : null,
        scene_audio: voiceInfo.sceneAudio,
        voice_cache: voiceInfo.cacheStats,
        voice_audio: voiceInfo.audioConfig,
        captions: {
          file: "captions.srt",
          cue_count: captionCues.length,
          mode: captionMode,
          cues_json: "caption-cues.json",
        },
        fade_seconds: fade,
        clip_trim_start_seconds: trimStart,
        clip_first_trim_start_seconds: firstSceneTrimStart,
        clip_settle_pad_seconds: settlePad,
        timeline_seconds: timeline.map((seg) => ({
          index: seg.index + 1,
          start: Number(seg.start.toFixed(3)),
          end: Number(seg.end.toFixed(3)),
        })),
      },
      null,
      2,
    )}\n`,
    "utf8",
  );

  process.stdout.write(
    `Tutorial video ready: ${finalVideo} (voice=${voiceInfo.provider}, captions=${captionMode}, cues=${captionCues.length}, cache=${voiceInfo.cacheStats.hits} hit/${voiceInfo.cacheStats.misses} miss)\n`,
  );
}

main().catch((err) => {
  process.stderr.write(`${err instanceof Error ? err.message : String(err)}\n`);
  process.exit(1);
});
