#!/bin/bash
# Claude Flow Production Backup Script
# Comprehensive backup solution with encryption and cloud sync

set -euo pipefail

# Configuration
BACKUP_DIR="${CLAUDE_BACKUP_DIR:-/app/backups}"
DATA_DIR="${CLAUDE_FLOW_DATA_DIR:-/app/data}"
CONFIG_DIR="${CLAUDE_FLOW_CONFIG_DIR:-/app/config}"
LOG_DIR="${CLAUDE_FLOW_LOG_DIR:-/var/log/claude-flow}"

# Backup settings
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-30}"
ENCRYPTION_KEY="${BACKUP_ENCRYPTION_KEY:-}"
S3_BUCKET="${BACKUP_S3_BUCKET:-}"
COMPRESSION_LEVEL="${BACKUP_COMPRESSION:-6}"

# Timestamps
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DATE=$(date +%Y-%m-%d)

# Logging
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [BACKUP] $1" | tee -a "$LOG_DIR/backup.log"
}

error() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $1" | tee -a "$LOG_DIR/backup.log" >&2
    exit 1
}

# Create backup directory structure
setup_backup_dirs() {
    log "Setting up backup directory structure..."
    
    mkdir -p "$BACKUP_DIR"/{daily,weekly,monthly}
    mkdir -p "$BACKUP_DIR/temp"
    mkdir -p "$LOG_DIR"
    
    # Set secure permissions
    chmod 700 "$BACKUP_DIR"
    chmod 755 "$LOG_DIR"
}

# Database backup
backup_database() {
    log "Starting database backup..."
    
    local db_backup="$BACKUP_DIR/temp/database_$TIMESTAMP.sql"
    
    if [[ -f "$DATA_DIR/claude-flow.db" ]]; then
        # SQLite backup using .dump command
        sqlite3 "$DATA_DIR/claude-flow.db" ".dump" > "$db_backup" || error "Database backup failed"
        
        # Compress the backup
        gzip -$COMPRESSION_LEVEL "$db_backup" || error "Database compression failed"
        
        log "Database backup completed: ${db_backup}.gz"
    else
        log "Warning: Database file not found at $DATA_DIR/claude-flow.db"
    fi
}

# Memory store backup
backup_memory_store() {
    log "Starting memory store backup..."
    
    local memory_backup="$BACKUP_DIR/temp/memory_store_$TIMESTAMP.json"
    
    if [[ -f "$DATA_DIR/memory-store.json" ]]; then
        cp "$DATA_DIR/memory-store.json" "$memory_backup" || error "Memory store backup failed"
        gzip -$COMPRESSION_LEVEL "$memory_backup" || error "Memory store compression failed"
        
        log "Memory store backup completed: ${memory_backup}.gz"
    else
        log "Warning: Memory store file not found"
    fi
}

# Configuration backup
backup_configuration() {
    log "Starting configuration backup..."
    
    local config_backup="$BACKUP_DIR/temp/config_$TIMESTAMP.tar.gz"
    
    if [[ -d "$CONFIG_DIR" ]]; then
        tar -czf "$config_backup" -C "$(dirname "$CONFIG_DIR")" "$(basename "$CONFIG_DIR")" || error "Configuration backup failed"
        
        log "Configuration backup completed: $config_backup"
    else
        log "Warning: Configuration directory not found"
    fi
}

