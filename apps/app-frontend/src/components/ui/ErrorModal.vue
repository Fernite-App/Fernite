<script setup>
import {
	CheckIcon,
	CopyIcon,
	DropdownIcon,
	HammerIcon,
	LogInIcon,
	UpdatedIcon,
	WrenchIcon,
	XIcon,
} from '@fernite/assets'
import { ButtonStyled, Collapsible, injectNotificationManager } from '@fernite/ui'
import { computed, ref } from 'vue'

import { ChatIcon } from '@/assets/icons'
import ModalWrapper from '@/components/ui/modal/ModalWrapper.vue'
import { trackEvent } from '@/helpers/analytics'
import { login as login_flow, set_default_user } from '@/helpers/auth.js'
import { install } from '@/helpers/profile.js'
import { cancel_directory_change } from '@/helpers/settings.ts'
import { handleSevereError } from '@/store/error.js'

const { handleError } = injectNotificationManager()

const errorModal = ref()
const error = ref()
const closable = ref(true)
const errorCollapsed = ref(false)

const title = ref('An error occurred')
const errorType = ref('unknown')
const supportLink = ref('https://support.modrinth.com')
const metadata = ref({})

defineExpose({
	async show(errorVal, context, canClose = true, source = null) {
		console.log(errorVal, context, canClose, source)
		closable.value = canClose

		if (errorVal.message && errorVal.message.includes('Minecraft authentication error:')) {
			title.value = 'Unable to sign in to Minecraft'
			errorType.value = 'minecraft_auth'
			supportLink.value =
				'https://support.modrinth.com/en/articles/9038231-minecraft-sign-in-issues'

			if (
				errorVal.message.includes('existing connection was forcibly closed') ||
				errorVal.message.includes('error sending request for url')
			) {
				metadata.value.network = true
			}
			if (errorVal.message.includes('because the target machine actively refused it')) {
				metadata.value.hostsFile = true
			}
		} else if (errorVal.message && errorVal.message.includes('User is not logged in')) {
			title.value = 'Sign in to Minecraft'
			errorType.value = 'minecraft_sign_in'
			supportLink.value = 'https://support.modrinth.com'
		} else if (errorVal.message && errorVal.message.includes('Move directory error:')) {
			title.value = 'Could not change app directory'
			errorType.value = 'directory_move'
			supportLink.value = 'https://support.modrinth.com'

			if (errorVal.message.includes('directory is not writeable')) {
				metadata.value.readOnly = true
			}

			if (errorVal.message.includes('Not enough space')) {
				metadata.value.notEnoughSpace = true
			}
		} else if (errorVal.message && errorVal.message.includes('No loader version selected for')) {
			title.value = 'No loader selected'
			errorType.value = 'no_loader_version'
			supportLink.value = 'https://support.modrinth.com'
			metadata.value.profilePath = context.profilePath
		} else if (
			errorVal.message &&
			(errorVal.message.includes('Java not found') ||
				errorVal.message.includes('No Java installation') ||
				errorVal.message.includes('java executable'))
		) {
			title.value = 'Java not found'
			errorType.value = 'java_not_found'
			supportLink.value = 'https://support.modrinth.com'
		} else if (
			errorVal.message &&
			(errorVal.message.includes('address already in use') ||
				errorVal.message.includes('port') ||
				errorVal.message.includes('EADDRINUSE'))
		) {
			title.value = 'Port already in use'
			errorType.value = 'port_in_use'
			supportLink.value = 'https://support.modrinth.com'
		} else if (
			errorVal.message &&
			(errorVal.message.includes('OutOfMemoryError') ||
				errorVal.message.includes('memory') ||
				errorVal.message.includes('heap'))
		) {
			title.value = 'Out of memory'
			errorType.value = 'out_of_memory'
			supportLink.value = 'https://support.modrinth.com'
		} else if (
			errorVal.message &&
			(errorVal.message.includes('connection timed out') ||
				errorVal.message.includes('connection refused') ||
				errorVal.message.includes('network is unreachable') ||
				errorVal.message.includes('ENOTFOUND') ||
				errorVal.message.includes('ECONNREFUSED'))
		) {
			title.value = 'Network error'
			errorType.value = 'network_error'
			supportLink.value = 'https://support.modrinth.com'
		} else if (source === 'state_init') {
			title.value = 'Error initializing Fernite Launcher'
			errorType.value = 'state_init'
			supportLink.value = 'https://support.modrinth.com'
		} else {
			title.value = 'An error occurred'
			errorType.value = 'unknown'
			supportLink.value = 'https://support.modrinth.com'
			metadata.value = {}
		}

		error.value = errorVal
		errorModal.value.show()
	},
})

