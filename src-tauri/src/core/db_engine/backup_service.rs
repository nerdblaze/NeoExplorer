pub struct BackupService {
    backup_period: usize,
}

impl BackupService {
    pub fn new(periodicity: usize) -> Self {
        Self {
            backup_period: periodicity,
        }
    }
}