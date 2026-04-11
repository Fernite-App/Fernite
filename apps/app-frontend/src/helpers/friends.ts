import type { User } from '@fernite/utils'
import { invoke } from '@tauri-apps/api/core'
import type { Dayjs } from 'dayjs'
import dayjs from 'dayjs'

import { get_user_many } from '@/helpers/cache'
import type { ModrinthCredentials } from '@/helpers/mr_auth'

export type UserStatus = {
	user_id: string
	profile_name: string | null
	last_update: string
}

export type UserFriend = {
	id: string
	friend_id: string
	accepted: boolean
	created: string
}

export async function friends(): Promise<UserFriend[]> {
	return await invoke('plugin:friends|friends')
}

export async function friend_statuses(): Promise<UserStatus[]> {
	return await invoke('plugin:friends|friend_statuses')
}

export async function add_friend(userId: string): Promise<void> {
	return await invoke('plugin:friends|add_friend', { userId })
}

export async function remove_friend(userId: string): Promise<void> {
	return await invoke('plugin:friends|remove_friend', { userId })
}

export type FriendWithUserData = {
	id: string
	friend_id: string | null
	status: string | null
	last_updated: Dayjs | null
	created: Dayjs
	username: string
	accepted: boolean
	online: boolean
	avatar: string
}
export async function transformFriends(
	friends: UserFriend[],
	credentials: ModrinthCredentials | null,
): Promise<FriendWithUserData[]> {
	if (friends.length === 0) {
		return []
	}

	const friendStatuses = await friend_statuses()

	// When credentials are available, resolve Modrinth usernames/avatars.
	// When offline (no credentials), we still show friends by their user_id.
	const userMap: Map<string, User> = new Map()
	if (credentials) {
		const userIds = friends.map((x) => (x.id === credentials.user_id ? x.friend_id : x.id))
		const users = await get_user_many(userIds)
		for (const u of users as User[]) {
			userMap.set(u.id, u)
		}
	}

	return friends.map((friend) => {
		// Determine which side is "the other person"
		const otherId = credentials
			? friend.id === credentials.user_id
				? friend.friend_id
				: friend.id
			: friend.friend_id ?? friend.id

		const user = userMap.get(otherId)
		const status = friendStatuses.find(
			(x) => x.user_id === friend.id || x.user_id === friend.friend_id,
		)
		return {
			id: friend.id,
			friend_id: friend.friend_id,
			status: status?.profile_name ?? null,
			last_updated: status && status.last_update ? dayjs(status.last_update) : null,
			created: dayjs(friend.created),
			avatar: user?.avatar_url ?? '',
			username: user?.username ?? otherId,
			online: !!status,
			accepted: friend.accepted,
		}
	})
}
