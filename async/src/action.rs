#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Quit,
  Tick,
  Resize(u16, u16),
  ToggleShowLogger,
  ScheduleIncrementCounter,
  ScheduleDecrementCounter,
  AddToCounter(usize),
  SubtractFromCounter(usize),
  EnterNormal,
  EnterInsert,
  EnterProcessing,
  ExitProcessing,
  Update,
  Noop,
}
