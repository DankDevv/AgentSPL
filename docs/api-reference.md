# API Reference

## AgentBuilder

### Constructor

```typescript
class AgentBuilder {
  constructor(config?: AgentConfig);
}
```

### Methods

#### withFeature

```typescript
withFeature(name: string, config?: FeatureConfig): AgentBuilder
```

Adds a feature to the agent configuration.

#### build

```typescript
build(): Agent
```

Creates an agent instance with the configured features.

## Agent

### Methods

#### start

```typescript
async start(): Promise<void>
```

Starts the agent and initializes all features.

#### stop

```typescript
async stop(): Promise<void>
```

Stops the agent and cleans up resources.

#### addPlugin

```typescript
async addPlugin(plugin: Plugin): Promise<void>
```

Adds a plugin to the running agent.

## Feature Interface

```typescript
interface Feature {
  name: string;
  version: string;
  init(): Promise<void>;
  start(): Promise<void>;
  stop(): Promise<void>;
}
```

## Plugin Interface

```typescript
interface Plugin {
  name: string;
  version: string;
  install(agent: Agent): Promise<void>;
  uninstall(): Promise<void>;
}
```

## Events

### System Events

```typescript
enum SystemEvent {
  STARTED = 'system:started',
  STOPPED = 'system:stopped',
  ERROR = 'system:error'
}
```

### Feature Events

```typescript
enum FeatureEvent {
  LOADED = 'feature:loaded',
  ENABLED = 'feature:enabled',
  DISABLED = 'feature:disabled'
}
```