const loadingMinecraft = ref(false)
async function loginMinecraft() {
	try {
		loadingMinecraft.value = true
		const loggedIn = await login_flow()

		if (loggedIn) {
			await set_default_user(loggedIn.profile.id).catch(handleError)
		}

		await trackEvent('AccountLogIn', { source: 'ErrorModal' })
		loadingMinecraft.value = false
		errorModal.value.hide()
	} catch (err) {
		loadingMinecraft.value = false
		handleSevereError(err)
	}
}

async function cancelDirectoryChange() {
	try {
		await cancel_directory_change()
		window.location.reload()
	} catch (err) {
		handleError(err)
	}
}

function retryDirectoryChange() {
	window.location.reload()
}

const loadingRepair = ref(false)
async function repairInstance() {
	loadingRepair.value = true
	try {
		await install(metadata.value.profilePath, false)
		errorModal.value.hide()
	} catch (err) {
		handleSevereError(err)
	}
	loadingRepair.value = false
}

const hasDebugInfo = computed(
	() =>
		errorType.value === 'directory_move' ||
		errorType.value === 'minecraft_auth' ||
		errorType.value === 'state_init' ||
		errorType.value === 'no_loader_version' ||
		errorType.value === 'java_not_found' ||
		errorType.value === 'port_in_use' ||
		errorType.value === 'out_of_memory' ||
		errorType.value === 'network_error',
)

const debugInfo = computed(() => error.value.message ?? error.value ?? 'No error message.')

const copied = ref(false)

async function copyToClipboard(text) {
	await navigator.clipboard.writeText(text)
	copied.value = true
	setTimeout(() => {
		copied.value = false
	}, 3000)
}
</script>

