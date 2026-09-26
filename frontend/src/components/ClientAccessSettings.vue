<script setup lang="ts">
import { CheckCircle2, Info, Link2, LoaderCircle, Plus, RotateCcw, Save, Server, Trash2 } from '@lucide/vue'
import { computed, onMounted, ref, watch } from 'vue'
import { settingsApi } from '@/api/modules'
import { ApiError } from '@/api/client'
import { useToast } from '@/composables/useToast'
import type { SettingsSectionState } from '@/composables/useSettingsNavigation'

const emit = defineEmits<{ state: [value: SettingsSectionState] }>()
const toast = useToast()
const loading = ref(true)
const saving = ref(false)
const saved = ref({ subscription_server: '', traffic_servers: [] as string[] })
const form = ref({ subscription_server: '', traffic_servers: [] as string[] })
const dirty = computed(() => JSON.stringify(saved.value) !== JSON.stringify(form.value))
const configured = computed(() => Boolean(form.value.subscription_server))

watch([loading, dirty], () => emit('state', {
  status: loading.value ? '正在读取' : configured.value ? '已配置' : '尚未配置',
  tone: configured.value ? 'success' : 'warning', dirty: dirty.value,
}), { immediate: true })

function restore() {
  form.value = {
    subscription_server: saved.value.subscription_server,
    traffic_servers: [...saved.value.traffic_servers],
  }
}

function addTrafficServer() {
  form.value.traffic_servers.push('')
}

function removeTrafficServer(index: number) {
  form.value.traffic_servers.splice(index, 1)
}

async function load() {
  loading.value = true
  try {
    const value = await settingsApi.getClientAccess()
    saved.value = { subscription_server: value.subscription_server, traffic_servers: [...value.traffic_servers] }
    restore()
  } catch (error) {
    toast.error(error instanceof ApiError ? error.message : '读取客户端接入设置失败')
  } finally { loading.value = false }
}

async function save() {
  saving.value = true
  try {
    const value = await settingsApi.updateClientAccess(form.value)
    saved.value = { subscription_server: value.subscription_server, traffic_servers: [...value.traffic_servers] }
    restore()
    toast.success('客户端接入设置已保存')
  } catch (error) {
    toast.error(error instanceof ApiError ? error.message : '保存失败')
  } finally { saving.value = false }
}

onMounted(load)
</script>

