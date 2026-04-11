import { defineStore } from 'pinia'

export const DEFAULT_FEATURE_FLAGS = {
	project_background: false,
	page_path: false,
	worlds_tab: false,
	worlds_in_home: true,
	servers_in_app: false,
	server_project_qa: false,
	i18n_debug: false,
}

export const THEME_OPTIONS = ['dark', 'light', 'oled', 'system'] as const

export const ACCENT_COLORS = [
	{ name: 'Green', value: '#1bd96a' },
	{ name: 'Blue', value: '#3b82f6' },
	{ name: 'Purple', value: '#8b5cf6' },
	{ name: 'Pink', value: '#ec4899' },
	{ name: 'Red', value: '#ef4444' },
	{ name: 'Orange', value: '#f97316' },
	{ name: 'Yellow', value: '#eab308' },
	{ name: 'Cyan', value: '#06b6d4' },
	{ name: 'Teal', value: '#14b8a6' },
	{ name: 'Rose', value: '#f43f5e' },
	{ name: 'Violet', value: '#a855f7' },
	{ name: 'Indigo', value: '#6366f1' },
] as const

export type ColorTheme = (typeof THEME_OPTIONS)[number]
export type AccentColor = string
export type FeatureFlag = keyof typeof DEFAULT_FEATURE_FLAGS
export type FeatureFlags = Record<FeatureFlag, boolean>

export type ThemeStore = {
	selectedTheme: ColorTheme
	accentColor: AccentColor
	customAccentColor: string
	advancedRendering: boolean
	toggleSidebar: boolean

	devMode: boolean
	featureFlags: FeatureFlags
}

export const DEFAULT_THEME_STORE: ThemeStore = {
	selectedTheme: 'dark',
	accentColor: '#1bd96a',
	customAccentColor: '',
	advancedRendering: true,
	toggleSidebar: false,

	devMode: false,
	featureFlags: DEFAULT_FEATURE_FLAGS,
}

export const useTheming = defineStore('themeStore', {
	state: () => DEFAULT_THEME_STORE,
	actions: {
		setThemeState(newTheme: ColorTheme) {
			if (THEME_OPTIONS.includes(newTheme)) {
				this.selectedTheme = newTheme
			} else {
				console.warn('Selected theme is not present. Check themeOptions.')
			}

			this.setThemeClass()
		},
		setThemeClass() {
			for (const theme of THEME_OPTIONS) {
				document.getElementsByTagName('html')[0].classList.remove(`${theme}-mode`)
			}

			let theme = this.selectedTheme
			if (this.selectedTheme === 'system') {
				const darkThemeMq = window.matchMedia('(prefers-color-scheme: dark)')
				if (darkThemeMq.matches) {
					theme = 'dark'
				} else {
					theme = 'light'
				}
			}

			document.getElementsByTagName('html')[0].classList.add(`${theme}-mode`)
			this.applyCustomAccent()
		},
		setAccentColor(color: AccentColor) {
			this.accentColor = color
			this.customAccentColor = color === '#1bd96a' ? '' : color
			this.applyCustomAccent()
		},
		setCustomAccentColor(color: string) {
			this.customAccentColor = color
			this.accentColor = color || '#1bd96a'
			this.applyCustomAccent()
		},
		applyAccentColor(color: AccentColor) {
			// Alias kept for backwards compatibility - delegates to applyCustomAccent
			this.accentColor = color
			this.customAccentColor = color === '#1bd96a' ? '' : color
			this.applyCustomAccent()
		},
		applyCustomAccent() {
			const root = document.documentElement
			const color = this.customAccentColor || this.accentColor || '#1bd96a'

			const r = parseInt(color.slice(1, 3), 16)
			const g = parseInt(color.slice(3, 5), 16)
			const b = parseInt(color.slice(5, 7), 16)

			// Convert RGB to HSL for smarter shade generation
			const rn = r / 255, gn = g / 255, bn = b / 255
			const max = Math.max(rn, gn, bn), min = Math.min(rn, gn, bn)
			const l = (max + min) / 2
			const d = max - min
			const s = d === 0 ? 0 : d / (1 - Math.abs(2 * l - 1))
			let h = 0
			if (d !== 0) {
				if (max === rn) h = ((gn - bn) / d) % 6
				else if (max === gn) h = (bn - rn) / d + 2
				else h = (rn - gn) / d + 4
				h = Math.round(h * 60)
				if (h < 0) h += 360
			}

			// HSL to RGB helper
			const hslToRgb = (hh: number, ss: number, ll: number) => {
				const c = (1 - Math.abs(2 * ll - 1)) * ss
				const x = c * (1 - Math.abs((hh / 60) % 2 - 1))
				const m = ll - c / 2
				let r1 = 0, g1 = 0, b1 = 0
				if (hh < 60) { r1 = c; g1 = x }
				else if (hh < 120) { r1 = x; g1 = c }
				else if (hh < 180) { g1 = c; b1 = x }
				else if (hh < 240) { g1 = x; b1 = c }
				else if (hh < 300) { r1 = x; b1 = c }
				else { r1 = c; b1 = x }
				return [Math.round((r1 + m) * 255), Math.round((g1 + m) * 255), Math.round((b1 + m) * 255)]
			}

			// Generate lighter and darker shades by adjusting lightness
			const lighterL = Math.min(0.95, l + 0.2)
			const darkerL = Math.max(0.05, l - 0.2)
			const [lr, lg, lb] = hslToRgb(h, s, lighterL)
			const [dr, dg, db] = hslToRgb(h, s, darkerL)

			root.style.setProperty('--color-brand', color)
			root.style.setProperty('--color-brand-highlight', `rgba(${r}, ${g}, ${b}, 0.15)`)
			root.style.setProperty('--color-brand-contrast', `rgba(${r}, ${g}, ${b}, 0.3)`)
			root.style.setProperty('--color-brand-secondary', `rgba(${r}, ${g}, ${b}, 0.08)`)
			root.style.setProperty('--color-brand-hover', `rgba(${r}, ${g}, ${b}, 0.2)`)
			root.style.setProperty('--color-brand-active', `rgba(${r}, ${g}, ${b}, 0.12)`)
			root.style.setProperty('--color-brand-disabled', `rgba(${r}, ${g}, ${b}, 0.4)`)
			root.style.setProperty('--color-brand-shadow', `rgba(${r}, ${g}, ${b}, 0.25)`)
			root.style.setProperty('--color-brand-text', `rgb(${lr}, ${lg}, ${lb})`)
			root.style.setProperty('--color-brand-dark', `rgb(${dr}, ${dg}, ${db})`)

			// Active nav button background uses brand-highlight; text should stay
			// contrast (white in dark mode) rather than inheriting the brand color.
			root.style.setProperty('--color-button-bg-selected', `rgba(${r}, ${g}, ${b}, 0.2)`)
			root.style.setProperty('--color-button-text-selected', 'var(--color-contrast)')

			// Do NOT override --brand-gradient-* variables — those control the sidebar
			// background and we want it to stay its default dark color regardless of accent.
		},
		getFeatureFlag(key: FeatureFlag) {
			return this.featureFlags[key] ?? DEFAULT_FEATURE_FLAGS[key]
		},
		getThemeOptions() {
			return THEME_OPTIONS
		},
	},
})
