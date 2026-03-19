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

## Pair Bot

We are shifting Code review to the left. That means that the bot will focus on helping us see mistakes, think about things commonly forgotten, and keep us accountable. It will also be a pair, so we can use it as a rubber duck.

- [ ] Visualize how much the session is costing

- [ ] Set up 'team' norms
  - [ ] Rules of how we work together
  - [ ] Personality to have
  - [ ] How to ask questions
- [ ] Onboard onto the project
  - [ ] Read the readme
  - [ ] Ask any questions
  - [ ] Take notes
- [ ] Doing the work
  - [ ] Discuss the task we're doing
  - [ ] Choose how we're going to test this (automated or manual)
  - [ ] Write the code
  - [ ] Run the test (manual and/or automatic)
  - [ ] commit to github with a good commit message
  - [ ] Refactor
  - [ ] Linting passes
  - [ ] Document the changes
  - [ ] Celebration

### To Do

- [x] Add command to reset context
- [ ] Whitelist directories that the agent has access to

## Polish

- [x] Context usage
  - [x] Use a visual percentage bar to show context usage
  - [x] User color with context usage
- [ ] Create Anathema frontend for the pair bot
