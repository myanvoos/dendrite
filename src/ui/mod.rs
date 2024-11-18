//! # UI
//! 
//! UI for dendrite

use tuirealm::{
  terminal::TerminalBridge, Application, AttrValue, Attribute, NoUserEvent, StateValue, Update
};

// Defines the identifiers for components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
  GlobalListener,
  IssueList,
  IssueTitle,
  IssueDate,
  IssueAuthor,
  IssueSummary,
  IssueLink,
  IssueSuggested,
  QuitPopup,
  ErrorPopup
}

// Defines messages produced by components
#[derive(Debug, PartialEq, Eq)] 
pub enum Message {
  CloseErrorPopup,
  CloseQuitPopup,
  Quit,
  None,
}

// Defines user events - actions triggered by user inputs
//
// Tab - Toggling graph/issue views
// Enter/Spacebar - Select a node or issue to view details
// F5 - Refresh the list of issues (refetch from GitHub)
// + - Zoom in. Similarly for (-)
// z - Fit graph to terminal window
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)] 
pub enum UserEvents {
  GraphView,
  IssueView,
  Select,
  Refresh,
  ZoomIn,
  ZoomOut,
  FitGraph
}

pub struct UI {
  application: Application<Id, Message, UserEvents>,
  redraw: bool,
}

