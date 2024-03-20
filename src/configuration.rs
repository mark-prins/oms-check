use config::{Config, ConfigError, File, FileFormat, FileSourceFile};
// use settings::Settings;
use std::{
    env::{self, VarError},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    path::PathBuf,
};

use crate::{
    environment::{AppEnvironment, EnvironmentVariable},
    setttings::Settings,
};

const CONFIGURATION_DIRECTORY_PATH: &str = "configuration";
const CONFIGURATION_BASE_FILE_PATH: &str = "base.yaml";

pub enum SettingsError {
    Config(ConfigError),
    Environment(VarError),
    File(IoError),
}

/// Gets directory storing configuration files.
///
/// All configuration files should be stored in the same directory.
pub fn get_configuration_directory() -> Result<PathBuf, SettingsError> {
    let configuration_directory = env::current_dir()?.join(CONFIGURATION_DIRECTORY_PATH);
    Ok(configuration_directory)
}

/// Gets base configuration file.
///
/// The base configuration file stores configuration properties which are common between local and
/// production environments.
pub fn get_configuration_base_file() -> Result<File<FileSourceFile, FileFormat>, SettingsError> {
    let configuration_directory = get_configuration_directory()?;
    let base_file = File::from(get_configuration_base_file_path(configuration_directory));
    Ok(base_file)
}
/// Gets base configuration file path
///
/// The base configuration file stores configuration properties which are common between local and
/// production environments.
pub fn get_configuration_base_file_path(configuration_directory: PathBuf) -> PathBuf {
    configuration_directory.join(CONFIGURATION_BASE_FILE_PATH)
}

/// Gets application configuration file.
///
/// The application configuration file stores environment-specific configuration properties. Valid
/// environments are `local` and `production`.
pub fn get_configuration_app_file() -> Result<File<FileSourceFile, FileFormat>, SettingsError> {
    let configuration_directory = get_configuration_directory()?;
    let app_file = File::from(get_configuration_app_file_path(configuration_directory));
    Ok(app_file)
}

/// Gets application configuration filepath
///
/// The application configuration file stores environment-specific configuration properties. Valid
/// environments are `local` and `production`.
pub fn get_configuration_app_file_path(configuration_directory: PathBuf) -> PathBuf {
    configuration_directory
        .join(AppEnvironment::get())
        .with_extension("yaml")
}

/// Gets app configuration.
///
/// App configuration varies based on whether the app is being run in a `local` or `production`
/// environment. Configuration files should be stored in a unique configuration directory, and
/// should define setting values for `base`, `local` and `production` environments.
pub fn get_configuration() -> Result<Settings, SettingsError> {
    let configuration: Config = Config::builder()
        .add_source(get_configuration_base_file()?)
        .add_source(get_configuration_app_file()?)
        .build()?;
    let settings: Settings = configuration.try_deserialize()?;
    Ok(settings)
}

impl Debug for SettingsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SettingsError::Config(err) => write!(f, "{:?}", err),
            SettingsError::Environment(err) => write!(f, "{:?}", err),
            SettingsError::File(err) => write!(f, "{:?}", err),
        }
    }
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SettingsError::Config(err) => write!(f, "{}", err),
            SettingsError::Environment(err) => write!(f, "{}", err),
            SettingsError::File(err) => write!(f, "{}", err),
        }
    }
}

impl From<ConfigError> for SettingsError {
    fn from(err: ConfigError) -> SettingsError {
        SettingsError::Config(err)
    }
}

impl From<IoError> for SettingsError {
    fn from(err: IoError) -> SettingsError {
        SettingsError::File(err)
    }
}

impl From<VarError> for SettingsError {
    fn from(err: VarError) -> SettingsError {
        SettingsError::Environment(err)
    }
}
