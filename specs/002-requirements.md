# Requirements: Distributed Issue Tracking System

**Input**: Design documents from `specs/001-distributed-issue-tracking/`

Several tasks were implemented in the [previous
specification](001-distributed-issue-tracking/tasks.md), but a lot of core
functionality remains. This specification will cover the remaining core
requirements.

## Known Issues

- Creating an issue with `-p, --project` flag does not work, the issue is created on the default project
- Listing issues with `-p, --project` flag does not work, all issues are listed for any project ID
- Listing issues with `--status` flag does not work, all issues are listed
- `odi issue show` is not implemented
- `odi issue list` does not show the assignee, and `show` is not implemented
- `odi issue label` is not implemented
- All `odi label` commands fail when a valid project ID is provided

## Requirements

### 1. Project Commands

#### 1.1. Create Project

- [ ] **T001** [P] Create project `odi project create "name" --description ""`
- [ ] **T002** [P] Create project with ID `odi project create "name" --id "id"`
- [ ] **T003** [P] Create project with ID and description `odi project create "name" --id "id" --description ""`

#### 1.2. List Projects

- [ ] **T004** [P] List all projects `odi project list`
- [ ] **T005** [P] List projects with ID `odi project list --id "id"`
- [ ] **T006** [P] List projects with description `odi project list --description "description"`
- [ ] **T007** [P] List projects with ID and description `odi project list --id "id" --description "description"`

### 2. Issue Commands

#### 2.1. Create Issue

- [ ] **T008** [P] Create issue on the default project `odi issue create "title" --description ""`
- [ ] **T009** [P] Create issue with ID on the default project `odi issue create "title" --id "id"`
- [ ] **T010** [P] Create issue with ID and description on the default project `odi issue create "title" --id "id" --description ""`
- [ ] **T011** [P] Create issue with project ID `odi issue create "title" --project "id"`

#### 2.2. List Issues

- [ ] **T013** [P] List all issues on the default project `odi issue list`
- [ ] **T014** [P] List issues with ID on the default project `odi issue list --id "id"`
- [ ] **T015** [P] List issues with description on the default project `odi issue list --description "description"`
- [ ] **T016** [P] List issues with ID and description on the default project `odi issue list --id "id" --description "description"`
- [ ] **T017** [P] List issues with project ID `odi issue list --project "id"`

### 3. Push/Pull Commands

- [ ] **T018** [P] Push all changes from the default local project to the default remote `odi remote push`
- [ ] **T019** [P] Pull all changes from the default remote project to the default local `odi remote pull`
- [ ] **T020** [P] Push all changes from the specified local project to the corresponding remote `odi remote push --project "id"`
- [ ] **T021** [P] Pull all changes from the specified remote project to the corresponding local `odi remote pull --project "id"`
