import type { AbstractWebNotificationManager } from '@fernite/ui'
import { provideTags } from '@fernite/ui'
import { ref } from 'vue'

import { get_game_versions, get_loaders } from '@/helpers/tags'

export function setupTagsProvider(notificationManager: AbstractWebNotificationManager) {
	const { handleError } = notificationManager

	const gameVersions = ref([])
	const loaders = ref([])
	get_game_versions()
		.then((v) => {
			gameVersions.value = v
		})
		.catch(handleError)
	get_loaders()
		.then((v) => {
			loaders.value = v
		})
		.catch(handleError)

	provideTags({ gameVersions, loaders })
}
