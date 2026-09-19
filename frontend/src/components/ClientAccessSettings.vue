<script setup lang="ts">
import { Link2, LoaderCircle, Plus, Save, Trash2 } from '@lucide/vue'
import { computed, onMounted, ref, watch } from 'vue'
import { settingsApi } from '@/api/modules'
import { ApiError } from '@/api/client'
import { useToast } from '@/composables/useToast'
import type { SettingsSectionState } from '@/composables/useSettingsNavigation'

const emit = defineEmits<{ state: [value: SettingsSectionState] }>()
const toast = useToast()
const loading = ref(true)
const saving = ref(false)
const saved = ref({ server: [] as string[], cert_mode: 'finger' })
const form = ref({ server: [] as string[], cert_mode: 'finger' })
const dirty = computed(() => JSON.stringify(saved.value) !== JSON.stringify(form.value))

watch([loading, dirty], () => emit('state', {
  status: loading.value ? '正在读取' : form.value.server.length ? `${form.value.server.length} 个端点` : '尚未配置',
  tone: form.value.server.length ? 'success' : 'warning', dirty: dirty.value,
}), { immediate: true })

async function load() {
  loading.value = true
  try {
    const value = await settingsApi.getClientAccess()
    saved.value = { server: [...value.server], cert_mode: value.cert_mode }
    form.value = { server: [...value.server], cert_mode: value.cert_mode }
  } catch (error) {
    toast.error(error instanceof ApiError ? error.message : '读取客户端接入设置失败')
  } finally { loading.value = false }
}

async function save() {
  saving.value = true
  try {
    const value = await settingsApi.updateClientAccess(form.value)
    saved.value = { server: [...value.server], cert_mode: value.cert_mode }
    form.value = { server: [...value.server], cert_mode: value.cert_mode }
    toast.success('客户端接入设置已保存')
  } catch (error) {
    toast.error(error instanceof ApiError ? error.message : '保存失败')
  } finally { saving.value = false }
}

onMounted(load)
</script>

<template>
  <section class="space-y-4">
    <header class="rounded-xl border border-slate-200 bg-white px-6 py-4 shadow-sm dark:border-slate-700 dark:bg-slate-800">
      <div class="flex items-start gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-500/10"><Link2 :size="20" /></div>
        <div><h2 class="font-semibold text-slate-950 dark:text-white">客户端接入设置</h2><p class="mt-1 text-sm text-slate-500">新建 VNT 设备优先使用这里配置的公网端点；留空时按服务监听端口自动生成。</p></div>
      </div>
    </header>
    <div class="rounded-xl border border-slate-200 bg-white p-6 shadow-sm dark:border-slate-700 dark:bg-slate-800">
      <div v-if="loading" class="flex justify-center py-10"><LoaderCircle class="animate-spin" /></div>
      <template v-else>
        <div class="flex items-center justify-between gap-3">
          <label class="text-sm font-semibold">默认公网 VNTS 端点</label>
          <button class="secondary-button" @click="form.server.push('')"><Plus :size="15" />添加端点</button>
        </div>
        <div v-for="(_, index) in form.server" :key="index" class="mt-2 flex gap-2">
          <input v-model.trim="form.server[index]" class="field flex-1" placeholder="tcp://vpn.example.com:29872" />
          <button class="secondary-button" @click="form.server.splice(index, 1)"><Trash2 :size="15" /></button>
        </div>
        <label class="mt-5 block text-sm font-semibold">证书校验</label>
        <select v-model="form.cert_mode" class="field mt-2 w-full">
          <option value="finger">自动嵌入当前证书 SHA-256 指纹</option>
          <option value="standard">系统信任链</option>
        </select>
        <p class="mt-2 text-xs text-slate-500">可配置多个默认地址；留空时新增设备按 TCP → QUIC → WSS 监听端口生成地址。首个地址预填为当前服务器，其余地址预填为其他服务器。</p>
        <div class="mt-6 flex justify-end"><button class="primary-button" :disabled="saving || !dirty" @click="save"><Save :size="15" />{{ saving ? '保存中' : '保存' }}</button></div>
      </template>
    </div>
  </section>
</template>
