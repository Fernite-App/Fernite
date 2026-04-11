//! Theseus settings file

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;

// Types
/// Global Theseus settings
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub max_concurrent_downloads: usize,
    pub max_concurrent_writes: usize,

    pub theme: Theme,
    pub accent_color: Option<String>,
    pub locale: String,
    pub default_page: DefaultPage,
    pub collapsed_navigation: bool,
    pub hide_nametag_skins_page: bool,
    pub advanced_rendering: bool,
    pub native_decorations: bool,
    pub toggle_sidebar: bool,

    pub discord_rpc: bool,

    pub onboarded: bool,

    pub extra_launch_args: Vec<String>,
    pub custom_env_vars: Vec<(String, String)>,
    pub memory: MemorySettings,
    pub force_fullscreen: bool,
    pub game_resolution: WindowSize,
    pub hide_on_process_start: bool,
    pub hooks: Hooks,

    pub custom_dir: Option<String>,
    pub prev_custom_dir: Option<String>,
    pub migrated: bool,

    pub developer_mode: bool,
    pub feature_flags: HashMap<FeatureFlag, bool>,

    pub skipped_update: Option<String>,
    pub pending_update_toast_for_version: Option<String>,
    pub auto_download_updates: Option<bool>,
    pub auto_update_mods: Option<bool>,

    pub version: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    PagePath,
    ProjectBackground,
    WorldsTab,
    WorldsInHome,
    ServersInApp,
    ServerProjectQa,
    I18nDebug,
}

impl Settings {
    const CURRENT_VERSION: usize = 2;

