pub enum AudioStatus {
    Silent,
    Speaking,
    DeleteMarker,
}

// Voice это слот для аудиобуффера, который будет обрабатываться и выводиться на динамики.
// Это не голос игрока с войсчата, это именно уже зараннее заготовленный аудиофайл или
// поток семплов.

pub struct Voice {
    // audio_id - это id пакета семплов или аудиофайла, который создается в любое время,
    // но удалиться только после окончания программы.
    audio_id: usize,

    // Чем ниже значение приоритета, тем приоритетнее голос для замены в случае переполнение.
    // Приоритет 10 предпочтительнее
    // Диапазон от 0 до 65535 включительно
    prioritet: u16,

    // Какой семпл мы считаем СЛЕДУЮЩИМ
    current: usize,

    can_loop: bool,
}

pub struct PlayerVoice {
    buffer: rtrb::Consumer<f32>,
    stream: u32, // TODO: add stream
    data: u32,   // TODO: add struct of sound spec. like position and rotation
}

/*
 * Ключевая разница между PlayerVoice и Voice, это с какими данными они работают.
 * Обычный voice это уже готовые семплы, полученные откуда угодно. Мы можем
 * сразу узнать семпл номер 3331 и обработать.
 * PlayerVoice в свою очередь, это потоквая передача семплов по сети. Мы не можем узнать
 * что следующее. Это добавляет работу для компенсации джиттера.
 */

pub enum AudioSource {
    Ready(Voice),
    RawStream(PlayerVoice),
}

pub struct VoiceSlot {
    pub source: Option<AudioSource>,
    pub priority: u16,
    pub volume: f32,
}