<template>
  <section class="client-access space-y-5">
    <header class="hero-card">
      <span class="hero-icon"><Link2 :size="23" /></span>
      <div class="relative z-10 min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2.5">
          <h2>客户端接入</h2>
          <span class="status-pill" :class="configured ? 'status-ready' : 'status-empty'">
            <CheckCircle2 v-if="configured" :size="14" />
            {{ configured ? '已配置' : '等待配置' }}
          </span>
        </div>
        <p>设置新 VNT 设备订阅使用的公网控制端点，以及可选的默认流量端点。</p>
      </div>
    </header>

    <div class="settings-card">
      <div v-if="loading" class="flex min-h-80 items-center justify-center gap-3 text-sm text-slate-500 dark:text-slate-400">
        <LoaderCircle :size="20" class="animate-spin text-blue-600" /> 正在读取设置…
      </div>

      <template v-else>
        <div class="section-heading">
          <span class="heading-icon"><Server :size="18" /></span>
          <div>
            <h3>连接端点</h3>
            <p>订阅服务器用于控制连接；流量端点可按需与它分离部署。</p>
          </div>
        </div>

        <div class="mt-6 grid gap-5 lg:grid-cols-2">
          <div class="form-panel form-panel-primary">
            <label for="subscription-server" class="form-label">订阅服务器 <span>必填</span></label>
            <p class="form-help">客户端从这个唯一地址连接并获取订阅配置。</p>
            <input
              id="subscription-server"
              v-model.trim="form.subscription_server"
              class="endpoint-input mt-4"
              autocomplete="url"
              placeholder="tcp://vpn.example.com:29872"
            />
            <p class="endpoint-note"><Info :size="15" />填写完整协议地址，例如 <code>tcp://</code>、<code>quic://</code> 或 <code>wss://</code>。</p>
          </div>

          <div class="form-panel">
            <div class="flex items-start justify-between gap-3">
              <div>
                <label class="form-label" for="traffic-server-0">默认流量服务器</label>
                <p class="form-help">留空时，流量连接与订阅连接使用同一个端点。</p>
              </div>
              <button type="button" class="add-button" @click="addTrafficServer"><Plus :size="16" />添加</button>
            </div>

            <div v-if="form.traffic_servers.length" class="mt-4 space-y-2.5">
              <div v-for="(_, index) in form.traffic_servers" :key="index" class="endpoint-row">
                <span class="endpoint-index">{{ index + 1 }}</span>
                <input
                  :id="`traffic-server-${index}`"
                  v-model.trim="form.traffic_servers[index]"
                  class="endpoint-input"
                  :placeholder="index ? 'tcp://edge.example.com:29872' : 'quic://vpn.example.com:29872'"
                />
                <button type="button" class="remove-button" :aria-label="`删除流量服务器 ${index + 1}`" title="删除端点" @click="removeTrafficServer(index)"><Trash2 :size="16" /></button>
              </div>
            </div>
            <div v-else class="empty-state"><Server :size="18" /><span>未配置独立流量服务器，将复用订阅服务器。</span></div>
          </div>
        </div>

        <footer class="action-bar">
          <p v-if="dirty" class="pending-note"><span />有未保存的修改</p>
          <p v-else class="saved-note">所有修改均已保存</p>
          <div class="flex flex-wrap items-center gap-2.5">
            <button type="button" class="reset-button" :disabled="saving || !dirty" @click="restore"><RotateCcw :size="16" />撤销修改</button>
            <button type="button" class="save-button" :disabled="saving || !dirty" @click="save">
              <LoaderCircle v-if="saving" :size="16" class="animate-spin" />
              <Save v-else :size="16" />{{ saving ? '保存中…' : '保存设置' }}
            </button>
          </div>
        </footer>
      </template>
    </div>
  </section>
</template>