<template>
	<ModalWrapper ref="errorModal" :header="title" :closable="closable">
		<div class="modal-body max-w-[550px]">
			<div class="markdown-body">
				<template v-if="errorType === 'minecraft_auth'">
					<template v-if="metadata.network">
						<h3>Network issues</h3>
						<p>
							It looks like there were issues with the Fernite Launcher connecting to Microsoft's
							servers. This is often the result of a poor connection, so we recommend trying again
							to see if it works. If issues continue to persist, follow the steps in
							<a
								href="https://support.modrinth.com/en/articles/9038231-minecraft-sign-in-issues#h_e71a5f805f"
							>
								our support article
							</a>
							to troubleshoot.
						</p>
					</template>
					<template v-else-if="metadata.hostsFile">
						<h3>Network issues</h3>
						<p>
							The Fernite App tried to connect to Microsoft / Xbox / Minecraft services, but the
							remote server rejected the connection. This may indicate that these services are
							blocked by the hosts file. Please visit
							<a
								href="https://support.modrinth.com/en/articles/9038231-minecraft-sign-in-issues#h_d694a29256"
							>
								our support article
							</a>
							for steps on how to fix the issue.
						</p>
					</template>
					<template v-else>
						<h3>Try another Microsoft account</h3>
						<p>
							Double check you've signed in with the right account. You may own Minecraft on a
							different Microsoft account.
						</p>
						<div class="cta-button">
							<button class="btn btn-primary" :disabled="loadingMinecraft" @click="loginMinecraft">
								<LogInIcon /> Try another account
							</button>
						</div>
						<h3>Using PC Game Pass, coming from Bedrock, or just bought the game?</h3>
						<p>
							Try signing in with the
							<a href="https://www.minecraft.net/en-us/download">official Minecraft Launcher</a>
							first. Once you're done, come back here and sign in!
						</p>
					</template>
					<div class="cta-button">
						<button class="btn btn-primary" :disabled="loadingMinecraft" @click="loginMinecraft">
							<LogInIcon /> Try signing in again
						</button>
					</div>
				</template>
				<template v-if="errorType === 'directory_move'">
					<template v-if="metadata.readOnly">
						<h3>Change directory permissions</h3>
						<p>
							It looks like the Fernite App is unable to write to the directory you selected. Please
							adjust the permissions of the directory and try again or cancel the directory change.
						</p>
					</template>
					<template v-else-if="metadata.notEnoughSpace">
						<h3>Not enough space</h3>
						<p>
							It looks like there is not enough space on the disk containing the directory you
							selected. Please free up some space and try again or cancel the directory change.
						</p>
					</template>
					<template v-else>
						<p>
							The Fernite App is unable to migrate to the new directory you selected. Please contact
							support for help or cancel the directory change.
						</p>
					</template>

					<div class="cta-button">
						<button class="btn" @click="retryDirectoryChange">
							<UpdatedIcon /> Retry directory change
						</button>
						<button class="btn btn-danger" @click="cancelDirectoryChange">
							<XIcon /> Cancel directory change
						</button>
					</div>
				</template>
				<div v-else-if="errorType === 'minecraft_sign_in'">
					<p>
						To play this instance, you must sign in through Microsoft below. If you don't have a
						Minecraft account, you can purchase the game on the
						<a href="https://www.minecraft.net/en-us/store/minecraft-java-bedrock-edition-pc"
							>Minecraft website</a
						>.
					</p>
					<div class="cta-button">
						<button class="btn btn-primary" :disabled="loadingMinecraft" @click="loginMinecraft">
							<LogInIcon /> Sign in to Minecraft
						</button>
					</div>
				</div>
				<template v-else-if="errorType === 'state_init'">
					<p>
						Fernite App failed to load correctly. This may be because of a corrupted file, or
						because the app is missing crucial files.
					</p>
					<p>You may be able to fix it through one of the following ways:</p>
					<ul>
						<li>Ensuring you are connected to the internet, then try restarting the app.</li>
						<li>Redownloading the app.</li>
					</ul>
				</template>
				<template v-else-if="errorType === 'no_loader_version'">
					<p>The Fernite App failed to find the loader version for this instance.</p>
					<p>To resolve this, you need to repair the instance. Click the button below to do so.</p>
					<div class="cta-button">
						<button class="btn btn-primary" :disabled="loadingRepair" @click="repairInstance">
							<HammerIcon /> Repair instance
						</button>
					</div>
				</template>
				<template v-else-if="errorType === 'java_not_found'">
					<h3>Java is not installed</h3>
					<p>
						The launcher could not find a valid Java installation on your system. This instance
						requires Java to run.
					</p>
					<h3>How to fix this</h3>
					<ul>
						<li>
							Go to <strong>Settings → Java Installations</strong> and click
							<strong>Auto-detect</strong>
							to let the launcher find Java automatically.
						</li>
						<li>If auto-detect fails, manually select a Java installation from the list.</li>
						<li>
							If you don't have Java installed, you can download it from
							<a href="https://adoptium.net" target="_blank">Adoptium</a>.
						</li>
					</ul>
					<div class="cta-button">
						<button class="btn btn-primary" @click="errorModal.hide()">
							<WrenchIcon /> Open Java settings
						</button>
					</div>
				</template>
				<template v-else-if="errorType === 'port_in_use'">
					<h3>Port already in use</h3>
					<p>
						Another process is already using the port that this instance needs. This can happen if
						you have another Minecraft instance or server running.
					</p>
					<h3>How to fix this</h3>
					<ul>
						<li>Close any other running Minecraft instances or servers.</li>
						<li>Restart your computer to clear any stuck processes.</li>
						<li>Check your task manager for any lingering Java processes.</li>
					</ul>
				</template>
				<template v-else-if="errorType === 'out_of_memory'">
					<h3>Out of memory</h3>
					<p>
						Minecraft ran out of allocated memory. This is common for large modpacks or when too
						many mods are installed.
					</p>
					<h3>How to fix this</h3>
					<ul>
						<li>
							Go to <strong>Settings → Default instance options</strong> and increase the allocated
							RAM.
						</li>
						<li>For large modpacks, we recommend allocating at least 6-8 GB of RAM.</li>
						<li>Remove any unnecessary mods to reduce memory usage.</li>
					</ul>
					<div class="cta-button">
						<button class="btn btn-primary" @click="errorModal.hide()">
							<WrenchIcon /> Open memory settings
						</button>
					</div>
				</template>
				<template v-else-if="errorType === 'network_error'">
					<h3>Network connection error</h3>
					<p>
						The launcher was unable to connect to the required servers. This is often caused by a
						poor internet connection or firewall settings.
					</p>
					<h3>How to fix this</h3>
					<ul>
						<li>Check your internet connection and try again.</li>
						<li>Disable any VPN or proxy that may be blocking the connection.</li>
						<li>Add the Fernite App to your firewall's allow list.</li>
						<li>Try restarting the launcher.</li>
					</ul>
					<div class="cta-button">
						<button class="btn btn-primary" @click="window.location.reload()">
							<UpdatedIcon /> Restart launcher
						</button>
					</div>
				</template>
				<template v-else>
					{{ debugInfo }}
				</template>
				<template v-if="hasDebugInfo">
					<div class="w-full h-[1px] bg-surface-5 mb-3"></div>
					<p>
						If nothing is working and you need help, visit
						<a :href="supportLink">our support page</a>
						and start a chat using the widget in the bottom right and we will be more than happy to
						assist! Make sure to provide the following debug information to the agent:
					</p>
				</template>
			</div>
			<div class="flex items-center gap-2">
				<ButtonStyled>
					<a :href="supportLink" @click="errorModal.hide()"><ChatIcon /> Get support</a>
				</ButtonStyled>
				<ButtonStyled v-if="closable">
					<button @click="errorModal.hide()"><XIcon /> Close</button>
				</ButtonStyled>
			</div>
			<template v-if="hasDebugInfo">
				<div class="flex flex-col gap-2">
					<div class="w-full h-[1px] bg-surface-5"></div>

					<div class="overflow-clip">
						<button
							class="flex items-center justify-between w-full bg-transparent border-0 py-4 cursor-pointer"
							@click="errorCollapsed = !errorCollapsed"
						>
							<span class="flex items-center gap-2 text-contrast font-extrabold m-0">
								<WrenchIcon class="h-4 w-4" />
								Debug information
							</span>
							<DropdownIcon
								class="h-5 w-5 text-secondary transition-transform"
								:class="{ 'rotate-180': !errorCollapsed }"
							/>
						</button>
						<Collapsible :collapsed="errorCollapsed">
							<div
								class="p-3 bg-surface-2 rounded-2xl text-xs grid grid-cols-[1fr_auto] max-w-full items-start"
							>
								<div
									class="m-0 p-0 rounded-none bg-transparent text-sm font-mono break-words overflow-auto"
								>
									{{ debugInfo }}
								</div>
								<ButtonStyled circular>
									<button
										v-tooltip="'Copy debug info'"
										:disabled="copied"
										@click="copyToClipboard(debugInfo)"
									>
										<template v-if="copied"> <CheckIcon class="text-green" /> </template>
										<template v-else> <CopyIcon /> </template>
									</button>
								</ButtonStyled>
							</div>
						</Collapsible>
					</div>
				</div>
			</template>
		</div>
	</ModalWrapper>
</template>

<style>
.light-mode {
	--color-orange-bg: rgba(255, 163, 71, 0.2);
}

.dark-mode,
.oled-mode {
	--color-orange-bg: rgba(224, 131, 37, 0.2);
}
</style>

<style scoped lang="scss">
.cta-button {
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 0.5rem;
	gap: 0.5rem;
}

.warning-banner {
	display: flex;
	flex-direction: column;
	gap: 0.5rem;
	padding: var(--gap-lg);
	background-color: var(--color-orange-bg);
	border: 2px solid var(--color-orange);
	border-radius: var(--radius-md);
	margin-bottom: 1rem;
}

.warning-banner__title {
	display: flex;
	align-items: center;
	gap: 0.5rem;
	font-weight: 700;

	svg {
		color: var(--color-orange);
		height: 1.5rem;
		width: 1.5rem;
	}
}

.modal-body {
	display: flex;
	flex-direction: column;
	gap: var(--gap-md);
}

.markdown-body {
	overflow: auto;
}
</style>
