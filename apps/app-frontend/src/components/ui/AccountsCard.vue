<template>
	<div
		v-if="mode !== 'isolated'"
		ref="button"
		class="button-base mt-2 px-3 py-2 bg-button-bg rounded-xl flex items-center gap-2"
		:class="{ expanded: mode === 'expanded' }"
		@click="toggleMenu"
	>
		<Avatar
			size="36px"
			:src="
				selectedAccount ? avatarUrl : 'https://launcher-files.modrinth.com/assets/steve_head.png'
			"
		/>
		<div class="flex flex-col w-full">
			<span>{{ selectedAccount ? selectedAccount.profile.name : 'Select account' }}</span>
			<span class="text-secondary text-xs">Minecraft account</span>
		</div>
		<DropdownIcon class="w-5 h-5 shrink-0" />
	</div>
	<transition name="fade">
		<Card
			v-if="showCard || mode === 'isolated'"
			ref="card"
			class="account-card"
			:class="{ expanded: mode === 'expanded', isolated: mode === 'isolated' }"
		>
			<div v-if="selectedAccount" class="account-row selected-row">
				<Avatar size="40px" :src="avatarUrl" class="account-avatar" />
				<div class="account-info">
					<span class="account-name">{{ selectedAccount.profile.name }}</span>
					<span class="account-label">Selected</span>
				</div>
				<Button v-tooltip="'Log out'" icon-only color="raised" class="trash-btn" @click="logout(selectedAccount.profile.id)">
					<TrashIcon />
				</Button>
			</div>
			<div v-else class="account-row logged-out-row">
				<span class="account-name">Not signed in</span>
				<Button v-tooltip="'Log in'" :disabled="loginDisabled" icon-only color="primary" @click="login()">
					<LogInIcon v-if="!loginDisabled" />
					<SpinnerIcon v-else class="animate-spin" />
				</Button>
			</div>
			<div v-if="displayAccounts.length > 0" class="account-group">
				<div v-for="account in displayAccounts" :key="account.profile.id" class="account-row other-row" @click="setAccount(account)">
					<Avatar size="40px" :src="getAccountAvatarUrl(account)" class="account-avatar" />
					<div class="account-info">
						<span class="account-name">{{ account.profile.name }}</span>
					</div>
					<Button v-tooltip="'Log out'" icon-only color="raised" class="trash-btn" @click.stop="logout(account.profile.id)">
						<TrashIcon />
					</Button>
				</div>
			</div>
			<Button class="action-btn" v-if="accounts.length > 0" @click="login()">
				<PlusIcon />
				Add Microsoft Account
			</Button>
			<Button class="action-btn" v-else @click="login()">
				<LogInIcon v-if="!loginDisabled" />
				<SpinnerIcon v-else class="animate-spin" />
				Sign in with Microsoft
			</Button>
			<Button class="action-btn" @click="showOfflineInput = !showOfflineInput">
				<PlusIcon />
				Add Offline Account
			</Button>
		</Card>
	</transition>

	<!-- Centered offline account modal -->
	<teleport to="body">
		<transition name="modal-fade">
			<div v-if="showOfflineInput" class="offline-modal-overlay" @click.self="showOfflineInput = false">
				<div class="offline-modal">
					<h3 class="offline-modal-title">Add offline account</h3>
					<StyledInput
						v-model="offlineUsername"
						placeholder="Username"
						autofocus
						@keyup.enter="addOfflineAccount"
						@keyup.esc="showOfflineInput = false"
					/>
					<div class="offline-modal-actions">
						<Button color="brand" @click="addOfflineAccount"> Create </Button>
						<Button @click="showOfflineInput = false"> Cancel </Button>
					</div>
				</div>
			</div>
		</transition>
	</teleport>
</template>

<script setup>
import { DropdownIcon, LogInIcon, PlusIcon, SpinnerIcon, TrashIcon } from '@fernite/assets'
import { Avatar, Button, Card, injectNotificationManager, StyledInput } from '@fernite/ui'
import { computed, onBeforeUnmount, onMounted, onUnmounted, ref } from 'vue'

import { trackEvent } from '@/helpers/analytics'
import {
	create_offline_user,
	get_default_user,
	login as login_flow,
	remove_user,
	set_default_user,
	users,
} from '@/helpers/auth'
import { process_listener } from '@/helpers/events'
import { getPlayerHeadUrl } from '@/helpers/rendering/batch-skin-renderer.ts'
import { get_available_skins } from '@/helpers/skins'
import { handleSevereError } from '@/store/error.js'

const { handleError } = injectNotificationManager()

defineProps({
	mode: {
		type: String,
		required: true,
		default: 'normal',
	},
})

const emit = defineEmits(['change'])

const accounts = ref({})
const loginDisabled = ref(false)
const defaultUser = ref()
const equippedSkin = ref(null)
const headUrlCache = ref(new Map())
const showOfflineInput = ref(false)
const offlineUsername = ref('')