<style scoped>
@reference "../style.css";
.hero-card { @apply relative flex items-start gap-4 overflow-hidden rounded-2xl border border-blue-100 bg-gradient-to-br from-blue-50 via-white to-sky-50 px-5 py-5 shadow-sm dark:border-blue-900/60 dark:from-slate-800 dark:via-slate-800 dark:to-blue-950/40 sm:px-7 sm:py-6; }
.hero-card::after { @apply absolute -right-10 -top-14 h-36 w-36 rounded-full bg-blue-200/40 blur-2xl content-[''] dark:bg-blue-500/10; }
.hero-icon { @apply relative z-10 flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-blue-600 text-white shadow-lg shadow-blue-600/20; }
.hero-card h2 { @apply text-lg font-bold tracking-tight text-slate-900 dark:text-white; }
.hero-card p { @apply mt-1.5 max-w-2xl text-sm leading-6 text-slate-600 dark:text-slate-300; }
.status-pill { @apply inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-xs font-semibold; }
.status-ready { @apply bg-emerald-100 text-emerald-700 dark:bg-emerald-500/15 dark:text-emerald-300; }
.status-empty { @apply bg-amber-100 text-amber-700 dark:bg-amber-500/15 dark:text-amber-300; }
.settings-card { @apply rounded-2xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-700 dark:bg-slate-800 sm:p-7; }
.section-heading { @apply flex items-start gap-3; }
.heading-icon { @apply flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-slate-100 text-slate-600 dark:bg-slate-700/70 dark:text-slate-200; }
.section-heading h3 { @apply pt-0.5 text-sm font-bold text-slate-900 dark:text-white; }
.section-heading p { @apply mt-0.5 text-sm leading-5 text-slate-500 dark:text-slate-400; }
.form-panel { @apply rounded-xl border border-slate-200 bg-slate-50/70 p-4 dark:border-slate-700 dark:bg-slate-900/30; }
.form-panel-primary { @apply border-blue-200 bg-blue-50/50 dark:border-blue-900/70 dark:bg-blue-950/20; }
.form-label { @apply text-sm font-bold text-slate-800 dark:text-slate-100; }
.form-label span { @apply ml-1.5 rounded-full bg-blue-100 px-1.5 py-0.5 text-[11px] font-semibold text-blue-700 dark:bg-blue-500/15 dark:text-blue-300; }
.form-help { @apply mt-1 text-xs leading-5 text-slate-500 dark:text-slate-400; }
.endpoint-input { @apply min-w-0 w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 font-mono text-sm text-slate-800 shadow-sm outline-none transition placeholder:font-sans placeholder:text-slate-400 hover:border-slate-400 focus:border-blue-500 focus:ring-4 focus:ring-blue-500/10 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:hover:border-slate-500 dark:focus:border-blue-400 dark:focus:ring-blue-400/15; }
.endpoint-note { @apply mt-3 flex items-start gap-1.5 text-xs leading-5 text-slate-500 dark:text-slate-400; }
.endpoint-note code { @apply rounded bg-slate-200 px-1 py-0.5 font-mono text-[11px] text-slate-600 dark:bg-slate-700 dark:text-slate-300; }
.add-button { @apply inline-flex shrink-0 items-center gap-1.5 rounded-lg border border-blue-200 bg-white px-3 py-2 text-xs font-bold text-blue-700 shadow-sm transition hover:border-blue-300 hover:bg-blue-50 focus:outline-none focus:ring-4 focus:ring-blue-500/10 dark:border-blue-800 dark:bg-slate-800 dark:text-blue-300 dark:hover:bg-blue-950/40; }
.endpoint-row { @apply flex items-center gap-2; }
.endpoint-index { @apply flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-slate-200/70 text-xs font-bold text-slate-500 dark:bg-slate-700 dark:text-slate-300; }
.remove-button { @apply inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-slate-200 bg-white text-slate-400 transition hover:border-red-200 hover:bg-red-50 hover:text-red-600 focus:outline-none focus:ring-4 focus:ring-red-500/10 dark:border-slate-600 dark:bg-slate-800 dark:hover:border-red-900 dark:hover:bg-red-950/30 dark:hover:text-red-300; }
.empty-state { @apply mt-4 flex min-h-24 items-center justify-center gap-2 rounded-lg border border-dashed border-slate-300 bg-white/60 px-4 text-center text-xs leading-5 text-slate-500 dark:border-slate-600 dark:bg-slate-800/40 dark:text-slate-400; }
.action-bar { @apply mt-8 flex flex-col-reverse gap-4 border-t border-slate-200 pt-5 dark:border-slate-700 sm:flex-row sm:items-center sm:justify-between; }
.pending-note, .saved-note { @apply flex items-center gap-2 text-xs font-medium; }
.pending-note { @apply text-amber-700 dark:text-amber-300; }.pending-note span { @apply h-2 w-2 rounded-full bg-amber-500; }.saved-note { @apply text-slate-500 dark:text-slate-400; }
.reset-button { @apply inline-flex items-center justify-center gap-1.5 rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm font-semibold text-slate-600 shadow-sm transition hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-45 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-200 dark:hover:bg-slate-700; }
.save-button { @apply inline-flex items-center justify-center gap-1.5 rounded-lg bg-blue-600 px-4 py-2.5 text-sm font-bold text-white shadow-sm shadow-blue-600/20 transition hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-500/25 disabled:cursor-not-allowed disabled:opacity-45 dark:bg-blue-500 dark:hover:bg-blue-400; }
</style>
