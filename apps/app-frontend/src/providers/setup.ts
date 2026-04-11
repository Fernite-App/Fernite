import type { AbstractWebNotificationManager } from '@fernite/ui'

import { setupCreationModal } from './setup/creation-modal'
import { setupFilePickerProvider } from './setup/file-picker'
import { setupInstanceImportProvider } from './setup/instance-import'

export { setupTagsProvider } from './setup/tags'

export function setupProviders(notificationManager: AbstractWebNotificationManager) {
	// NOTE: setupTagsProvider is intentionally NOT called here. It must be called
	// after initialize_state() resolves, because get_loader_tags / get_game_version_tags
	// require the backend state to be initialized first.
	setupFilePickerProvider()
	setupInstanceImportProvider(notificationManager)

	return {
		...setupCreationModal(notificationManager),
	}
}
