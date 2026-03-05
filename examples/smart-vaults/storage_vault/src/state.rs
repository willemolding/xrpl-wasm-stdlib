use xrpl_wasm_stdlib::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};

const MAX_RECORDS: usize = 10; // Maximum number of deposit records to store

pub struct StorageVaultState {
    deposit_records: [(AccountID, i64); MAX_RECORDS], // Simple fixed-size array to store deposit records
    record_count: usize,                              // Track the number of records stored
}

impl StorageVaultState {
    pub const SERIALIZED_SIZE: usize = 4 + MAX_RECORDS * (ACCOUNT_ID_SIZE + 4); // 4 bytes for record count + records

    pub fn from_bytes(data: &[u8]) -> Self {
        // Deserialize the byte array into the StorageVaultState struct
        // For simplicity, this example assumes a fixed format and does not handle errors robustly
        let mut state = StorageVaultState {
            deposit_records: [(AccountID([0; ACCOUNT_ID_SIZE]), 0); MAX_RECORDS],
            record_count: 0,
        };

        const RECORD_SIZE: usize = ACCOUNT_ID_SIZE + 4; // AccountID + u32 amount
        if data.len() >= 4 {
            state.record_count = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
            for i in 0..state.record_count.min(MAX_RECORDS) {
                let offset = 4 + i * RECORD_SIZE;
                if data.len() >= offset + RECORD_SIZE {
                    let mut id_bytes = [0u8; ACCOUNT_ID_SIZE];
                    id_bytes.copy_from_slice(&data[offset..offset + ACCOUNT_ID_SIZE]);
                    let account_id = AccountID(id_bytes);
                    let amount = u32::from_le_bytes(
                        data[offset + ACCOUNT_ID_SIZE..offset + RECORD_SIZE]
                            .try_into()
                            .unwrap(),
                    ) as i64;
                    state.deposit_records[i] = (account_id, amount);
                }
            }
        }

        state
    }

    pub fn to_bytes(&self, data: &mut [u8]) -> usize {
        const RECORD_SIZE: usize = ACCOUNT_ID_SIZE + 4;
        let count = self.record_count.min(MAX_RECORDS);
        data[0..4].copy_from_slice(&(count as u32).to_le_bytes());
        for i in 0..count {
            let offset = 4 + i * RECORD_SIZE;
            let (ref account_id, amount) = self.deposit_records[i];
            data[offset..offset + ACCOUNT_ID_SIZE].copy_from_slice(&account_id.0);
            data[offset + ACCOUNT_ID_SIZE..offset + RECORD_SIZE]
                .copy_from_slice(&(amount as u32).to_le_bytes());
        }
        4 + count * RECORD_SIZE
    }

    /// Returns true if a matching record was found and removed, false if no match found
    pub fn remove_record(&mut self, account_id: &AccountID, amount: i64) -> bool {
        for i in 0..self.record_count.min(MAX_RECORDS) {
            if self.deposit_records[i].0 == *account_id && self.deposit_records[i].1 == amount {
                // Swap with last record and shrink
                self.record_count -= 1;
                self.deposit_records[i] = self.deposit_records[self.record_count];
                self.deposit_records[self.record_count] = (AccountID([0; ACCOUNT_ID_SIZE]), 0);
                return true;
            }
        }
        false
    }

    pub fn append_record(&mut self, account_id: AccountID, amount: i64) -> bool {
        if self.record_count >= MAX_RECORDS {
            return false;
        }
        self.deposit_records[self.record_count] = (account_id, amount);
        self.record_count += 1;
        true
    }
}
