<template>
	<div v-if="!hidden" class="splash-screen dark" :class="{ 'fade-out': doneLoading }">
		<div v-if="os !== 'MacOS'" class="app-buttons">
			<button
				class="btn icon-only transparent"
				icon-only
				@click="() => getCurrentWindow().minimize()"
			>
				<MinimizeIcon />
			</button>
			<button class="btn icon-only transparent" @click="() => getCurrentWindow().toggleMaximize()">
				<MaximizeIcon />
			</button>
			<button class="btn icon-only transparent" @click="handleClose">
				<XIcon />
			</button>
		</div>
		<div class="app-logo-wrapper" data-tauri-drag-region>
			<svg
				class="app-logo"
				width="1032"
				height="149"
				viewBox="0 0 1032 149"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M233.9 119.1V60.3H215V44.7H233.9V30.9C233.9 25.8 235.4 21.75 238.4 18.75C241.5 15.65 245.5 14.1 250.4 14.1H269.9V29.7H257C254.2 29.7 252.8 31.2 252.8 34.2V44.7H272.3V60.3H252.8V119.1H233.9Z"
					fill="white"
				/>
				<path
					d="M321.342 121.2C313.942 121.2 307.392 119.65 301.692 116.55C296.092 113.35 291.692 108.9 288.492 103.2C285.392 97.4 283.842 90.6 283.842 82.8V81C283.842 73.2 285.392 66.45 288.492 60.75C291.592 54.95 295.942 50.5 301.542 47.4C307.142 44.2 313.642 42.6 321.042 42.6C328.342 42.6 334.692 44.25 340.092 47.55C345.492 50.75 349.692 55.25 352.692 61.05C355.692 66.75 357.192 73.4 357.192 81V87.45H303.042C303.242 92.55 305.142 96.7 308.742 99.9C312.342 103.1 316.742 104.7 321.942 104.7C327.242 104.7 331.142 103.55 333.642 101.25C336.142 98.95 338.042 96.4 339.342 93.6L354.792 101.7C353.392 104.3 351.342 107.15 348.642 110.25C346.042 113.25 342.542 115.85 338.142 118.05C333.742 120.15 328.142 121.2 321.342 121.2ZM303.192 73.35H337.992C337.592 69.05 335.842 65.6 332.742 63C329.742 60.4 325.792 59.1 320.892 59.1C315.792 59.1 311.742 60.4 308.742 63C305.742 65.6 303.892 69.05 303.192 73.35Z"
					fill="white"
				/>
				<path
					d="M377.014 119.1V44.7H395.614V53.1H398.314C399.414 50.1 401.214 47.9 403.714 46.5C406.314 45.1 409.314 44.4 412.714 44.4H421.714V61.2H412.414C407.614 61.2 403.664 62.5 400.564 65.1C397.464 67.6 395.914 71.5 395.914 76.8V119.1H377.014Z"
					fill="white"
				/>
				<path
					d="M439.487 119.1V44.7H458.087V54.45H460.787C461.987 51.85 464.237 49.4 467.537 47.1C470.837 44.7 475.837 43.5 482.537 43.5C488.337 43.5 493.387 44.85 497.687 47.55C502.087 50.15 505.487 53.8 507.887 58.5C510.287 63.1 511.487 68.5 511.487 74.7V119.1H492.587V76.2C492.587 70.6 491.187 66.4 488.387 63.6C485.687 60.8 481.787 59.4 476.687 59.4C470.887 59.4 466.387 61.35 463.187 65.25C459.987 69.05 458.387 74.4 458.387 81.3V119.1H439.487Z"
					fill="white"
				/>
				<path
					d="M534.918 119.1V44.7H553.818V119.1H534.918ZM544.368 36C540.968 36 538.068 34.9 535.668 32.7C533.368 30.5 532.218 27.6 532.218 24C532.218 20.4 533.368 17.5 535.668 15.3C538.068 13.1 540.968 12 544.368 12C547.868 12 550.768 13.1 553.068 15.3C555.368 17.5 556.518 20.4 556.518 24C556.518 27.6 555.368 30.5 553.068 32.7C550.768 34.9 547.868 36 544.368 36Z"
					fill="white"
				/>
				<path
					d="M606.262 119.1C601.362 119.1 597.362 117.6 594.262 114.6C591.262 111.5 589.762 107.4 589.762 102.3V60.3H571.162V44.7H589.762V21.6H608.662V44.7H629.062V60.3H608.662V99C608.662 102 610.062 103.5 612.862 103.5H627.262V119.1H606.262Z"
					fill="white"
				/>
				<path
					d="M680.434 121.2C673.034 121.2 666.484 119.65 660.784 116.55C655.184 113.35 650.784 108.9 647.584 103.2C644.484 97.4 642.934 90.6 642.934 82.8V81C642.934 73.2 644.484 66.45 647.584 60.75C650.684 54.95 655.034 50.5 660.634 47.4C666.234 44.2 672.734 42.6 680.134 42.6C687.434 42.6 693.784 44.25 699.184 47.55C704.584 50.75 708.784 55.25 711.784 61.05C714.784 66.75 716.284 73.4 716.284 81V87.45H662.134C662.334 92.55 664.234 96.7 667.834 99.9C671.434 103.1 675.834 104.7 681.034 104.7C686.334 104.7 690.234 103.55 692.734 101.25C695.234 98.95 697.134 96.4 698.434 93.6L713.884 101.7C712.484 104.3 710.434 107.15 707.734 110.25C705.134 113.25 701.634 115.85 697.234 118.05C692.834 120.15 687.234 121.2 680.434 121.2ZM662.284 73.35H697.084C696.684 69.05 694.934 65.6 691.834 63C688.834 60.4 684.884 59.1 679.984 59.1C674.884 59.1 670.834 60.4 667.834 63C664.834 65.6 662.984 69.05 662.284 73.35Z"
					fill="white"
				/>
				<path
					d="M787.9 120.6C782.6 120.6 777.85 119.7 773.65 117.9C769.45 116 766.1 113.3 763.6 109.8C761.2 106.2 760 101.85 760 96.75C760 91.65 761.2 87.4 763.6 84C766.1 80.5 769.5 77.9 773.8 76.2C778.2 74.4 783.2 73.5 788.8 73.5H809.2V69.3C809.2 65.8 808.1 62.95 805.9 60.75C803.7 58.45 800.2 57.3 795.4 57.3C790.7 57.3 787.2 58.4 784.9 60.6C782.6 62.7 781.1 65.45 780.4 68.85L763 63C764.2 59.2 766.1 55.75 768.7 52.65C771.4 49.45 774.95 46.9 779.35 45C783.85 43 789.3 42 795.7 42C805.5 42 813.25 44.45 818.95 49.35C824.65 54.25 827.5 61.35 827.5 70.65V98.4C827.5 101.4 828.9 102.9 831.7 102.9H837.7V118.5H825.1C821.4 118.5 818.35 117.6 815.95 115.8C813.55 114 812.35 111.6 812.35 108.6V108.45H809.5C809.1 109.65 808.2 111.25 806.8 113.25C805.4 115.15 803.2 116.85 800.2 118.35C797.2 119.85 793.1 120.6 787.9 120.6ZM791.2 105.3C796.5 105.3 800.8 103.85 804.1 100.95C807.5 97.95 809.2 94 809.2 89.1V87.6H790.15C786.65 87.6 783.9 88.35 781.9 89.85C779.9 91.35 778.9 93.45 778.9 96.15C778.9 98.85 779.95 101.05 782.05 102.75C784.15 104.45 787.2 105.3 791.2 105.3Z"
					fill="#A8FF78"
				/>
				<path
					d="M854.519 148.5V44.1H873.119V53.1H875.819C877.519 50.2 880.169 47.65 883.769 45.45C887.369 43.15 892.519 42 899.219 42C905.219 42 910.769 43.5 915.869 46.5C920.969 49.4 925.069 53.7 928.169 59.4C931.269 65.1 932.819 72 932.819 80.1V82.5C932.819 90.6 931.269 97.5 928.169 103.2C925.069 108.9 920.969 113.25 915.869 116.25C910.769 119.15 905.219 120.6 899.219 120.6C894.719 120.6 890.919 120.05 887.819 118.95C884.819 117.95 882.369 116.65 880.469 115.05C878.669 113.35 877.219 111.65 876.119 109.95H873.419V148.5H854.519ZM893.519 104.1C899.419 104.1 904.269 102.25 908.069 98.55C911.969 94.75 913.919 89.25 913.919 82.05V80.55C913.919 73.35 911.969 67.9 908.069 64.2C904.169 60.4 899.319 58.5 893.519 58.5C887.719 58.5 882.869 60.4 878.969 64.2C875.069 67.9 873.119 73.35 873.119 80.55V82.05C873.119 89.25 875.069 94.75 878.969 98.55C882.869 102.25 887.719 104.1 893.519 104.1Z"
					fill="#A8FF78"
				/>
				<path
					d="M953.173 148.5V44.1H971.773V53.1H974.473C976.173 50.2 978.823 47.65 982.423 45.45C986.023 43.15 991.173 42 997.873 42C1003.87 42 1009.42 43.5 1014.52 46.5C1019.62 49.4 1023.72 53.7 1026.82 59.4C1029.92 65.1 1031.47 72 1031.47 80.1V82.5C1031.47 90.6 1029.92 97.5 1026.82 103.2C1023.72 108.9 1019.62 113.25 1014.52 116.25C1009.42 119.15 1003.87 120.6 997.873 120.6C993.373 120.6 989.573 120.05 986.473 118.95C983.473 117.95 981.023 116.65 979.123 115.05C977.323 113.35 975.873 111.65 974.773 109.95H972.073V148.5H953.173ZM992.173 104.1C998.073 104.1 1002.92 102.25 1006.72 98.55C1010.62 94.75 1012.57 89.25 1012.57 82.05V80.55C1012.57 73.35 1010.62 67.9 1006.72 64.2C1002.82 60.4 997.973 58.5 992.173 58.5C986.373 58.5 981.523 60.4 977.623 64.2C973.723 67.9 971.773 73.35 971.773 80.55V82.05C971.773 89.25 973.723 94.75 977.623 98.55C981.523 102.25 986.373 104.1 992.173 104.1Z"
					fill="#A8FF78"
				/>
				<path
					d="M142.315 2.54452C140.855 -0.726526 136.686 -0.857368 134.94 2.2305C126.862 16.3352 112.4 25.1278 95.8793 25.1278H75.0329C47.4114 25.1278 25.0015 47.6326 25.0015 75.3711C25.0015 77.2029 25.2099 78.9561 25.3923 80.7356C42.0173 68.7766 66.0168 58.6233 100.049 58.6233C102.342 58.6233 104.218 60.5074 104.218 62.8103C104.218 65.1131 102.342 66.9972 100.049 66.9972C34.5387 66.9972 6.76083 107.323 0.611135 122.474C-1.10869 126.74 0.923832 131.607 5.17129 133.36C9.44481 135.14 14.2916 133.072 16.0635 128.833C16.4544 127.891 21.5097 116.298 34.7993 105.125C43.2421 116.612 59.2938 127.577 80.3748 125.326C121.286 122.343 150.08 85.4982 150.08 40.384C150.08 27.2475 147.266 13.6399 142.315 2.54452Z"
					fill="#A8FF78"
				/>
			</svg>
			<ProgressBar class="loading-bar" :progress="Math.min(loadingProgress, 100)" />
			<span v-if="message">{{ message }}</span>
		</div>
		<div class="gradient-bg" data-tauri-drag-region></div>
		<div class="cube-bg"></div>
		<div class="base-bg"></div>
	</div>
