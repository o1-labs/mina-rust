Derives `[openmina_core::ActionEvent]` trait implementation for action.

### Action containers

For action containers, it simply delegates to inner actions.

```rust
use openmina_core::{ActionEvent, log::EventContext};

struct MockContext;

impl EventContext for MockContext {
    fn timestamp(&self) -> redux::Timestamp { redux::Timestamp::global_now() }
    fn time(&self) -> &dyn tracing::Value { &"test" }
    fn node_id(&self) -> &dyn tracing::Value { &"test_node" }
    fn log_node_id(&self) -> bool { true }
}

let context = MockContext;

#[derive(ActionEvent)]
enum ActionContainer {
    SubAction1(Action1),
}

#[derive(ActionEvent)]
enum Action1 {
    Init,
    Done,
}

ActionContainer::SubAction1(Action1::Init).action_event(&context);
```

```rust
use openmina_core::{ActionEvent, log::EventContext};

enum ActionContainer { SubAction1(Action1) }

enum Action1 { Init, Done }

impl ActionEvent for ActionContainer {
    fn action_event<T>(&self, context: &T)
    where T: EventContext
    {
        match self {
            ActionContainer::SubAction1(action) => action.action_event(context),
        }
    }
}

impl ActionEvent for Action1 {
    fn action_event<T>(&self, context: &T)
    where T: EventContext
    {
        match self {
            Action1::Init => openmina_core::action_debug!(context),
            Action1::Done => openmina_core::action_debug!(context),
        }
    }
}
```

### Tracing level

By default, tracing event of level `debug` is generated for an action. It can be
overriden by using `#[action_event(level = ...)]` attribute. Also, actions that
names ends with `Error` or `Warn` will be traced with `warn` level.

```rust
use openmina_core::ActionEvent;

#[derive(ActionEvent)]
#[action_event(level = trace)]
pub enum Action {
    ActionDefaultLevel,
    #[action_event(level = warn)]
    ActionOverrideLevel,
    ActionWithError,
    ActionWithWarn,
}
```

```rust
use openmina_core::{ActionEvent, log::EventContext};

enum Action { ActionDefaultLevel, ActionOverrideLevel, ActionWithError, ActionWithWarn }

impl ActionEvent for Action {
    fn action_event<T>(&self, context: &T)
    where
        T: EventContext,
    {
        match self {
            Action::ActionDefaultLevel => openmina_core::action_trace!(context),
            Action::ActionOverrideLevel => openmina_core::action_warn!(context),
            Action::ActionWithError => openmina_core::action_warn!(context),
            Action::ActionWithWarn => openmina_core::action_warn!(context),
        }
    }
}
```

### Summary field

If an action has doc-comment, its first line will be used for `summary` field of
tracing events for the action.

```rust
# use openmina_core::ActionEvent;
#[derive(ActionEvent)]
pub enum Action {
    Unit,
    /// documentation
    UnitWithDoc,
    /// Multiline documentation.
    /// Another line.
    ///
    /// And another.
    UnitWithMultilineDoc,
}
```

```rust
use openmina_core::{ActionEvent, log::EventContext};

enum Action { Unit, UnitWithDoc, UnitWithMultilineDoc }

impl ActionEvent for Action {
    fn action_event<T>(&self, context: &T)
    where
        T: EventContext,
    {
        match self {
            Action::Unit => openmina_core::action_debug!(context),
            Action::UnitWithDoc => openmina_core::action_debug!(context, summary = "documentation"),
            Action::UnitWithMultilineDoc => openmina_core::action_debug!(context, summary = "Multiline documentation"),
        }
    }
}
```

### Fields

Certain fields can be added to the tracing event, using
`#[action_event(fields(...))]` attribute.

```rust
use openmina_core::ActionEvent;

#[derive(ActionEvent)]
pub enum Action {
    NoFields { f1: bool },
    #[action_event(fields(f1))]
    Field { f1: bool },
    #[action_event(fields(f = f1))]
    FieldWithName { f1: bool },
    #[action_event(fields(debug(f1)))]
    DebugField { f1: bool },
    #[action_event(fields(display(f1)))]
    DisplayField { f1: bool },
}
```

```rust
use openmina_core::{ActionEvent, log::EventContext};
enum Action {
  NoFields { f1: bool },
  Field { f1: bool },
  FieldWithName { f1: bool },
  DebugField { f1: bool },
  DisplayField { f1: bool }
}
impl ActionEvent for Action {
    fn action_event<T>(&self, context: &T)
    where
        T: EventContext,
    {
        match self {
            Action::NoFields { f1 } => openmina_core::action_debug!(context),
            Action::Field { f1 } => openmina_core::action_debug!(context, f1 = f1),
            Action::FieldWithName { f1 } => openmina_core::action_debug!(context, f = f1),
            Action::DebugField { f1 } => openmina_core::action_debug!(context, f1 = debug(f1)),
            Action::DisplayField { f1 } => openmina_core::action_debug!(context, f1 = display(f1)),
        }
    }
}
```

### Logging using custom expression.

When an action needs some custom logic to log (e.g. different logging basing on
a field's enum variant), logging can be delegated to a function implementing
that logic.

```rust
use openmina_core::ActionEvent;

fn foo<T: openmina_core::log::EventContext>(_: &T) {}

fn bar<T: openmina_core::log::EventContext>(_: &T, _: &bool) {}

#[derive(ActionEvent)]
pub enum Action {
    #[action_event(expr(foo(context)))]
    Unit,
    #[action_event(expr(bar(context, f1)))]
    Named { f1: bool },
}
```

```rust
use openmina_core::{ActionEvent, log::EventContext};

fn foo<T: EventContext>(_: &T) {}

fn bar<T: EventContext>(_: &T, _: &bool) {}

enum Action { Unit, Named { f1: bool } }

impl ActionEvent for Action {
    fn action_event<T>(&self, context: &T)
    where
        T: EventContext,
    {
        #[allow(unused_variables)]
        match self {
            Action::Unit => foo(context),
            Action::Named { f1 } => bar(context, f1),
        }
    }
}
```