    pub async fn get(
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<Self> {
        let res = sqlx::query(
            "SELECT max_concurrent_writes, max_concurrent_downloads, theme, accent_color, locale, default_page, collapsed_navigation, hide_nametag_skins_page, advanced_rendering, native_decorations, discord_rpc, developer_mode, onboarded, json(extra_launch_args) extra_launch_args, json(custom_env_vars) custom_env_vars, mc_memory_max, mc_force_fullscreen, mc_game_resolution_x, mc_game_resolution_y, hide_on_process_start, hook_pre_launch, hook_wrapper, hook_post_exit, custom_dir, prev_custom_dir, migrated, json(feature_flags) feature_flags, toggle_sidebar, skipped_update, pending_update_toast_for_version, auto_download_updates, auto_update_mods, version FROM settings",
        )
            .fetch_one(exec)
            .await?;

        let max_concurrent_downloads: i32 =
            res.try_get("max_concurrent_downloads")?;
        let max_concurrent_writes: i32 =
            res.try_get("max_concurrent_writes")?;
        let theme: String = res.try_get("theme")?;
        let accent_color: Option<String> =
            res.try_get("accent_color").ok().flatten();
        let locale: String = res.try_get("locale")?;
        let default_page: String = res.try_get("default_page")?;
        let collapsed_navigation: i64 = res.try_get("collapsed_navigation")?;
        let hide_nametag_skins_page: i64 =
            res.try_get("hide_nametag_skins_page")?;
        let advanced_rendering: i64 = res.try_get("advanced_rendering")?;
        let native_decorations: i64 = res.try_get("native_decorations")?;
        let toggle_sidebar: i64 = res.try_get("toggle_sidebar")?;
        let discord_rpc: i64 = res.try_get("discord_rpc")?;
        let developer_mode: i64 = res.try_get("developer_mode")?;
        let onboarded: i64 = res.try_get("onboarded")?;
        let extra_launch_args: Option<String> =
            res.try_get("extra_launch_args").ok().flatten();
        let custom_env_vars: Option<String> =
            res.try_get("custom_env_vars").ok().flatten();
        let mc_memory_max: i64 = res.try_get("mc_memory_max")?;
        let mc_force_fullscreen: i64 = res.try_get("mc_force_fullscreen")?;
        let mc_game_resolution_x: i64 = res.try_get("mc_game_resolution_x")?;
        let mc_game_resolution_y: i64 = res.try_get("mc_game_resolution_y")?;
        let hide_on_process_start: i64 =
            res.try_get("hide_on_process_start")?;
        let hook_pre_launch: Option<String> =
            res.try_get("hook_pre_launch").ok().flatten();
        let hook_wrapper: Option<String> =
            res.try_get("hook_wrapper").ok().flatten();
        let hook_post_exit: Option<String> =
            res.try_get("hook_post_exit").ok().flatten();
        let custom_dir: Option<String> =
            res.try_get("custom_dir").ok().flatten();
        let prev_custom_dir: Option<String> =
            res.try_get("prev_custom_dir").ok().flatten();
        let migrated: i64 = res.try_get("migrated")?;
        let feature_flags: Option<String> =
            res.try_get("feature_flags").ok().flatten();
        let skipped_update: Option<String> =
            res.try_get("skipped_update").ok().flatten();
        let pending_update_toast_for_version: Option<String> = res
            .try_get("pending_update_toast_for_version")
            .ok()
            .flatten();
        let auto_download_updates: Option<i64> =
            res.try_get("auto_download_updates").ok().flatten();
        let auto_update_mods: Option<i64> =
            res.try_get("auto_update_mods").ok().flatten();
        let version: i64 = res.try_get("version")?;

        Ok(Self {
            max_concurrent_downloads: max_concurrent_downloads as usize,
            max_concurrent_writes: max_concurrent_writes as usize,
            theme: Theme::from_string(&theme),
            accent_color,
            locale,
            default_page: DefaultPage::from_string(&default_page),
            collapsed_navigation: collapsed_navigation == 1,
            hide_nametag_skins_page: hide_nametag_skins_page == 1,
            advanced_rendering: advanced_rendering == 1,
            native_decorations: native_decorations == 1,
            toggle_sidebar: toggle_sidebar == 1,
            discord_rpc: discord_rpc == 1,
            developer_mode: developer_mode == 1,
            onboarded: onboarded == 1,
            extra_launch_args: extra_launch_args
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            custom_env_vars: custom_env_vars
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            memory: MemorySettings {
                maximum: mc_memory_max as u32,
            },
            force_fullscreen: mc_force_fullscreen == 1,
            game_resolution: WindowSize(
                mc_game_resolution_x as u16,
                mc_game_resolution_y as u16,
            ),
            hide_on_process_start: hide_on_process_start == 1,
            hooks: Hooks {
                pre_launch: hook_pre_launch,
                wrapper: hook_wrapper,
                post_exit: hook_post_exit,
            },
            custom_dir,
            prev_custom_dir,
            migrated: migrated == 1,
            feature_flags: feature_flags
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            skipped_update,
            pending_update_toast_for_version,
            auto_download_updates: auto_download_updates.map(|x| x == 1),
            auto_update_mods: auto_update_mods.map(|x| x == 1),
            version: version as usize,
        })
    }

    pub async fn update(
        &self,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<()> {
        let max_concurrent_writes = self.max_concurrent_writes as i32;
        let max_concurrent_downloads = self.max_concurrent_downloads as i32;
        let theme = self.theme.as_str();
        let default_page = self.default_page.as_str();
        let extra_launch_args = serde_json::to_string(&self.extra_launch_args)?;
        let custom_env_vars = serde_json::to_string(&self.custom_env_vars)?;
        let feature_flags = serde_json::to_string(&self.feature_flags)?;
        let version = self.version as i64;

        sqlx::query(
            "UPDATE settings SET max_concurrent_writes = $1, max_concurrent_downloads = $2, theme = $3, accent_color = $4, locale = $5, default_page = $6, collapsed_navigation = $7, advanced_rendering = $8, native_decorations = $9, discord_rpc = $10, developer_mode = $11, onboarded = $12, extra_launch_args = jsonb($13), custom_env_vars = jsonb($14), mc_memory_max = $15, mc_force_fullscreen = $16, mc_game_resolution_x = $17, mc_game_resolution_y = $18, hide_on_process_start = $19, hook_pre_launch = $20, hook_wrapper = $21, hook_post_exit = $22, custom_dir = $23, prev_custom_dir = $24, migrated = $25, toggle_sidebar = $26, feature_flags = jsonb($27), hide_nametag_skins_page = $28, skipped_update = $29, pending_update_toast_for_version = $30, auto_download_updates = $31, auto_update_mods = $32, version = $33",
        )
        .bind(max_concurrent_writes)
        .bind(max_concurrent_downloads)
        .bind(theme)
        .bind(&self.accent_color)
        .bind(&self.locale)
        .bind(default_page)
        .bind(self.collapsed_navigation)
        .bind(self.advanced_rendering)
        .bind(self.native_decorations)
        .bind(self.discord_rpc)
        .bind(self.developer_mode)
        .bind(self.onboarded)
        .bind(extra_launch_args)
        .bind(custom_env_vars)
        .bind(self.memory.maximum as i64)
        .bind(self.force_fullscreen)
        .bind(self.game_resolution.0 as i64)
        .bind(self.game_resolution.1 as i64)
        .bind(self.hide_on_process_start)
        .bind(&self.hooks.pre_launch)
        .bind(&self.hooks.wrapper)
        .bind(&self.hooks.post_exit)
        .bind(&self.custom_dir)
        .bind(&self.prev_custom_dir)
        .bind(self.migrated)
        .bind(self.toggle_sidebar)
        .bind(feature_flags)
        .bind(self.hide_nametag_skins_page)
        .bind(&self.skipped_update)
        .bind(&self.pending_update_toast_for_version)
        .bind(self.auto_download_updates)
        .bind(self.auto_update_mods)
        .bind(version)
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn migrate(exec: &Pool<Sqlite>) -> crate::Result<()> {
        let mut settings = Self::get(exec).await?;

        if settings.version < Settings::CURRENT_VERSION {
            tracing::info!(
                "Migrating settings version {} to {:?}",
                settings.version,
                Settings::CURRENT_VERSION
            );
        }
        while settings.version < Settings::CURRENT_VERSION {
            if let Err(err) = settings.perform_migration() {
                tracing::error!(
                    "Failed to migrate settings from version {}: {}",
                    settings.version,
                    err
                );
                return Err(err);
            }
        }

        settings.update(exec).await?;

        Ok(())
    }

    pub fn perform_migration(&mut self) -> crate::Result<()> {
        match self.version {
            1 => {
                let quoter = shlex::Quoter::new().allow_nul(true);

                // Previously split by spaces
                if let Some(pre_launch) = self.hooks.pre_launch.as_ref() {
                    self.hooks.pre_launch =
                        Some(quoter.join(pre_launch.split(' ')).unwrap())
                }

                // Previously treated as complete path to command
                if let Some(wrapper) = self.hooks.wrapper.as_ref() {
                    self.hooks.wrapper =
                        Some(quoter.quote(wrapper).unwrap().to_string())
                }

                // Previously split by spaces
                if let Some(post_exit) = self.hooks.post_exit.as_ref() {
                    self.hooks.post_exit =
                        Some(quoter.join(post_exit.split(' ')).unwrap())
                }

                self.version = 2;
            }
            version => {
                return Err(crate::ErrorKind::OtherError(format!(
                    "Invalid settings version: {version}"
                ))
                .into());
            }
        }

        Ok(())
    }
}

/// Theseus theme
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Dark,
    Light,
    Oled,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Oled => "oled",
            Theme::System => "system",
        }
    }

    pub fn from_string(string: &str) -> Theme {
        match string {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "oled" => Theme::Oled,
            "system" => Theme::System,
            _ => Theme::Dark,
        }
    }
}

/// Minecraft memory settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MemorySettings {
    pub maximum: u32,
}

/// Game window size
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WindowSize(pub u16, pub u16);

/// Game initialization hooks
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_with::serde_as]
pub struct Hooks {
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub pre_launch: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub wrapper: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub post_exit: Option<String>,
}

/// Opening window to start with
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DefaultPage {
    Home,
    Library,
}

impl DefaultPage {
    pub fn as_str(&self) -> &'static str {
        match self {
            DefaultPage::Home => "home",
            DefaultPage::Library => "library",
        }
    }

    pub fn from_string(string: &str) -> Self {
        match string {
            "home" => Self::Home,
            "library" => Self::Library,
            _ => Self::Home,
        }
    }
}
