<script setup lang="ts">
import { Combobox, ThemeSelector, Toggle } from '@fernite/ui'
import { ref, watch } from 'vue'

import ColorPickerCustom from '@/components/ui/ColorPickerCustom.vue'
import { get, set } from '@/helpers/settings.ts'
import { getOS } from '@/helpers/utils'
import { useTheming } from '@/store/state'
import type { ColorTheme } from '@/store/theme.ts'

const themeStore = useTheming()

const os = ref(await getOS())
const settings = ref(await get())

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)

// Preset accent colors with labels
const presetColors = [
	{ hex: '#1bd96a', label: 'Green (Default)' },
	{ hex: '#3b82f6', label: 'Blue' },
	{ hex: '#8b5cf6', label: 'Purple' },
	{ hex: '#ec4899', label: 'Pink' },
	{ hex: '#ef4444', label: 'Red' },
	{ hex: '#f97316', label: 'Orange' },
	{ hex: '#eab308', label: 'Yellow' },
	{ hex: '#06b6d4', label: 'Cyan' },
	{ hex: '#14b8a6', label: 'Teal' },
	{ hex: '#f43f5e', label: 'Rose' },
	{ hex: '#a855f7', label: 'Violet' },
	{ hex: '#6366f1', label: 'Indigo' },
]

const currentAccent = ref(settings.value.accent_color || '#1bd96a')

function applyColor(hex: string) {
	currentAccent.value = hex
	if (hex === '#1bd96a') {
		themeStore.setCustomAccentColor('')
		settings.value.accent_color = hex
	} else {
		themeStore.setCustomAccentColor(hex)
		settings.value.accent_color = hex
	}
}

function resetToDefault() {
	currentAccent.value = '#1bd96a'
	themeStore.setCustomAccentColor('')
	settings.value.accent_color = '#1bd96a'
}
</script>

<template>
	<h2 class="m-0 text-lg font-extrabold text-contrast">Color theme</h2>
	<p class="m-0 mt-1">Select your preferred color theme for Fernite App.</p>

	<ThemeSelector
		:update-color-theme="
			(theme: ColorTheme) => {
				themeStore.setThemeState(theme)
				settings.theme = theme
			}
		"
		:current-theme="settings.theme"
		:theme-options="themeStore.getThemeOptions()"
		system-theme-color="system"
	/>

	<!-- Accent Color Picker -->
	<div class="mt-6">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="m-0 text-lg font-extrabold text-contrast">Accent color</h2>
				<p class="m-0 mt-1">Choose a custom accent color. Shades are generated automatically.</p>
			</div>
			<button
				v-if="currentAccent !== '#1bd96a'"
				class="px-3 py-1.5 text-sm font-semibold bg-bg-raised rounded-lg hover:bg-button-bg transition-colors border border-solid border-surface-5 text-secondary whitespace-nowrap"
				@click="resetToDefault"
			>
				Reset
			</button>
		</div>
		<ColorPickerCustom :model-value="currentAccent" :presets="presetColors" @update:model-value="applyColor" />
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Advanced rendering</h2>
			<p class="m-0 mt-1">
				Enables advanced rendering such as blur effects that may cause performance issues without
				hardware-accelerated rendering.
			</p>
		</div>

		<Toggle
			id="advanced-rendering"
			:model-value="themeStore.advancedRendering"
			@update:model-value="
				(e) => {
					themeStore.advancedRendering = !!e
					settings.advanced_rendering = themeStore.advancedRendering
				}
			"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Hide nametag</h2>
			<p class="m-0 mt-1">Disables the nametag above your player on the skins page.</p>
		</div>
		<Toggle id="hide-nametag-skins-page" v-model="settings.hide_nametag_skins_page" />
	</div>

	<div v-if="os !== 'MacOS'" class="mt-4 flex items-center justify-between gap-4">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Native decorations</h2>
			<p class="m-0 mt-1">Use system window frame (app restart required).</p>
		</div>
		<Toggle id="native-decorations" v-model="settings.native_decorations" />
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Minimize launcher</h2>
			<p class="m-0 mt-1">Minimize the launcher when a Minecraft process starts.</p>
		</div>
		<Toggle id="minimize-launcher" v-model="settings.hide_on_process_start" />
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Default landing page</h2>
			<p class="m-0 mt-1">Change the page to which the launcher opens on.</p>
		</div>
		<Combobox
			id="opening-page"
			v-model="settings.default_page"
			name="Opening page dropdown"
			class="w-40"
			:options="['Home', 'Library'].map((v) => ({ value: v, label: v }))"
			:display-value="settings.default_page ?? 'Select an option'"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Jump back into worlds</h2>
			<p class="m-0 mt-1">Includes recent worlds in the "Jump back in" section on the Home page.</p>
		</div>
		<Toggle
			:model-value="themeStore.getFeatureFlag('worlds_in_home')"
			@update:model-value="
				() => {
					const newValue = !themeStore.getFeatureFlag('worlds_in_home')
					themeStore.featureFlags['worlds_in_home'] = newValue
					settings.feature_flags['worlds_in_home'] = newValue
				}
			"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">Toggle sidebar</h2>
			<p class="m-0 mt-1">Enables the ability to toggle the sidebar.</p>
		</div>
		<Toggle
			id="toggle-sidebar"
			:model-value="settings.toggle_sidebar"
			@update:model-value="
				(e) => {
					settings.toggle_sidebar = !!e
					themeStore.toggleSidebar = settings.toggle_sidebar
				}
			"
		/>
	</div>
</template>

<style scoped lang="scss">
/* Styles in ColorPickerCustom.vue */
</style>
