use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const KARMA_MAX: f32 = 99.9;
const INTERACTIONS_PER_PERIOD: u8 = 5;
const PERIODS_PER_DAY: u8 = 3;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerProgress>()
            .init_resource::<ApplicationSettings>()
            .init_resource::<PersistenceStatus>()
            .add_systems(Startup, load_persisted_data)
            .add_systems(Update, (save_progress, save_settings));
    }
}

#[derive(Resource, Default)]
struct PersistenceStatus {
    progress_load_failed: bool,
    settings_load_failed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    InsufficientFunds {
        balance: u64,
        required: u64,
    },
    BalanceOverflow,
    InvalidKarma,
    InvalidCard(String),
    CardNotOwned(String),
    DuplicateCard(String),
    InvalidCollectionSize,
    CollectionThresholdNotMet {
        completion_percent: f32,
        required: f32,
    },
    PrestigeLocked,
    InsufficientPrestige {
        balance: u64,
        required: u64,
    },
    InvalidClock,
    InvalidSettings,
    InvalidProgress(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientFunds { balance, required } => {
                write!(
                    f,
                    "insufficient funds: balance {balance}, required {required}"
                )
            }
            Self::BalanceOverflow => write!(f, "currency balance overflow"),
            Self::InvalidKarma => write!(f, "karma must be a finite value from 0 to 100"),
            Self::InvalidCard(id) => write!(f, "invalid card data for '{id}'"),
            Self::CardNotOwned(id) => write!(f, "card '{id}' is not in the collection"),
            Self::DuplicateCard(id) => write!(f, "duplicate card instance '{id}'"),
            Self::InvalidCollectionSize => write!(f, "collection size must be greater than zero"),
            Self::CollectionThresholdNotMet {
                completion_percent,
                required,
            } => write!(
                f,
                "collection is {completion_percent:.1}% complete; {required:.1}% is required"
            ),
            Self::PrestigeLocked => write!(f, "prestige has not been unlocked"),
            Self::InsufficientPrestige { balance, required } => {
                write!(
                    f,
                    "insufficient prestige: balance {balance}, required {required}"
                )
            }
            Self::InvalidClock => write!(f, "invalid in-game clock value"),
            Self::InvalidSettings => write!(f, "application settings are outside valid ranges"),
            Self::InvalidProgress(message) => write!(f, "invalid player progress: {message}"),
        }
    }
}

impl Error for GameError {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(u64);

impl Money {
    pub fn new(balance: u64) -> Self {
        Self(balance)
    }

    pub fn balance(self) -> u64 {
        self.0
    }

