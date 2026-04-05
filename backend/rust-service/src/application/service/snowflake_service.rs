use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::application::service::errors::SnowflakeServiceError;

const CUSTOM_EPOCH: u64 = 1767225600000;

const WORKER_ID_BITS: u64 = 10;
const KIND_BITS: u64 = 2;
const SEQUENCE_BITS: u64 = 10;

const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

const KIND_SHIFT: u64 = SEQUENCE_BITS;
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS + KIND_BITS;
const TIMESTAMP_SHIFT: u64 = SEQUENCE_BITS + KIND_BITS + WORKER_ID_BITS;

struct State {
    last_timestamp: u64,
    sequence: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SnowflakeKind {
    Api = 0,
    Image = 1,
}

pub struct SnowflakeGenerator {
    worker_id: u64,
    kind: SnowflakeKind,
    state: Mutex<State>,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u64, kind: SnowflakeKind) -> Result<Self, SnowflakeServiceError> {
        if worker_id > MAX_WORKER_ID {
            return Err(SnowflakeServiceError::InvalidWorkerId(worker_id, MAX_WORKER_ID));
        }

        Ok(Self {
            worker_id,
            kind,
            state: Mutex::new(State {
                last_timestamp: 0,
                sequence: 0,
            }),
        })
    }

    pub fn generate_id(&self) -> Result<i64, SnowflakeServiceError> {
        let mut state = self.state.lock().unwrap();
        let mut current_timestamp = current_time_millis()?;

        if current_timestamp < state.last_timestamp {
            return Err(SnowflakeServiceError::ClockMoveBackward);
        }

        if current_timestamp == state.last_timestamp {
            state.sequence = (state.sequence + 1) & MAX_SEQUENCE;

            if state.sequence == 0 {
                while current_timestamp <= state.last_timestamp {
                    current_timestamp = current_time_millis()?;
                }
                state.sequence = 0;
            }
        } else {
            state.sequence = 0;
        }

        state.last_timestamp = current_timestamp;

        let id = (current_timestamp << TIMESTAMP_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | ((self.kind as u64) << KIND_SHIFT)
            | state.sequence;

        Ok(id as i64)
    }
}

fn current_time_millis() -> Result<u64, SnowflakeServiceError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SnowflakeServiceError::ClockMoveBackward)?
        .as_millis() as u64;
    Ok(now - CUSTOM_EPOCH)
}