</template>

<script setup>
import { MaximizeIcon, MinimizeIcon, XIcon } from '@fernite/assets'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ref, watch } from 'vue'

import ProgressBar from '@/components/ui/ProgressBar.vue'
import { loading_listener } from '@/helpers/events.js'
import { getOS } from '@/helpers/utils.js'
import { useLoading } from '@/store/loading.js'

const doneLoading = ref(false)
const loadingProgress = ref(0)
const hidden = ref(false)
const message = ref()

const loading = useLoading()

watch(loading, (newValue) => {
	if (!newValue.barEnabled) {
		if (loading.loading) {
			loadingProgress.value = 0
			fakeLoadingIncrease()
		} else {
			loadingProgress.value = 100
			doneLoading.value = true

			setTimeout(() => {
				hidden.value = true
				loading.setEnabled(true)
			}, 50)
		}
	}
})

function fakeLoadingIncrease() {
	if (loadingProgress.value < 95) {
		setTimeout(() => {
			loadingProgress.value += 1
			fakeLoadingIncrease()
		}, 5)
	}
}

const os = ref('')
getOS().then((x) => (os.value = x))

loading_listener(async (e) => {
	if (e.event.type === 'directory_move') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Updating app directory...'
	} else if (e.event.type === 'checking_for_updates') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Checking for updates...'
	} else if (e.event.type === 'state_init') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.event.step || 'Initializing...'
	} else if (e.event.type === 'minecraft_launch') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.message || `Launching ${e.event.profile_name}...`
	} else if (e.event.type === 'minecraft_download') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.message || `Downloading Minecraft for ${e.event.profile_name}...`
	} else if (e.event.type === 'java_download') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = `Downloading Java ${e.event.version}...`
	} else if (e.event.type === 'legacy_data_migration') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Migrating data from previous version...'
	} else if (e.event.type === 'profile_update') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.message || `Updating ${e.event.profile_name}...`
	} else if (e.event.type === 'zip_extract') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.message || `Extracting ${e.event.profile_name}...`
	} else if (e.event.type === 'pack_download' || e.event.type === 'pack_file_download') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = e.message || `Downloading ${e.event.pack_name}...`
	} else if (e.event.type === 'config_change') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Changing app directory...'
	} else if (e.event.type === 'copy_profile') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = `Duplicating ${e.event.profile_name}...`
	} else if (e.event.type === 'launcher_update') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = `Updating to version ${e.event.version}...`
	}
})