async function refreshValues() {
	defaultUser.value = await get_default_user().catch(handleError)
	accounts.value = await users().catch(handleError)

	try {
		const skins = await get_available_skins()
		equippedSkin.value = skins.find((skin) => skin.is_equipped)

		if (equippedSkin.value) {
			try {
				const headUrl = await getPlayerHeadUrl(equippedSkin.value)
				headUrlCache.value.set(equippedSkin.value.texture_key, headUrl)
			} catch (error) {
				console.warn('Failed to get head render for equipped skin:', error)
			}
		}
	} catch {
		equippedSkin.value = null
	}
}

function setLoginDisabled(value) {
	loginDisabled.value = value
}

defineExpose({
	refreshValues,
	setLoginDisabled,
	loginDisabled,
})
await refreshValues()

const displayAccounts = computed(() =>
	accounts.value.filter((account) => defaultUser.value !== account.profile.id),
)

const avatarUrl = computed(() => {
	if (equippedSkin.value?.texture_key) {
		const cachedUrl = headUrlCache.value.get(equippedSkin.value.texture_key)
		if (cachedUrl) {
			return cachedUrl
		}
		return `https://mc-heads.net/avatar/${equippedSkin.value.texture_key}/128`
	}
	if (selectedAccount.value?.profile?.id) {
		return `https://mc-heads.net/avatar/${selectedAccount.value.profile.id}/128`
	}
	return 'https://launcher-files.modrinth.com/assets/steve_head.png'
})

function getAccountAvatarUrl(account) {
	if (
		account.profile.id === selectedAccount.value?.profile?.id &&
		equippedSkin.value?.texture_key
	) {
		const cachedUrl = headUrlCache.value.get(equippedSkin.value.texture_key)
		if (cachedUrl) {
			return cachedUrl
		}
	}
	return `https://mc-heads.net/avatar/${account.profile.id}/128`
}

const selectedAccount = computed(() =>
	accounts.value.find((account) => account.profile.id === defaultUser.value),
)

async function setAccount(account) {
	defaultUser.value = account.profile.id
	await set_default_user(account.profile.id).catch(handleError)
	emit('change')
}

async function login() {
	loginDisabled.value = true
	const loggedIn = await login_flow().catch(handleSevereError)

	if (loggedIn) {
		await setAccount(loggedIn)
		await refreshValues()
	}

	trackEvent('AccountLogIn')
	loginDisabled.value = false
}

const logout = async (id) => {
	await remove_user(id).catch(handleError)
	await refreshValues()
	if (!selectedAccount.value && accounts.value.length > 0) {
		await setAccount(accounts.value[0])
		await refreshValues()
	} else {
		emit('change')
	}
	trackEvent('AccountLogOut')
}

async function addOfflineAccount() {
	const username = offlineUsername.value.trim()
	if (!username || loginDisabled.value) return

	loginDisabled.value = true
	try {
		const offlineAccount = await create_offline_user(username).catch(handleError)
		if (offlineAccount) {
			await setAccount(offlineAccount)
			await refreshValues()
		}
	} finally {
		loginDisabled.value = false
		offlineUsername.value = ''
		showOfflineInput.value = false
	}
}

const showCard = ref(false)
const card = ref(null)
const button = ref(null)
const handleClickOutside = (event) => {
	const elements = document.elementsFromPoint(event.clientX, event.clientY)
	if (
		card.value &&
		card.value.$el !== event.target &&
		!elements.includes(card.value.$el) &&
		!button.value.contains(event.target)
	) {
		toggleMenu(false)
	}
}

function toggleMenu(override = true) {
	if (showCard.value || !override) {
		showCard.value = false
	} else {
		showCard.value = true
	}
}

const unlisten = await process_listener(async (e) => {
	if (e.event === 'launched') {
		await refreshValues()
	}
})

onMounted(() => {
	window.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
	window.removeEventListener('click', handleClickOutside)
})

onUnmounted(() => {
	unlisten()
})
</script>

<style scoped lang="scss">
.account-card {
	position: sticky;
	display: flex;
	flex-direction: column;
	margin-top: 0.5rem;
	right: 2rem;
	z-index: 11;
	gap: 0.4rem;
	padding: 0.75rem;
	border: 1px solid var(--color-divider);
	// width: 260px;
	user-select: none;
	-ms-user-select: none;
	-webkit-user-select: none;
	max-height: calc(100vh - 300px);
	overflow-y: auto;

	&::-webkit-scrollbar-track {
		border-top-right-radius: 1rem;
		border-bottom-right-radius: 1rem;
	}

	&::-webkit-scrollbar {
		border-top-right-radius: 1rem;
		border-bottom-right-radius: 1rem;
	}

	&.hidden {
		display: none;
	}

	&.expanded {
		left: 13.5rem;
	}

	&.isolated {
		position: relative;
		left: 0;
		top: 0;
	}
}

