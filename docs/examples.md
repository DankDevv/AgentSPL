# Examples

## Basic Agent

```javascript
import { AgentBuilder } from 'agentspl';

// Create a basic agent with conversation capability
const agent = new AgentBuilder()
  .withFeature('conversation')
  .build();

await agent.start();
```

## Advanced Agent

```javascript
// Create an advanced agent with multiple features
const agent = new AgentBuilder({
  name: 'AdvancedAgent',
  version: '1.0.0'
})
  .withFeature('conversation', {
    language: 'en',
    model: 'gpt-4'
  })
  .withFeature('planning', {
    algorithm: 'hierarchical'
  })
  .withFeature('learning', {
    mode: 'continuous'
  })
  .build();

await agent.start();
```

## Custom Plugin

```javascript
// Define a custom plugin
class CustomPlugin {
  name = 'custom-plugin';
  version = '1.0.0';

  async install(agent) {
    // Plugin installation logic
  }

  async uninstall() {
    // Plugin cleanup logic
  }
}

// Create an agent with the custom plugin
const agent = new AgentBuilder()
  .withFeature('conversation')
  .build();

await agent.addPlugin(new CustomPlugin());
```

## Event Handling

```javascript
// Handle agent events
agent.on('system:started', () => {
  console.log('Agent started');
});

agent.on('feature:loaded', (feature) => {
  console.log(`Feature ${feature.name} loaded`);
});

// Handle errors
agent.on('system:error', (error) => {
  console.error('Agent error:', error);
});
```

## Feature Configuration

```javascript
// Configure features with specific settings
const agent = new AgentBuilder()
  .withFeature('conversation', {
    language: 'en',
    model: 'gpt-4',
    contextWindow: 10,
    responses: {
      greeting: 'Hello, how can I help you?',
      farewell: 'Goodbye, have a great day!'
    }
  })
  .withFeature('planning', {
    algorithm: 'hierarchical',
    maxSteps: 100,
    timeout: 5000,
    goals: {
      priority: 'high',
      concurrent: true
    }
  })
  .build();
```