# Application state backup
backup_application_state() {
    log "Starting application state backup..."
    
    local state_backup="$BACKUP_DIR/temp/app_state_$TIMESTAMP.tar.gz"
    local state_dirs=()
    
    # Collect state directories that exist
    for dir in "swarms" "agents" "neural" "workflows"; do
        if [[ -d "$DATA_DIR/$dir" ]]; then
            state_dirs+=("$dir")
        fi
    done
    
    if [[ ${#state_dirs[@]} -gt 0 ]]; then
        tar -czf "$state_backup" -C "$DATA_DIR" "${state_dirs[@]}" || error "Application state backup failed"
        
        log "Application state backup completed: $state_backup"
    else
        log "Warning: No application state directories found"
    fi
}

# Log files backup
backup_logs() {
    log "Starting log files backup..."
    
    local logs_backup="$BACKUP_DIR/temp/logs_$TIMESTAMP.tar.gz"
    
    if [[ -d "$LOG_DIR" ]] && [[ "$(ls -A "$LOG_DIR" 2>/dev/null)" ]]; then
        # Backup logs excluding the current backup log
        tar -czf "$logs_backup" --exclude="backup.log" -C "$(dirname "$LOG_DIR")" "$(basename "$LOG_DIR")" || error "Log backup failed"
        
        log "Log files backup completed: $logs_backup"
    else
        log "Warning: No log files found to backup"
    fi
}

# Encrypt backup files
encrypt_backups() {
    if [[ -n "$ENCRYPTION_KEY" ]]; then
        log "Encrypting backup files..."
        
        for file in "$BACKUP_DIR"/temp/*.{gz,tar.gz} 2>/dev/null; do
            if [[ -f "$file" ]]; then
                openssl enc -aes-256-cbc -salt -in "$file" -out "${file}.enc" -k "$ENCRYPTION_KEY" || error "Encryption failed for $file"
                rm "$file" # Remove unencrypted file
                log "Encrypted: $(basename "$file")"
            fi
        done
    else
        log "Warning: No encryption key provided, backups will be stored unencrypted"
    fi
}

# Create final backup archive
create_final_archive() {
    log "Creating final backup archive..."
    
    local final_backup="$BACKUP_DIR/daily/claude_flow_backup_$TIMESTAMP.tar.gz"
    
    # Create the final archive
    tar -czf "$final_backup" -C "$BACKUP_DIR/temp" . || error "Failed to create final backup archive"
    
    # Calculate and store checksum
    sha256sum "$final_backup" > "${final_backup}.sha256" || error "Failed to create checksum"
    
    log "Final backup archive created: $final_backup"
    log "Backup size: $(du -h "$final_backup" | cut -f1)"
    
    # Clean up temporary files
    rm -rf "$BACKUP_DIR/temp"/*
    
    echo "$final_backup"
}

# Sync to cloud storage
sync_to_cloud() {
    local backup_file="$1"
    
    if [[ -n "$S3_BUCKET" ]] && command -v aws >/dev/null 2>&1; then
        log "Syncing backup to cloud storage..."
        
        local s3_path="s3://$S3_BUCKET/claude-flow/backups/$(basename "$backup_file")"
        local checksum_path="s3://$S3_BUCKET/claude-flow/backups/$(basename "${backup_file}.sha256")"
        
        # Upload backup file
        aws s3 cp "$backup_file" "$s3_path" --storage-class STANDARD_IA || error "Failed to upload backup to S3"
        
        # Upload checksum
        aws s3 cp "${backup_file}.sha256" "$checksum_path" || error "Failed to upload checksum to S3"
        
        log "Backup synced to cloud: $s3_path"
    elif [[ -n "$S3_BUCKET" ]]; then
        log "Warning: AWS CLI not available, skipping cloud sync"
    else
        log "No cloud storage configured, skipping sync"
    fi
}

# Cleanup old backups
cleanup_old_backups() {
    log "Cleaning up old backups..."
    
    # Clean up local backups older than retention period
    find "$BACKUP_DIR/daily" -name "*.tar.gz" -mtime +$RETENTION_DAYS -delete 2>/dev/null || true
    find "$BACKUP_DIR/daily" -name "*.sha256" -mtime +$RETENTION_DAYS -delete 2>/dev/null || true
    
    # Clean up cloud backups if S3 is configured
    if [[ -n "$S3_BUCKET" ]] && command -v aws >/dev/null 2>&1; then
        local cutoff_date=$(date -d "$RETENTION_DAYS days ago" +%Y%m%d)
        
        aws s3 ls "s3://$S3_BUCKET/claude-flow/backups/" | while read -r line; do
            local file_date=$(echo "$line" | awk '{print $1}' | tr -d '-')
            local file_name=$(echo "$line" | awk '{print $4}')
            
            if [[ "$file_date" < "$cutoff_date" ]]; then
                aws s3 rm "s3://$S3_BUCKET/claude-flow/backups/$file_name" || true
                log "Removed old cloud backup: $file_name"
            fi
        done
    fi
    
    log "Cleanup completed"
}

# Verify backup integrity
verify_backup() {
    local backup_file="$1"
    
    log "Verifying backup integrity..."
    
    # Verify checksum
    if sha256sum -c "${backup_file}.sha256" >/dev/null 2>&1; then
        log "Backup integrity verified: checksum valid"
    else
        error "Backup integrity check failed: invalid checksum"
    fi
    
    # Test archive can be opened
    if tar -tzf "$backup_file" >/dev/null 2>&1; then
        log "Backup archive integrity verified"
    else
        error "Backup archive is corrupted"
    fi
}

# Health check before backup
pre_backup_checks() {
    log "Performing pre-backup health checks..."
    
    # Check available disk space
    local available_space=$(df "$BACKUP_DIR" | awk 'NR==2 {print $4}')
    local required_space=1048576  # 1GB in KB
    
    if [[ $available_space -lt $required_space ]]; then
        error "Insufficient disk space for backup (available: ${available_space}KB, required: ${required_space}KB)"
    fi
    
    # Check if Claude Flow is running
    if pgrep -f "claude-flow" >/dev/null 2>&1; then
        log "Claude Flow is running - creating hot backup"
    else
        log "Claude Flow is not running - creating cold backup"
    fi
    
    # Check database lock status
    if [[ -f "$DATA_DIR/claude-flow.db" ]] && lsof "$DATA_DIR/claude-flow.db" >/dev/null 2>&1; then
        log "Database is in use - backup will use WAL mode"
    fi
    
    log "Pre-backup checks completed successfully"
}

# Send notification
send_notification() {
    local status="$1"
    local message="$2"
    local backup_file="${3:-}"
    
    local notification_data=$(cat <<EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "service": "claude-flow-backup",
    "status": "$status",
    "message": "$message",
    "backup_file": "$backup_file",
    "hostname": "$(hostname)",
    "environment": "production"
}
EOF
)
    
    # Log notification
    log "Notification: $status - $message"
    
    # Send to monitoring system if webhook is configured
    if [[ -n "${BACKUP_WEBHOOK_URL:-}" ]]; then
        curl -X POST "$BACKUP_WEBHOOK_URL" \
             -H "Content-Type: application/json" \
             -d "$notification_data" \
             --max-time 10 \
             --silent \
             --show-error || log "Warning: Failed to send webhook notification"
    fi
}

# Main backup function
main() {
    log "Starting Claude Flow backup process..."
    send_notification "started" "Backup process initiated"
    
    local start_time=$(date +%s)
    
    # Trap to ensure cleanup on exit
    trap 'rm -rf "$BACKUP_DIR/temp"/* 2>/dev/null || true' EXIT
    
    try {
        # Pre-backup checks
        pre_backup_checks
        
        # Setup directories
        setup_backup_dirs
        
        # Perform backups
        backup_database
        backup_memory_store
        backup_configuration
        backup_application_state
        backup_logs
        
        # Encrypt if configured
        encrypt_backups
        
        # Create final archive
        local backup_file=$(create_final_archive)
        
        # Verify backup
        verify_backup "$backup_file"
        
        # Sync to cloud
        sync_to_cloud "$backup_file"
        
        # Cleanup old backups
        cleanup_old_backups
        
        # Calculate duration
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        log "Backup process completed successfully in ${duration} seconds"
        send_notification "success" "Backup completed successfully" "$backup_file"
        
        # Weekly and monthly backup handling
        local day_of_week=$(date +%u)  # 1=Monday, 7=Sunday
        local day_of_month=$(date +%d)
        
        if [[ $day_of_week -eq 7 ]]; then  # Sunday
            cp "$backup_file" "$BACKUP_DIR/weekly/" || log "Warning: Failed to copy weekly backup"
            log "Weekly backup created"
        fi
        
        if [[ $day_of_month -eq 01 ]]; then  # First day of month
            cp "$backup_file" "$BACKUP_DIR/monthly/" || log "Warning: Failed to copy monthly backup"
            log "Monthly backup created"
        fi
        
    } catch {
        local error_msg="Backup process failed: $1"
        log "$error_msg"
        send_notification "failure" "$error_msg"
        exit 1
    }
}

# Error handling wrapper
try() {
    local cmd="$1"
    eval "$cmd"
}

catch() {
    local handler="$1"
    if [[ $? -ne 0 ]]; then
        eval "$handler"
    fi
}

# Script execution
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi