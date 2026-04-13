<script setup lang="ts">
import { computed, ref, watch } from 'vue'

const props = withDefaults(
	defineProps<{
		modelValue: string
		presets?: Array<{ hex: string; label: string }>
	}>(),
	{
		presets: () => [
			{ hex: '#1bd96a', label: 'Fernite Green' },
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
		],
	},
)

const emit = defineEmits<{
	'update:modelValue': [value: string]
}>()

// Parse hex to RGB
function hexToRgb(hex: string) {
	const clean = hex.replace('#', '')
	const r = parseInt(clean.substring(0, 2), 16)
	const g = parseInt(clean.substring(2, 4), 16)
	const b = parseInt(clean.substring(4, 6), 16)
	return { r: isNaN(r) ? 0 : r, g: isNaN(g) ? 0 : g, b: isNaN(b) ? 0 : b }
}

function rgbToHex(r: number, g: number, b: number) {
	return (
		'#' + [r, g, b].map((v) => Math.max(0, Math.min(255, v)).toString(16).padStart(2, '0')).join('')
	)
}

const rgb = ref(hexToRgb(props.modelValue || '#1bd96a'))
const hexInput = ref((props.modelValue || '#1bd96a').toUpperCase())

watch(
	() => props.modelValue,
	(val) => {
		if (val && val !== rgbToHex(rgb.value.r, rgb.value.g, rgb.value.b)) {
			rgb.value = hexToRgb(val)
			hexInput.value = val.toUpperCase()
		}
	},
)

const currentHex = computed(() => rgbToHex(rgb.value.r, rgb.value.g, rgb.value.b))

function onRgbChange() {
	const hex = currentHex.value
	hexInput.value = hex.toUpperCase()
	emit('update:modelValue', hex)
}

function onHexInput(e: Event) {
	const val = (e.target as HTMLInputElement).value
	hexInput.value = val
	const clean = val.replace('#', '')
	if (clean.length === 6 && /^[0-9a-fA-F]{6}$/.test(clean)) {
		rgb.value = hexToRgb('#' + clean)
		emit('update:modelValue', '#' + clean.toLowerCase())
	}
}

function applyPreset(hex: string) {
	rgb.value = hexToRgb(hex)
	hexInput.value = hex.toUpperCase()
	emit('update:modelValue', hex)
}

function luminance(r: number, g: number, b: number) {
	return 0.299 * r + 0.587 * g + 0.114 * b
}

const textOnColor = computed(() =>
	luminance(rgb.value.r, rgb.value.g, rgb.value.b) > 128 ? '#000' : '#fff',
)
</script>

<template>
	<div class="color-picker">
		<!-- Preview swatch -->
		<div class="color-preview" :style="{ background: currentHex, color: textOnColor }">
			<span class="color-preview-hex">{{ hexInput }}</span>
			<div class="color-preview-dots">
				<span class="dot" :style="{ background: currentHex }" />
				<span class="dot" :style="{ background: currentHex + 'aa' }" />
				<span class="dot" :style="{ background: currentHex + '44' }" />
			</div>
		</div>

		<!-- Preset swatches -->
		<div class="preset-grid">
			<button
				v-for="preset in presets"
				:key="preset.hex"
				v-tooltip="preset.label"
				class="preset-swatch"
				:class="{ active: currentHex.toLowerCase() === preset.hex.toLowerCase() }"
				:style="{ '--swatch': preset.hex }"
				@click="applyPreset(preset.hex)"
			/>
		</div>

		<!-- RGB sliders -->
		<div class="sliders">
			<div class="slider-row">
				<span class="slider-label">R</span>
				<div class="slider-track-wrap">
					<div
						class="slider-track-bg"
						:style="{ background: `linear-gradient(to right, #000, rgb(255,${rgb.g},${rgb.b}))` }"
					/>
					<input
						v-model.number="rgb.r"
						type="range"
						min="0"
						max="255"
						class="slider-input"
						@input="onRgbChange"
					/>
				</div>
				<input
					v-model.number="rgb.r"
					type="number"
					min="0"
					max="255"
					class="slider-number"
					@change="onRgbChange"
				/>
			</div>
			<div class="slider-row">
				<span class="slider-label">G</span>
				<div class="slider-track-wrap">
					<div
						class="slider-track-bg"
						:style="{ background: `linear-gradient(to right, #000, rgb(${rgb.r},255,${rgb.b}))` }"
					/>
					<input
						v-model.number="rgb.g"
						type="range"
						min="0"
						max="255"
						class="slider-input"
						@input="onRgbChange"
					/>
				</div>
				<input
					v-model.number="rgb.g"
					type="number"
					min="0"
					max="255"
					class="slider-number"
					@change="onRgbChange"
				/>
			</div>
			<div class="slider-row">
				<span class="slider-label">B</span>
				<div class="slider-track-wrap">
					<div
						class="slider-track-bg"
						:style="{ background: `linear-gradient(to right, #000, rgb(${rgb.r},${rgb.g},255))` }"
					/>
					<input
						v-model.number="rgb.b"
						type="range"
						min="0"
						max="255"
						class="slider-input"
						@input="onRgbChange"
					/>
				</div>
				<input
					v-model.number="rgb.b"
					type="number"
					min="0"
					max="255"
					class="slider-number"
					@change="onRgbChange"
				/>
			</div>
		</div>

		<!-- Hex input -->
		<div class="hex-row">
			<span class="hex-label">HEX</span>
			<input
				class="hex-input"
				:value="hexInput"
				maxlength="7"
				spellcheck="false"
				placeholder="#1bd96a"
				@input="onHexInput"
			/>
		</div>
	</div>