.account-row {
	display: flex;
	flex-direction: row;
	align-items: center;
	gap: 0.75rem;
	width: 100%;
	padding: 0.625rem 0.75rem;
	border-radius: var(--radius-lg);
}

.selected-row {
	background: var(--color-brand-highlight);
}

.logged-out-row {
	background: var(--color-bg);
	justify-content: space-between;
}

.other-row {
	background: var(--color-raised-bg);
	cursor: pointer;

	&:hover {
		filter: brightness(1.1);
	}
}

.account-avatar {
	flex-shrink: 0;
	border-radius: 4px;
}

.account-info {
	display: flex;
	flex-direction: column;
	flex: 1;
	min-width: 0;
}

.account-name {
	font-weight: 700;
	font-size: 1rem;
	color: var(--color-contrast);
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
	letter-spacing: 0.02em;
}

.account-label {
	font-size: 0.8rem;
	color: var(--color-contrast);
	opacity: 0.75;
}

.trash-btn {
	flex-shrink: 0;
	margin-left: auto;
}

.account-group {
	width: 100%;
	display: flex;
	flex-direction: column;
	gap: 0.4rem;
}

.action-btn {
	width: 100% !important;
	justify-content: flex-start !important;
	background: var(--color-raised-bg) !important;
	border-radius: var(--radius-lg) !important;
	padding: 0.6rem 0.75rem !important;
	font-size: 0.875rem !important;
	font-weight: 500 !important;
	gap: 0.5rem !important;
	box-shadow: none !important;
}

.fade-enter-active,
.fade-leave-active {
	transition:
		opacity 0.25s ease,
		translate 0.25s ease,
		scale 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
	translate: 0 -2rem;
	scale: 0.9;
}

.avatar-button {
	display: flex;
	align-items: center;
	gap: 0.5rem;
	color: var(--color-base);
	background-color: var(--color-button-bg);
	border-radius: var(--radius-md);
	width: 100%;
	padding: 0.5rem 0.75rem;
	text-align: left;

	&.expanded {
		border: 1px solid var(--color-divider);
		padding: 1rem;
	}
}

.avatar-text {
	margin: auto 0 auto 0.25rem;
	display: flex;
	flex-direction: column;
}

.text {
	width: 6rem;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}

.accounts-text {
	display: flex;
	align-items: center;
	gap: 0.25rem;
	margin: 0;
}

.qr-code {
	background-color: white !important;
	border-radius: var(--radius-md);
}

.modal-body {
	display: flex;
	flex-direction: row;
	gap: var(--gap-lg);
	align-items: center;
	padding: var(--gap-xl);

	.modal-text {
		display: flex;
		flex-direction: column;
		gap: var(--gap-sm);
		width: 100%;

		h2,
		p {
			margin: 0;
		}

		.code-text {
			display: flex;
			flex-direction: row;
			gap: var(--gap-xs);
			align-items: center;

			.code {
				background-color: var(--color-bg);
				border-radius: var(--radius-md);
				border: solid 1px var(--color-button-bg);
				font-family: var(--mono-font);
				letter-spacing: var(--gap-md);
				color: var(--color-contrast);
				font-size: 2rem;
				font-weight: bold;
				padding: var(--gap-sm) 0 var(--gap-sm) var(--gap-md);
			}

			.btn {
				width: 2.5rem;
				height: 2.5rem;
			}
		}
	}
}

.button-row {
	display: flex;
	flex-direction: row;
}

.modal {
	position: absolute;
}

.code {
	color: var(--color-brand);
	padding: 0.05rem 0.1rem;
	// row not column
	display: flex;

	.card {
		background: var(--color-base);
		color: var(--color-contrast);
		padding: 0.5rem 1rem;
	}
}

.offline-modal-overlay {
	position: fixed;
	inset: 0;
	z-index: 1000;
	display: flex;
	align-items: center;
	justify-content: center;
	background: rgba(0, 0, 0, 0.5);
	backdrop-filter: blur(2px);
}

.offline-modal {
	background: var(--color-raised-bg);
	border: 1px solid var(--color-divider);
	border-radius: var(--radius-lg);
	padding: 1.5rem;
	display: flex;
	flex-direction: column;
	gap: 0.75rem;
	min-width: 280px;
	box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.offline-modal-title {
	margin: 0;
	font-size: 1rem;
	font-weight: 600;
	color: var(--color-contrast);
}

.offline-modal-actions {
	display: flex;
	gap: 0.5rem;
}

.modal-fade-enter-active,
.modal-fade-leave-active {
	transition: opacity 0.2s ease;

	.offline-modal {
		transition: transform 0.2s ease, opacity 0.2s ease;
	}
}

.modal-fade-enter-from,
.modal-fade-leave-to {
	opacity: 0;

	.offline-modal {
		transform: scale(0.95) translateY(-8px);
		opacity: 0;
	}
}
</style>
