#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

// Struktur data note
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    id: u64,
    title: String,
    content: String,
    created_at: u64, // timestamp
}

// Storage key
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // Ambil semua notes
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Membuat note baru
    pub fn create_note(
        env: Env,
        title: String,
        content: String,
    ) -> String {

        // 1. Ambil notes lama
        let mut notes: Vec<Note> = env
            .storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Ambil timestamp ledger sekarang
        let timestamp = env.ledger().timestamp();

        // 3. Buat note baru
        let note = Note {
            id: env.prng().gen::<u64>(),
            title,
            content,
            created_at: timestamp,
        };

        // 4. Tambahkan ke vector
        notes.push_back(note);

        // 5. Simpan kembali ke storage
        env.storage().instance().set(&NOTE_DATA, &notes);

        String::from_str(&env, "Notes berhasil ditambahkan")
    }

    // Hapus note berdasarkan ID
    pub fn delete_note(env: Env, id: u64) -> String {

        // 1. Ambil data notes
        let mut notes: Vec<Note> = env
            .storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari note berdasarkan id
        for i in 0..notes.len() {

            if notes.get(i).unwrap().id == id {

                // hapus note
                notes.remove(i);

                // simpan ulang
                env.storage().instance().set(&NOTE_DATA, &notes);

                return String::from_str(
                    &env,
                    "Berhasil hapus notes"
                );
            }
        }

        String::from_str(&env, "Notes tidak ditemukan")
    }
}

mod test;