    pub fn deposit(&mut self, amount: u64) -> Result<(), GameError> {
        self.0 = self
            .0
            .checked_add(amount)
            .ok_or(GameError::BalanceOverflow)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), GameError> {
        if amount > self.0 {
            return Err(GameError::InsufficientFunds {
                balance: self.0,
                required: amount,
            });
        }
        self.0 -= amount;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Karma(f32);

impl Default for Karma {
    fn default() -> Self {
        Self(50.0)
    }
}

impl Karma {
    pub fn new(value: f32) -> Result<Self, GameError> {
        if !value.is_finite() || !(0.0..=100.0).contains(&value) {
            return Err(GameError::InvalidKarma);
        }
        Ok(Self(value.min(KARMA_MAX)))
    }

    pub fn percent(self) -> f32 {
        self.0
    }

    pub fn gain(&mut self, percentage_points: f32) -> Result<(), GameError> {
        if !percentage_points.is_finite() || percentage_points < 0.0 {
            return Err(GameError::InvalidKarma);
        }
        self.0 = (self.0 + percentage_points).min(KARMA_MAX);
        Ok(())
    }

    pub fn lose(&mut self, percentage_points: f32) -> Result<(), GameError> {
        if !percentage_points.is_finite() || percentage_points < 0.0 {
            return Err(GameError::InvalidKarma);
        }
        self.0 = (self.0 - percentage_points).max(0.0);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayPeriod {
    Morning,
    Afternoon,
    Evening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameClock {
    day: u64,
    interactions_today: u8,
}

impl Default for GameClock {
    fn default() -> Self {
        Self {
            day: 1,
            interactions_today: 0,
        }
    }
}

impl GameClock {
    pub fn day(self) -> u64 {
        self.day
    }

    pub fn interactions_today(self) -> u8 {
        self.interactions_today
    }

    pub fn period(self) -> DayPeriod {
        match self.interactions_today / INTERACTIONS_PER_PERIOD {
            0 => DayPeriod::Morning,
            1 => DayPeriod::Afternoon,
            _ => DayPeriod::Evening,
        }
    }

    /// Advances one completed interaction and returns true when a new day starts.
    pub fn advance(&mut self) -> Result<bool, GameError> {
        if self.day == 0 || self.interactions_today >= INTERACTIONS_PER_PERIOD * PERIODS_PER_DAY {
            return Err(GameError::InvalidClock);
        }
        if self.interactions_today + 1 == INTERACTIONS_PER_PERIOD * PERIODS_PER_DAY {
            self.day = self.day.checked_add(1).ok_or(GameError::InvalidClock)?;
            self.interactions_today = 0;
            return Ok(true);
        }
        self.interactions_today += 1;
        Ok(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInstance {
    pub instance_id: String,
    pub card_id: String,
    pub rarity: CardRarity,
    pub psa_grade: Option<u8>,
}

impl CardInstance {
    pub fn validate(&self) -> Result<(), GameError> {
        if self.instance_id.trim().is_empty()
            || self.card_id.trim().is_empty()
            || self
                .psa_grade
                .is_some_and(|grade| !(1..=10).contains(&grade))
        {
            return Err(GameError::InvalidCard(self.instance_id.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Collection {
    cards: Vec<CardInstance>,
}

impl Collection {
    pub fn cards(&self) -> &[CardInstance] {
        &self.cards
    }

    pub fn add(&mut self, card: CardInstance) -> Result<(), GameError> {
        card.validate()?;
        if self
            .cards
            .iter()
            .any(|owned| owned.instance_id == card.instance_id)
        {
            return Err(GameError::DuplicateCard(card.instance_id));
        }
        self.cards.push(card);
        Ok(())
    }

    pub fn remove(&mut self, instance_id: &str) -> Result<CardInstance, GameError> {
        let Some(index) = self
            .cards
            .iter()
            .position(|card| card.instance_id == instance_id)
        else {
            return Err(GameError::CardNotOwned(instance_id.to_owned()));
        };
        Ok(self.cards.remove(index))
    }

    pub fn completion_percent(&self, total_cards: usize) -> Result<f32, GameError> {
        if total_cards == 0 {
            return Err(GameError::InvalidCollectionSize);
        }
        let unique_owned = self
            .cards
            .iter()
            .map(|card| &card.card_id)
            .collect::<std::collections::HashSet<_>>()
            .len();
        Ok((unique_owned as f32 / total_cards as f32 * 100.0).min(100.0))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Deck {
    card_ids: Vec<String>,
}

impl Deck {
    pub fn card_ids(&self) -> &[String] {
        &self.card_ids
    }

    pub fn add(&mut self, collection: &Collection, instance_id: &str) -> Result<(), GameError> {
        if !collection
            .cards
            .iter()
            .any(|card| card.instance_id == instance_id)
        {
            return Err(GameError::CardNotOwned(instance_id.to_owned()));
        }
        if self.card_ids.iter().any(|id| id == instance_id) {
            return Err(GameError::DuplicateCard(instance_id.to_owned()));
        }
        self.card_ids.push(instance_id.to_owned());
        Ok(())
    }

    pub fn remove(&mut self, instance_id: &str) -> Result<(), GameError> {
        let Some(index) = self.card_ids.iter().position(|id| id == instance_id) else {
            return Err(GameError::CardNotOwned(instance_id.to_owned()));
        };
        self.card_ids.remove(index);
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Prestige {
    unlocked: bool,
    points: u64,
}

impl Prestige {
    pub fn unlocked(&self) -> bool {
        self.unlocked
    }

    pub fn points(&self) -> u64 {
        self.points
    }

    pub fn spend(&mut self, amount: u64) -> Result<(), GameError> {
        if !self.unlocked {
            return Err(GameError::PrestigeLocked);
        }
        if amount > self.points {
            return Err(GameError::InsufficientPrestige {
                balance: self.points,
                required: amount,
            });
        }
        self.points -= amount;
        Ok(())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerProgress {
    money: Money,
    karma: Karma,
    prestige: Prestige,
    collection: Collection,
    deck: Deck,
    clock: GameClock,
    difficulty: u8,
}

impl Default for PlayerProgress {
    fn default() -> Self {
        Self {
            money: Money::default(),
            karma: Karma::default(),
            prestige: Prestige::default(),
            collection: Collection::default(),
            deck: Deck::default(),
            clock: GameClock::default(),
            difficulty: 1,
        }
    }
}

impl PlayerProgress {
    pub fn money(&self) -> Money {
        self.money
    }

    pub fn karma(&self) -> Karma {
        self.karma
    }

    pub fn prestige(&self) -> &Prestige {
        &self.prestige
    }

    pub fn collection(&self) -> &Collection {
        &self.collection
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn clock(&self) -> GameClock {
        self.clock
    }

    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn purchase(&mut self, price: u64) -> Result<(), GameError> {
        let mut next_clock = self.clock;
        next_clock.advance()?;
        self.money.withdraw(price)?;
        self.clock = next_clock;
        Ok(())
    }

    pub fn earn_money(&mut self, amount: u64) -> Result<(), GameError> {
        let mut next_clock = self.clock;
        next_clock.advance()?;
        let mut next_money = self.money;
        next_money.deposit(amount)?;
        self.money = next_money;
        self.clock = next_clock;
        Ok(())
    }

    pub fn complete_battle(&mut self, won: bool, reward: u64) -> Result<(), GameError> {
        let mut next_clock = self.clock;
        next_clock.advance()?;
        let mut next_money = self.money;
        let mut next_karma = self.karma;
        if won {
            next_money.deposit(reward)?;
            next_karma.lose(1.0)?;
        } else {
            next_karma.gain(1.0)?;
        }
        self.money = next_money;
        self.karma = next_karma;
        self.clock = next_clock;
        Ok(())
    }

    pub fn donate(&mut self, amount: u64) -> Result<(), GameError> {
        let mut next_clock = self.clock;
        next_clock.advance()?;
        let mut next_money = self.money;
        next_money.withdraw(amount)?;
        let mut next_karma = self.karma;
        next_karma.gain(2.0)?;
        self.money = next_money;
        self.karma = next_karma;
        self.clock = next_clock;
        Ok(())
    }

    pub fn accept_unfair_trade(&mut self) -> Result<(), GameError> {
        let mut next_clock = self.clock;
        next_clock.advance()?;
        let mut next_karma = self.karma;
        next_karma.gain(2.0)?;
        self.karma = next_karma;
        self.clock = next_clock;
        Ok(())
    }

    pub fn add_card(&mut self, card: CardInstance) -> Result<(), GameError> {
        self.collection.add(card)
    }

    pub fn remove_card(&mut self, instance_id: &str) -> Result<CardInstance, GameError> {
        let removed = self.collection.remove(instance_id)?;
        self.deck.card_ids.retain(|id| id != instance_id);
        Ok(removed)
    }

    pub fn add_card_to_deck(&mut self, instance_id: &str) -> Result<(), GameError> {
        self.deck.add(&self.collection, instance_id)
    }

    pub fn remove_card_from_deck(&mut self, instance_id: &str) -> Result<(), GameError> {
        self.deck.remove(instance_id)
    }

    pub fn spend_prestige(&mut self, amount: u64) -> Result<(), GameError> {
        self.prestige.spend(amount)
    }

    pub fn set_difficulty(&mut self, difficulty: u8) {
        self.difficulty = difficulty;
    }

    pub fn exchange_collection(
        &mut self,
        total_cards: usize,
        required_completion_percent: f32,
        prestige_reward: u64,
    ) -> Result<(), GameError> {
        if !required_completion_percent.is_finite()
            || !(0.0..=100.0).contains(&required_completion_percent)
        {
            return Err(GameError::InvalidCollectionSize);
        }
        let completion_percent = self.collection.completion_percent(total_cards)?;
        if completion_percent < required_completion_percent {
            return Err(GameError::CollectionThresholdNotMet {
                completion_percent,
                required: required_completion_percent,
            });
        }
        let new_points = self
            .prestige
            .points
            .checked_add(prestige_reward)
            .ok_or(GameError::BalanceOverflow)?;
        self.prestige.points = new_points;
        self.prestige.unlocked = true;
        self.collection.cards.clear();
        self.deck.card_ids.clear();
        Ok(())
    }

    fn validate(&self) -> Result<(), GameError> {
        if !self.karma.0.is_finite() || !(0.0..=KARMA_MAX).contains(&self.karma.0) {
            return Err(GameError::InvalidProgress(
                "karma is outside its valid range".into(),
            ));
        }
        if self.clock.day == 0
            || self.clock.interactions_today >= INTERACTIONS_PER_PERIOD * PERIODS_PER_DAY
        {
            return Err(GameError::InvalidClock);
        }
        let mut instance_ids = std::collections::HashSet::new();
        for card in &self.collection.cards {
            card.validate()?;
            if !instance_ids.insert(card.instance_id.as_str()) {
                return Err(GameError::DuplicateCard(card.instance_id.clone()));
            }
        }
        let mut deck_ids = std::collections::HashSet::new();
        for card_id in &self.deck.card_ids {
            if !instance_ids.contains(card_id.as_str()) {
                return Err(GameError::CardNotOwned(card_id.clone()));
            }
            if !deck_ids.insert(card_id.as_str()) {
                return Err(GameError::DuplicateCard(card_id.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub effects_volume: f32,
    pub left_handed_controls: bool,
    pub swipe_sensitivity: f32,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.8,
            effects_volume: 1.0,
            left_handed_controls: false,
            swipe_sensitivity: 1.0,
        }
    }
}

impl ApplicationSettings {
    fn validate(self) -> Result<(), GameError> {
        let volumes = [self.master_volume, self.music_volume, self.effects_volume];
        if volumes
            .iter()
            .any(|volume| !volume.is_finite() || !(0.0..=1.0).contains(volume))
            || !self.swipe_sensitivity.is_finite()
            || !(0.1..=4.0).contains(&self.swipe_sensitivity)
        {
            return Err(GameError::InvalidSettings);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionKind {
    Battle,
    Trade,
    Donation,
    MoneyGift,
    CardGift,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncounterDefinition {
    pub id: String,
    pub interaction: InteractionKind,
    pub available_periods: Vec<DayPeriod>,
    pub min_karma: Option<f32>,
    pub max_karma: Option<f32>,
    pub min_money: Option<u64>,
    pub min_prestige: Option<u64>,
    pub min_difficulty: Option<u8>,
    pub max_difficulty: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct OpponentProvider;

impl OpponentProvider {
    pub fn select<'a>(
        definitions: &'a [EncounterDefinition],
        player: &PlayerProgress,
    ) -> Option<&'a EncounterDefinition> {
        let karma = player.karma.percent();
        let period = player.clock.period();
        definitions
            .iter()
            .filter(|definition| {
                definition.available_periods.contains(&period)
                    && definition.min_karma.is_none_or(|minimum| karma >= minimum)
                    && definition.max_karma.is_none_or(|maximum| karma <= maximum)
                    && definition
                        .min_money
                        .is_none_or(|minimum| player.money.balance() >= minimum)
                    && definition
                        .min_prestige
                        .is_none_or(|minimum| player.prestige.points >= minimum)
                    && definition
                        .min_difficulty
                        .is_none_or(|minimum| player.difficulty >= minimum)
                    && definition
                        .max_difficulty
                        .is_none_or(|maximum| player.difficulty <= maximum)
            })
            .max_by_key(|definition| Self::relevance(definition.interaction, player))
    }

    fn relevance(interaction: InteractionKind, player: &PlayerProgress) -> u8 {
        let low_karma = player.karma.percent() < 30.0;
        let low_funds = player.money.balance() < 10;
        match (interaction, low_karma, low_funds) {
            (InteractionKind::Battle, _, true) => 3,
            (
                InteractionKind::Trade | InteractionKind::Donation | InteractionKind::MoneyGift,
                true,
                _,
            ) => 3,
            (InteractionKind::Battle, _, false) => 1,
            (
                InteractionKind::Trade | InteractionKind::Donation | InteractionKind::MoneyGift,
                false,
                _,
            ) => 1,
            (InteractionKind::CardGift, _, _) => 0,
        }
    }
}

#[derive(Debug)]
enum PersistenceError {
    DirectoryUnavailable,
    Io(io::Error),
    Json(serde_json::Error),
    InvalidData(GameError),
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectoryUnavailable => write!(f, "no application data directory is available"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Json(error) => write!(f, "invalid JSON: {error}"),
            Self::InvalidData(error) => write!(f, "{error}"),
        }
    }
}

impl From<io::Error> for PersistenceError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

fn data_directory() -> Result<PathBuf, PersistenceError> {
    #[cfg(target_os = "android")]
    {
        return bevy::android::ANDROID_APP
            .get()
            .and_then(|app| app.internal_data_path())
            .map(|path| path.to_path_buf())
            .ok_or(PersistenceError::DirectoryUnavailable);
    }
    #[cfg(not(target_os = "android"))]
    {
        directories::ProjectDirs::from("com", "Lotus", "Lotus-CCG")
            .map(|directories| directories.data_local_dir().to_path_buf())
            .ok_or(PersistenceError::DirectoryUnavailable)
    }
}

fn load_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Option<T>, PersistenceError> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<(), PersistenceError> {
    let bytes = serde_json::to_vec_pretty(value)?;
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, bytes)?;
    fs::rename(temporary_path, path)?;
    Ok(())
}

fn load_persisted_data(
    mut progress: ResMut<PlayerProgress>,
    mut settings: ResMut<ApplicationSettings>,
    mut status: ResMut<PersistenceStatus>,
) {
    let directory = match data_directory() {
        Ok(directory) => directory,
        Err(error) => {
            error!("Could not locate application data directory: {error}");
            status.progress_load_failed = true;
            status.settings_load_failed = true;
            return;
        }
    };
    if let Err(error) = fs::create_dir_all(&directory) {
        error!("Could not create application data directory: {error}");
        status.progress_load_failed = true;
        status.settings_load_failed = true;
        return;
    }
    match load_json::<PlayerProgress>(&directory.join("progress.json")) {
        Ok(Some(saved)) => match saved.validate() {
            Ok(()) => *progress = saved,
            Err(error) => {
                status.progress_load_failed = true;
                error!(
                    "Could not load player progress: {}",
                    PersistenceError::InvalidData(error)
                );
            }
        },
        Ok(None) => {}
        Err(error) => {
            status.progress_load_failed = true;
            error!("Could not load player progress: {error}");
        }
    }
    match load_json::<ApplicationSettings>(&directory.join("settings.json")) {
        Ok(Some(saved)) => match saved.validate() {
            Ok(()) => *settings = saved,
            Err(error) => {
                status.settings_load_failed = true;
                error!(
                    "Could not load application settings: {}",
                    PersistenceError::InvalidData(error)
                );
            }
        },
        Ok(None) => {}
        Err(error) => {
            status.settings_load_failed = true;
            error!("Could not load application settings: {error}");
        }
    }
}

fn save_progress(
    progress: Res<PlayerProgress>,
    status: Res<PersistenceStatus>,
    mut checked_initial_state: Local<bool>,
) {
    if !*checked_initial_state {
        *checked_initial_state = true;
        if status.progress_load_failed {
            return;
        }
    }
    if !progress.is_changed() {
        return;
    }
    if let Err(validation_error) = progress.validate() {
        error!("Could not save invalid player progress: {validation_error}");
        return;
    }
    if let Err(error) = persist("progress.json", &*progress) {
        error!("Could not save player progress: {error}");
    }
}

fn save_settings(
    settings: Res<ApplicationSettings>,
    status: Res<PersistenceStatus>,
    mut checked_initial_state: Local<bool>,
) {
    if !*checked_initial_state {
        *checked_initial_state = true;
        if status.settings_load_failed {
            return;
        }
    }
    if !settings.is_changed() {
        return;
    }
    if let Err(validation_error) = settings.validate() {
        error!("Could not save invalid application settings: {validation_error}");
        return;
    }
    if let Err(error) = persist("settings.json", &*settings) {
        error!("Could not save application settings: {error}");
    }
}

fn persist<T: Serialize>(filename: &str, value: &T) -> Result<(), PersistenceError> {
    let directory = data_directory()?;
    fs::create_dir_all(&directory)?;
    save_json(&directory.join(filename), value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(id: &str) -> CardInstance {
        CardInstance {
            instance_id: id.into(),
            card_id: id.into(),
            rarity: CardRarity::Rare,
            psa_grade: Some(10),
        }
    }

    #[test]
    fn purchases_validate_before_changing_money() {
        let mut progress = PlayerProgress::default();
        progress.money = Money::new(3);

        assert_eq!(
            progress.purchase(5),
            Err(GameError::InsufficientFunds {
                balance: 3,
                required: 5
            })
        );
        assert_eq!(progress.money().balance(), 3);
    }

    #[test]
    fn karma_gains_never_reach_one_hundred_percent() {
        let mut karma = Karma::new(99.0).unwrap();
        karma.gain(10.0).unwrap();
        assert_eq!(karma.percent(), 99.9);
        karma.gain(10.0).unwrap();
        assert_eq!(karma.percent(), 99.9);
    }

    #[test]
    fn clock_advances_periods_and_rolls_over_after_fifteen_interactions() {
        let mut clock = GameClock::default();
        for interaction in 0..15 {
            assert_eq!(
                clock.period(),
                match interaction / 5 {
                    0 => DayPeriod::Morning,
                    1 => DayPeriod::Afternoon,
                    _ => DayPeriod::Evening,
                }
            );
            assert_eq!(clock.advance().unwrap(), interaction == 14);
        }
        assert_eq!(clock.day(), 2);
        assert_eq!(clock.interactions_today(), 0);
    }

    #[test]
    fn losing_battle_gains_karma_without_changing_money() {
        let mut progress = PlayerProgress::default();
        progress.money = Money::new(7);
        let karma_before = progress.karma().percent();

        progress.complete_battle(false, 50).unwrap();

        assert_eq!(progress.money().balance(), 7);
        assert_eq!(progress.karma().percent(), karma_before + 1.0);
        assert_eq!(progress.clock().interactions_today(), 1);
    }

    #[test]
    fn decks_only_accept_owned_cards_and_exchange_clears_the_collection() {
        let mut progress = PlayerProgress::default();
        progress.add_card(card("owned")).unwrap();
        progress.add_card_to_deck("owned").unwrap();

        assert_eq!(
            progress.add_card_to_deck("missing"),
            Err(GameError::CardNotOwned("missing".into()))
        );
        assert!(progress.exchange_collection(2, 100.0, 3).is_err());
        assert_eq!(progress.collection.cards().len(), 1);
        progress.exchange_collection(1, 100.0, 3).unwrap();
        assert!(progress.collection.cards().is_empty());
        assert!(progress.deck.card_ids().is_empty());
        assert!(progress.prestige().unlocked());
        assert_eq!(progress.prestige().points(), 3);
    }

    #[test]
    fn duplicate_copies_do_not_inflate_collection_completion() {
        let mut collection = Collection::default();
        collection
            .add(CardInstance {
                instance_id: "copy-1".into(),
                card_id: "catalog-card".into(),
                rarity: CardRarity::Common,
                psa_grade: None,
            })
            .unwrap();
        collection
            .add(CardInstance {
                instance_id: "copy-2".into(),
                card_id: "catalog-card".into(),
                rarity: CardRarity::Common,
                psa_grade: None,
            })
            .unwrap();

        assert_eq!(collection.completion_percent(2).unwrap(), 50.0);
    }

    #[test]
    fn failed_donation_changes_neither_money_karma_nor_time() {
        let mut progress = PlayerProgress::default();
        progress.money = Money::new(1);
        let before = progress.clone();

        assert!(progress.donate(2).is_err());
        assert_eq!(progress, before);
    }

    #[test]
    fn encounter_selection_respects_period_and_player_conditions() {
        let evening = EncounterDefinition {
            id: "evening-only".into(),
            interaction: InteractionKind::Battle,
            available_periods: vec![DayPeriod::Evening],
            min_karma: None,
            max_karma: None,
            min_money: None,
            min_prestige: None,
            min_difficulty: None,
            max_difficulty: None,
        };
        let mut progress = PlayerProgress::default();
        assert!(OpponentProvider::select(&[evening.clone()], &progress).is_none());
        for _ in 0..10 {
            progress.clock.advance().unwrap();
        }
        assert_eq!(
            OpponentProvider::select(&[evening.clone()], &progress)
                .unwrap()
                .id,
            evening.id
        );
        let restricted = EncounterDefinition {
            min_prestige: Some(1),
            ..evening
        };
        assert!(OpponentProvider::select(&[restricted], &progress).is_none());
    }

    #[test]
    fn progression_and_settings_serialize_independently() {
        let progress = PlayerProgress::default();
        let settings = ApplicationSettings {
            left_handed_controls: true,
            ..default()
        };
        let progress_json = serde_json::to_string(&progress).unwrap();
        let settings_json = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            serde_json::from_str::<PlayerProgress>(&progress_json).unwrap(),
            progress
        );
        assert!(
            serde_json::from_str::<ApplicationSettings>(&settings_json)
                .unwrap()
                .left_handed_controls
        );
    }

    #[test]
    fn save_file_can_be_replaced_after_initial_write() {
        let path = std::env::temp_dir().join(format!("lotus-ccg-save-{}.json", std::process::id()));
        save_json(&path, &PlayerProgress::default()).unwrap();
        let mut progress = PlayerProgress::default();
        progress.money = Money::new(42);
        save_json(&path, &progress).unwrap();
        assert_eq!(
            load_json::<PlayerProgress>(&path).unwrap().unwrap().money(),
            Money::new(42)
        );
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn malformed_save_data_is_reported_instead_of_deserialized_as_defaults() {
        let path = std::env::temp_dir().join(format!(
            "lotus-ccg-invalid-save-{}.json",
            std::process::id()
        ));
        fs::write(&path, b"{").unwrap();

        assert!(load_json::<PlayerProgress>(&path).is_err());
        fs::remove_file(path).unwrap();
    }
}
