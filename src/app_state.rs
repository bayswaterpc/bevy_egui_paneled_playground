
// NOTE: this "state based" approach to multiple windows is a short term workaround.
// Future Bevy releases shouldn't require such a strict order of operations.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    CentralPanelState,
    GameState,
}