</template>

<style scoped lang="scss">
.color-picker {
	display: flex;
	flex-direction: column;
	gap: 1rem;
	width: 100%;
}

/* Preview */
.color-preview {
	border-radius: 0.75rem;
	padding: 0.875rem 1rem;
	display: flex;
	align-items: center;
	justify-content: space-between;
	transition: background 0.25s ease;
	box-shadow:
		inset 0 0 0 1px rgba(255, 255, 255, 0.12),
		0 4px 16px rgba(0, 0, 0, 0.25);
}
.color-preview-hex {
	font-family: monospace;
	font-size: 1rem;
	font-weight: 700;
	letter-spacing: 0.08em;
	text-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}
.color-preview-dots {
	display: flex;
	gap: 0.375rem;
	align-items: center;
}
.dot {
	width: 0.75rem;
	height: 0.75rem;
	border-radius: 50%;
	display: inline-block;
	box-shadow: 0 1px 3px rgba(0, 0, 0, 0.35);
}

/* Presets */
.preset-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(2rem, 1fr));
	gap: 0.4rem;
}
.preset-swatch {
	width: 2rem;
	height: 2rem;
	border-radius: 0.5rem;
	border: 2px solid transparent;
	background: var(--swatch);
	cursor: pointer;
	outline: none;
	transition:
		transform 0.13s ease,
		box-shadow 0.13s ease,
		border-color 0.13s ease;

	&:hover {
		transform: scale(1.15);
		box-shadow: 0 3px 10px rgba(0, 0, 0, 0.3);
	}

	&.active {
		border-color: var(--color-contrast);
		transform: scale(1.12);
		box-shadow:
			0 0 0 2px var(--swatch),
			0 4px 14px var(--swatch);
	}
}

/* RGB Sliders */
.sliders {
	display: flex;
	flex-direction: column;
	gap: 0.6rem;
}

.slider-row {
	display: flex;
	align-items: center;
	gap: 0.625rem;
}

.slider-label {
	font-size: 0.7rem;
	font-weight: 700;
	letter-spacing: 0.04em;
	color: var(--color-secondary);
	width: 0.875rem;
	text-align: center;
	flex-shrink: 0;
}

.slider-track-wrap {
	flex: 1;
	position: relative;
	height: 1.25rem;
	display: flex;
	align-items: center;
}

.slider-track-bg {
	position: absolute;
	inset: 0;
	border-radius: 9999px;
	height: 0.5rem;
	top: 50%;
	transform: translateY(-50%);
	pointer-events: none;
}

.slider-input {
	position: relative;
	z-index: 1;
	width: 100%;
	-webkit-appearance: none;
	appearance: none;
	height: 0.5rem;
	background: transparent;
	cursor: pointer;
	outline: none;

	&::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 1.125rem;
		height: 1.125rem;
		border-radius: 50%;
		background: var(--color-contrast);
		border: 2px solid var(--color-raised-bg);
		box-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
		transition: transform 0.1s ease;

		&:hover {
			transform: scale(1.15);
		}
	}

	&::-moz-range-thumb {
		width: 1.125rem;
		height: 1.125rem;
		border-radius: 50%;
		background: var(--color-contrast);
		border: 2px solid var(--color-raised-bg);
		box-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
		cursor: pointer;
	}
}

.slider-number {
	width: 2.75rem;
	background: var(--color-bg-raised);
	border: 1px solid var(--color-surface-5);
	border-radius: 0.4rem;
	color: var(--color-contrast);
	font-size: 0.75rem;
	font-weight: 600;
	text-align: center;
	padding: 0.25rem 0;
	outline: none;
	-moz-appearance: textfield;

	&::-webkit-inner-spin-button,
	&::-webkit-outer-spin-button {
		display: none;
	}

	&:focus {
		border-color: var(--color-brand);
	}
}

/* Hex input */
.hex-row {
	display: flex;
	align-items: center;
	gap: 0.5rem;
	background: var(--color-bg-raised);
	border: 1px solid var(--color-surface-5);
	border-radius: 0.5rem;
	padding: 0.375rem 0.625rem;
	transition: border-color 0.15s ease;

	&:focus-within {
		border-color: var(--color-brand);
	}
}

.hex-label {
	font-size: 0.65rem;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: var(--color-secondary);
	flex-shrink: 0;
}

.hex-input {
	flex: 1;
	background: transparent;
	border: none;
	outline: none;
	color: var(--color-contrast);
	font-size: 0.875rem;
	font-weight: 600;
	font-family: monospace;
	letter-spacing: 0.05em;
	min-width: 0;
}
</style>
