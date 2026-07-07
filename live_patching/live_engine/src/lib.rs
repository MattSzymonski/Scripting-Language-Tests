//! # live_engine — Function-level live-patching engine
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │  Orchestrator (orchestrator/)                             │
//! │  • Builds game_hot cdylib                                 │
//! │  • Loads function symbols via libloading                  │
//! │  • Watches game source files                              │
//! │  • Triggers recompile + repatch on change                 │
//! └────────────┬─────────────────────────────────────────────┘
//!              │ stores fn ptrs in
//! ┌────────────▼─────────────────────────────────────────────┐
//! │  live_engine (this crate)                                 │
//! │  ┌─────────────────┐  ┌──────────────────────────────┐   │
//! │  │ HotMemory        │  │ HotFnTable                   │   │
//! │  │ • alloc_rwx()    │  │ • AtomicPtr<GameUpdateFn>    │   │
//! │  │ • VirtualAlloc   │  │ • AtomicPtr<GameRenderFn>    │   │
//! │  │ on Windows       │  │ • swap() atomically          │   │
//! │  └─────────────────┘  └──────────┬───────────────────┘   │
//! │                                  │ engine calls through   │
//! │  Engine::run() ─────────────────┘ this table every frame  │
//! └──────────────────────────────────────────────────────────┘
//!              │ calls via opaque ABI
//! ┌────────────▼─────────────────────────────────────────────┐
//! │  game_hot/  (cdylib — compiled & reloaded at runtime)     │
//! │  extern "C" fn game_update(api, world, dt)                │
//! │  extern "C" fn game_render(api, world)                    │
//! └──────────────────────────────────────────────────────────┘
//! ```

pub mod api;
pub mod engine;
pub mod hot_memory;
pub mod hot_table;
