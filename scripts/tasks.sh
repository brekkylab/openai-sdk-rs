#!/usr/bin/env bash

# Development task management script
# Usage: ./scripts/tasks.sh [command]

set -euo pipefail

TASKS_FILE=".tasks.json"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Initialize task file
init_tasks() {
    if [ ! -f "$TASKS_FILE" ]; then
        cat > "$TASKS_FILE" << 'EOF'
{
  "tasks": [],
  "completed": [],
  "next_id": 1
}
EOF
        log_success "Task management file initialized"
    fi
}

# Add task
add_task() {
    local description="$*"
    if [ -z "$description" ]; then
        log_error "Please provide task description"
        exit 1
    fi
    
    init_tasks
    
    # Use jq to add task
    if command -v jq &> /dev/null; then
        local id=$(jq -r '.next_id' "$TASKS_FILE")
        local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
        
        jq --arg desc "$description" --arg ts "$timestamp" --argjson id "$id" \
           '.tasks += [{"id": $id, "description": $desc, "created": $ts}] | .next_id += 1' \
           "$TASKS_FILE" > "$TASKS_FILE.tmp" && mv "$TASKS_FILE.tmp" "$TASKS_FILE"
        
        log_success "Added task #$id: $description"
    else
        log_warning "jq not installed, using simple mode"
        echo "- [ ] $description ($(date))" >> tasks.md
        log_success "Task added to tasks.md"
    fi
}

# List tasks
list_tasks() {
    init_tasks
    
    if command -v jq &> /dev/null; then
        local pending=$(jq -r '.tasks | length' "$TASKS_FILE")
        local completed=$(jq -r '.completed | length' "$TASKS_FILE")
        
        echo -e "${BLUE}=== Pending Tasks ($pending) ===${NC}"
        jq -r '.tasks[] | "[\(.id)] \(.description)"' "$TASKS_FILE" | while read -r line; do
            echo "  $line"
        done
        
        echo
        echo -e "${GREEN}=== Completed Tasks ($completed) ===${NC}"
        jq -r '.completed[] | "[\(.id)] \(.description)"' "$TASKS_FILE" | while read -r line; do
            echo "  ✓ $line"
        done
    else
        if [ -f tasks.md ]; then
            cat tasks.md
        else
            log_info "No tasks yet"
        fi
    fi
}

# Complete task
complete_task() {
    local task_id="$1"
    if [ -z "$task_id" ]; then
        log_error "Please provide task ID"
        exit 1
    fi
    
    init_tasks
    
    if command -v jq &> /dev/null; then
        local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
        
        # Check if task exists
        local task_exists=$(jq --argjson id "$task_id" '.tasks | any(.id == $id)' "$TASKS_FILE")
        if [ "$task_exists" = "false" ]; then
            log_error "Task #$task_id does not exist"
            exit 1
        fi
        
        # Move task to completed
        jq --argjson id "$task_id" --arg ts "$timestamp" \
           '(.tasks[] | select(.id == $id)) as $task | 
            .completed += [$task + {"completed": $ts}] | 
            .tasks = [.tasks[] | select(.id != $id)]' \
           "$TASKS_FILE" > "$TASKS_FILE.tmp" && mv "$TASKS_FILE.tmp" "$TASKS_FILE"
        
        log_success "Task #$task_id completed"
    else
        log_warning "Please manually mark task as completed in tasks.md"
    fi
}

# Development status check
dev_status() {
    log_info "=== Development Status Overview ==="
    
    # Git status
    echo -e "\n${BLUE}Git Status:${NC}"
    if [[ -n $(git status --porcelain) ]]; then
        echo "  ⚠️  Uncommitted changes"
        git status --short | head -5
    else
        echo "  ✅ Clean working directory"
    fi
    
    # Branch info
    local branch=$(git branch --show-current)
    echo "  📍 Current branch: $branch"
    
    # Recent commits
    echo -e "\n${BLUE}Recent Commits:${NC}"
    git log --oneline -3 | sed 's/^/  /'
    
    # Pending tasks
    echo -e "\n${BLUE}Pending Tasks:${NC}"
    if command -v jq &> /dev/null && [ -f "$TASKS_FILE" ]; then
        local pending=$(jq -r '.tasks | length' "$TASKS_FILE")
        echo "  📝 Pending tasks: $pending"
        if [ "$pending" -gt 0 ]; then
            jq -r '.tasks[0:3][] | "    - [\(.id)] \(.description)"' "$TASKS_FILE"
            if [ "$pending" -gt 3 ]; then
                echo "    ... and $((pending - 3)) more tasks"
            fi
        fi
    else
        echo "  📝 Use './scripts/tasks.sh list' to view tasks"
    fi
    
    # Build status
    echo -e "\n${BLUE}Build Status:${NC}"
    if cargo check --quiet 2>/dev/null; then
        echo "  ✅ Compilation successful"
    else
        echo "  ❌ Compilation failed"
    fi
    
    # Test status
    echo -e "\n${BLUE}Recommended Actions:${NC}"
    echo "  🚀 Quick check: ./scripts/dev-workflow.sh quick"
    echo "  🧪 Run tests: ./scripts/dev-workflow.sh test"
    echo "  📦 Release version: ./scripts/quick-release.sh patch"
}

# Show help
show_help() {
    cat << 'EOF'
Development Task Management

Usage:
  ./scripts/tasks.sh <command> [options]

Commands:
  add <description>   Add new task
  list               List all tasks
  done <id>          Complete task
  status             Show development status overview
  help               Show help information

Examples:
  ./scripts/tasks.sh add "Implement new API endpoint"
  ./scripts/tasks.sh list
  ./scripts/tasks.sh done 1
  ./scripts/tasks.sh status

Note: Install jq for full functionality
  brew install jq  # macOS
  
EOF
}

# Main function
main() {
    local command="${1:-help}"
    
    case "$command" in
        "add")
            shift
            add_task "$@"
            ;;
        "list"|"ls")
            list_tasks
            ;;
        "done"|"complete")
            complete_task "${2:-}"
            ;;
        "status"|"st")
            dev_status
            ;;
        "help"|"--help"|"-h")
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            echo
            show_help
            exit 1
            ;;
    esac
}

main "$@"
