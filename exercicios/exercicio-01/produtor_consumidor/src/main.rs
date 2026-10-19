use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const BUFFER_CAPACITY: usize = 5;
const ITEMS_TO_PRODUCE: usize = 12;

/// Buffer circular limitado protegido por um Mutex.
struct BoundedBuffer {
    queue: VecDeque<u32>,
    capacity: usize,
}

impl BoundedBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.queue.len() == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Estado compartilhado entre produtor e consumidor.
struct SharedState {
    buffer: Mutex<BoundedBuffer>,
    not_full: Condvar,  // Produtor espera enquanto o buffer estiver cheio.
    not_empty: Condvar, // Consumidor espera enquanto o buffer estiver vazio.
}

impl SharedState {
    fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            buffer: Mutex::new(BoundedBuffer::new(capacity)),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        })
    }

    /// Insere um item; bloqueia se o buffer estiver cheio.
    fn produce(&self, item: u32) {
        let mut buffer = self.buffer.lock().expect("Mutex do buffer foi corrompido");

        while buffer.is_full() {
            buffer = self
                .not_full
                .wait(buffer)
                .expect("Falha ao aguardar espaço no buffer");
        }

        buffer.queue.push_back(item);

        println!("[Produtor] produziu {} | buffer: {:?}", item, buffer.queue);

        self.not_empty.notify_one();
    }

    /// Remove um item; bloqueia se o buffer estiver vazio.
    fn consume(&self) -> u32 {
        let mut buffer = self.buffer.lock().expect("Mutex do buffer foi corrompido");

        while buffer.is_empty() {
            buffer = self
                .not_empty
                .wait(buffer)
                .expect("Falha ao aguardar item no buffer");
        }

        let item = buffer
            .queue
            .pop_front()
            .expect("Buffer deveria conter um item");

        println!("[Consumidor] consumiu {} | buffer: {:?}", item, buffer.queue);

        self.not_full.notify_one();

        item
    }
}

fn producer(state: Arc<SharedState>) {
    for item in 1u32..=ITEMS_TO_PRODUCE as u32 {
        state.produce(item);
        thread::sleep(Duration::from_millis(150));
    }
}

fn consumer(state: Arc<SharedState>) {
    for _ in 0..ITEMS_TO_PRODUCE {
        let _item = state.consume();
        thread::sleep(Duration::from_millis(250));
    }
}

fn main() {
    let state = SharedState::new(BUFFER_CAPACITY);

    let producer_state = Arc::clone(&state);
    let consumer_state = Arc::clone(&state);

    let producer_handle = thread::spawn(move || producer(producer_state));
    let consumer_handle = thread::spawn(move || consumer(consumer_state));

    producer_handle.join().expect("A thread produtora falhou");
    consumer_handle.join().expect("A thread consumidora falhou");

    println!("Processamento concluído.");
    println!("SO 2026.2 - Karen Menezes Ribeiro")
}