const handleClose = async () => {
	await getCurrentWindow().close()
}
</script>

<style scoped lang="scss">
.splash-screen {
	transition: opacity 0.25s ease-in-out;
	opacity: 1;

	&.fade-out {
		opacity: 0;
	}
}

.app-buttons {
	position: absolute;
	right: 0;
	z-index: 9999;
	display: flex;
}

.app-logo-wrapper {
	position: absolute;
	height: 100vh;
	width: 100%;

	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;

	gap: 1rem;

	z-index: 9998;
}

.app-logo {
	height: auto;
	width: clamp(16rem, 40vw, 32rem);
}

.loading-bar {
	max-width: 20rem;
}

.gradient-bg {
	position: absolute;
	height: 100vh;
	width: 100vw;
	background:
		linear-gradient(180deg, rgba(66, 131, 92, 0.275) 0%, rgba(17, 35, 43, 0.5) 97.29%),
		linear-gradient(0deg, rgba(22, 24, 28, 0.64), rgba(22, 24, 28, 0.64));
	z-index: 9997;
}

.cube-bg {
	position: absolute;

	left: 50%;
	top: 50%;
	transform: translate(-50%, -50%);

	width: 180vw;
	height: 180vh;
	opacity: 0.8;
	background: #16181c url('@/assets/loading/cube.png') center no-repeat;
	background-size: contain;

	z-index: 9996;
}

.base-bg {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: var(--color-bg);
	z-index: 9995;
}
</style>
