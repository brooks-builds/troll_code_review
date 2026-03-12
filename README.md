# Troll Code Review

This is an agentic AI code review tool, not meant to be serious, but rather pair with Brooks on stream and "help" him code.

## Background

AI agentic tools can improve the speed of code generation, this isn't in question. However, there is a question of if code generation speed is the actual problem that needs solving. Sometimes, the bottleneck is really QA, and ensuring that best practices are being followed.

This agent could help with that, but is actually going to troll me instead. However the idea is sound, if you want to take this and turn it into something actually helpful, please feel free to let me know how it goes.

## What this does

- controller agent
  - personality
  - goal
    - watch code as its written, ensure "quality" is good
    - communicate back with the dev
  - sub agents
    - find what files to look at

- tools
  - search for files that have been recently edited
  - read file
  - continue without doing anything this iteration (print elipses)

## Monitoring

- We'll want to be able to monitor what the agents are doing, even if that's nothing

## Crawl/Walk/Run

### Crawl

- [ ] Basic agent
  - [ ] Can chat with it
  - [ ] Can run a tool
