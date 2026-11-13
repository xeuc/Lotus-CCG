# Game Architecture

## 1. Project Overview

### 1.1. General Concept

The game is a card collection and real-time battle game developed in Rust using Bevy.

The player collects cards, builds decks, interacts with characters, trades cards, and participates in real-time battles.

The ultimate goal is to collect every available card.

The main environment is a first-person 3D desk where the player interacts with different characters and objects.

### 1.2. Implementation Approach

The project is developed incrementally, with individual modules initially maintained in separate repositories and manually integrated later.

The implementation should use Bevy ECS where it provides practical benefits, without forcing every gameplay feature into ECS.

Existing implementations should be inspected before making changes. This document describes the intended architecture and behavior, not necessarily the exact state of the current prototype.

---

## 2. Loading State

The application starts in the Loading State.

It loads the main 3D environment, models, textures, card assets, and other resources required by the game.

Once the required assets are ready, the application transitions to the Menu State.

The Loading State also provides a way to reinitialize the menu environment when necessary.

The current implementation uses this state because returning from other states sometimes causes the environment to appear after the HUD, resulting in a temporary incomplete screen.

This behavior should be investigated. Assets should ideally remain loaded across state transitions when possible, avoiding unnecessary reloads.

---

## 3. Menu State

### 3.1. General Concept

The Menu State is the main gameplay environment.

The player is positioned behind a desk and sees the environment from a first-person perspective.

The environment is currently implemented using a 3D model and should remain 3D for the initial development.

There are four interaction areas around the player: North, East, South, and West.

### 3.2. Navigation

Two arrows are displayed at the bottom of the screen.

Clicking the right arrow rotates the camera 90 degrees clockwise.

Clicking the left arrow rotates the camera 90 degrees counterclockwise.

Navigation is circular and can continue indefinitely.

The current camera direction determines the displayed interaction area.

### 3.3. North – Opponents

The North direction represents the main interaction area.

Characters approach the player and request different interactions.

The most common interactions are battles and card trades. Other possibilities include giving cards, offering money, or requesting donations.

The current character stands in front of the player. Additional characters may appear behind them, representing the upcoming encounters.

The player can accept or reject the current request.

Swiping down accepts the request, while swiping up dismisses the character and moves to the next one.

These gestures are contextual and may have different meanings in other interfaces.

Characters are generated dynamically according to the player's current situation.

Relevant parameters include Karma, money, Prestige, difficulty, and in-game time.

The Opponent Provider determines which encounters are appropriate for the current player.

### 3.4. East – Card Collection

The East direction provides access to the player's card collection and deck management.

The current visual concept uses floating vertical books to represent the collection and active deck.

The exact interaction design is still being evaluated.

One possible implementation is to display two separate 3D books on the desk.

Clicking the collection book opens the full collection interface.

Clicking the deck book opens the deck management interface.

This approach would provide dedicated interfaces without the navigation arrows and shared HUD obstructing card management.

The deck-building experience should avoid unnecessary scrolling and repetitive card selection.

Cards should be easy to inspect, add, remove, and replace.

The design must also account for the fact that the cards are used in real-time battles, rather than in a simple automated turn-based card game.

### 3.5. West – Card Packs

The West direction provides access to the card pack shop.

Card packs are displayed floating above the desk.

The player can browse available packs using vertical swipe gestures.

Swiping up selects the previous pack, while swiping down selects the next pack.

Each pack has its own price.

Selecting a pack and pressing the Open button starts the pack-opening interaction.

### 3.6. South – Exit and Prestige

The South direction contains the application exit door.

Selecting Exit closes the application.

Prestige is represented separately by a blue light coming from above.

This light is initially absent.

Once the player unlocks Prestige, the light becomes available and the player can swipe upwards to access the Prestige Shop.

The Prestige Shop can be accessed independently of the regular card collection and battle interfaces.

---

## 4. Settings Overlay

### 4.1. General Concept

Settings are accessed through a cog icon displayed near the bottom-right navigation controls.

The icon opens a phone interface.

The phone is simply the visual representation of the game's settings menu. It does not represent an actual in-game phone interaction.

The phone interface is an overlay and should not require a separate gameplay State.

### 4.2. Availability

The settings icon should remain accessible across all relevant game interfaces.

Opening the phone must not destroy or reset the current interface.

Closing it should restore the previous screen and its interaction context.

### 4.3. Options

The phone contains application settings such as sound configuration and input configuration.

Additional options can be introduced later.

