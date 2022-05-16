#[derive(Debug)]
pub struct Song {
    pub audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub extra_perams_size: u16
    //data somewhere
}