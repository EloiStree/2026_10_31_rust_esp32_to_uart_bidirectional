use embassy_time::Instant;

const QUEUE_CAPACITY: usize = 32;
const MAX_DELAY_FORMAT_MS: u64 = 30 * 60 * 1000;

#[derive(Clone, Copy)]
struct PendingValue {
    index: u32,
    value: u32,
    due_at_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueFull;

pub struct IndexValueDateDelayer {
    pending: [Option<PendingValue>; QUEUE_CAPACITY],
}

impl Default for IndexValueDateDelayer {
    fn default() -> Self {
        Self {
            pending: [None; QUEUE_CAPACITY],
        }
    }
}

impl IndexValueDateDelayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append_index_value_to_delay_of(
        &mut self,
        index: u32,
        value: u32,
        milliseconds_to_delay: u64,
    ) -> Result<(), QueueFull> {
        let now_ms = self.current_time_in_milliseconds_in_esp32();
        self.append_index_value_to_delay_of_with_time(index, value, now_ms, milliseconds_to_delay)
    }

    pub fn append_index_value_to_delay_of_with_time(
        &mut self,
        index: u32,
        value: u32,
        time_on_esp32: u64,
        milliseconds_to_delay: u64,
    ) -> Result<(), QueueFull> {
        let slot = self
            .pending
            .iter_mut()
            .find(|slot| slot.is_none())
            .ok_or(QueueFull)?;

        *slot = Some(PendingValue {
            index,
            value,
            due_at_ms: time_on_esp32.saturating_add(milliseconds_to_delay),
        });
        Ok(())
    }

    pub fn current_time_in_milliseconds_in_esp32(&self) -> u64 {
        Instant::now().as_millis()
    }

    pub fn take_due(&mut self, now_ms: u64) -> Option<(u32, u32)> {
        let mut next_slot = None;
        let mut earliest_due_ms = u64::MAX;

        for (slot_index, slot) in self.pending.iter().enumerate() {
            if let Some(item) = slot {
                if item.due_at_ms <= now_ms && item.due_at_ms < earliest_due_ms {
                    earliest_due_ms = item.due_at_ms;
                    next_slot = Some(slot_index);
                }
            }
        }

        let item = self.pending[next_slot?].take()?;
        Some((item.index, item.value))
    }

    pub fn does_16_bytes_ii_q_has_delay_format(&self, bytes: &[u8; 16]) -> bool {
        read_u64_le(&bytes[8..16]) < MAX_DELAY_FORMAT_MS
    }

    pub fn append_index_value_with_delay_from_16_bytes_format(
        &mut self,
        bytes: &[u8; 16],
        milliseconds_to_delay: u64,
    ) -> Result<(), QueueFull> {
        let index = read_u32_le(&bytes[0..4]);
        let value = read_u32_le(&bytes[4..8]);
        let encoded_delay_ms = read_u64_le(&bytes[8..16]);
        let delay_ms = encoded_delay_ms.saturating_add(milliseconds_to_delay);

        self.append_index_value_to_delay_of(index, value, delay_ms)
    }

    pub fn append_index_value_with_delay_from_array_dividable_by_16_bytes(
        &mut self,
        bytes_array: &[u8],
    ) -> Result<usize, QueueFull> {
        let records = bytes_array.chunks_exact(16);
        let records_to_queue = records
            .clone()
            .filter(|chunk| {
                let bytes: &[u8; 16] = (*chunk).try_into().expect("chunk has exactly 16 bytes");
                self.does_16_bytes_ii_q_has_delay_format(bytes)
            })
            .count();
        let free_slots = self.pending.iter().filter(|slot| slot.is_none()).count();

        if records_to_queue > free_slots {
            return Err(QueueFull);
        }

        let now_ms = self.current_time_in_milliseconds_in_esp32();
        let mut queued = 0;
        for chunk in bytes_array.chunks_exact(16) {
            let bytes: &[u8; 16] = chunk.try_into().expect("chunk has exactly 16 bytes");
            if self.does_16_bytes_ii_q_has_delay_format(bytes) {
                let index = read_u32_le(&bytes[0..4]);
                let value = read_u32_le(&bytes[4..8]);
                let delay_ms = read_u64_le(&bytes[8..16]);
                self.append_index_value_to_delay_of_with_time(index, value, now_ms, delay_ms)?;
                queued += 1;
            }
        }

        Ok(queued)
    }
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn read_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}