The interface should support scrolling as more settings become available.

### 4.4. Accessibility

The settings icon should remain within the comfortable thumb-reach area of the screen.

The default position is above the right navigation arrow.

An accessibility option should allow repositioning the icon to the left side for left-handed players.

---

## 5. In-Game Time

### 5.1. General Concept

The game uses an abstract in-game clock.

Time does not advance continuously in real time.

Each completed interaction advances the in-game clock.

The player does not wait for the clock to advance.

### 5.2. Daily Cycle

Each day contains three periods: morning, afternoon, and evening.

Each period allows five interactions.

After fifteen interactions, the next day begins.

The cycle repeats indefinitely.

### 5.3. Time-Based Encounters

The current period influences which encounters can appear.

For example, stronger opponents or less trustworthy traders may appear during the evening.

The time is one of the parameters used by the Opponent Provider.

---

## 6. Gameplay Data and Systems

These are gameplay-level structures and services, not necessarily Bevy ECS Systems.

They are responsible for managing shared gameplay rules and data.

### 6.1. Money

Money represents the player's in-game currency.

It can be earned through battles and other interactions.

It is primarily spent on card packs and other purchases.

All transactions should go through a centralized money interface.

The interface must validate transactions before modifying the balance.

For example, attempting to purchase a pack without sufficient money should return an error.

The calling interface is responsible for displaying the appropriate feedback.

If the player has three units of currency and attempts to purchase a five-unit pack, the UI should highlight the current balance and display a temporary insufficient-funds message.

### 6.2. Karma

Karma represents the player's behavior and decisions.

It is expressed as a percentage and is intentionally difficult to maximize.

Karma is primarily influenced by trades, donations, and battle outcomes.

Accepting an unfair trade increases Karma, even if the player gives away a valuable card.

Giving money to someone in need can also increase Karma.

Winning a battle decreases Karma.

Losing a battle increases Karma without causing the player to lose money.

Karma is used to regulate the encounters offered to the player.

Its progression should be handled by a dedicated Karma structure rather than allowing arbitrary modifications throughout the game.

Positive Karma gains should approach 100% asymptotically rather than allowing the value to reach 100%.

For example, a gain of 10% when Karma is already at 99% should result in 99.9%, not 100%.

### 6.3. Prestige

Prestige is a separate progression currency.

It is unlocked through a collection-based progression mechanic.

Once the player reaches the required collection completion threshold, they can exchange their entire card collection for Prestige progression.

This resets the collection while granting Prestige points.

Prestige points can be spent in the dedicated Prestige Shop.

The shop contains special upgrades and opportunities that are unavailable through regular progression.

Examples include access to rare card packs and improved card quality opportunities.

### 6.4. Card Data

Cards have individual properties, including rarity and PSA grading.

The player's collection must track which cards they own and their individual properties.

One of the long-term collection objectives is to obtain cards with PSA 10 grading.

The card data structure must support collection management, deck building, trading, and battle integration.

### 6.5. Opponent Provider

The Opponent Provider generates the characters and encounters offered to the player.

It considers Karma, money, Prestige, difficulty, and the current in-game time.

Its purpose is to provide an effectively endless sequence of encounters that adapts to the player's current situation.

For example, a player with insufficient money may receive more opportunities to earn money through battles.

A player with low Karma may receive more opportunities involving trades or other Karma-related interactions.

The system should avoid offering interactions that are irrelevant to the player's current situation.

### 6.6. Event Definitions

Opponent and interaction definitions should be data-driven.

YAML is a potential format for defining encounters.

Each definition may contain the opponent type, associated deck, available interaction, time restrictions, conditions, and possible rewards.

Conditions may reference player properties such as Karma, money, Prestige, or other progression values.

Rewards may define currency changes, percentage-based Karma changes, card rewards, or other effects.

The Opponent Provider is responsible for filtering and selecting appropriate definitions.

For example, encounters restricted to the evening should not be considered during the morning.

The selection algorithm should also consider the player's current needs rather than relying exclusively on fixed thresholds.

The exact YAML schema and selection algorithm will be developed separately.

---

## 7. Persistence

Player progression and application configuration should be treated as separate persistence concerns.

Gameplay persistence includes the card collection, active deck, money, Karma, Prestige, and other progression data.

Application configuration includes sound settings, input preferences, and accessibility options.

The two categories may use different serialization approaches and storage mechanisms.

The implementation should use the appropriate Bevy facilities or other Rust serialization tools for each category.