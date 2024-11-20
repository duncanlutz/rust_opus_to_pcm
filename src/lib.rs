use audiopus::{coder::Decoder, Channels, SampleRate};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn decode_opus_packet(
  sample_rate: u32,
  channels: u32,
  opus_packet: Buffer,
) -> Result<Int16Array> {
  let channels = match channels {
    1 => Channels::Mono,
    2 => Channels::Stereo,
    _ => return Err(Error::from_reason("Unsupported number of channels")),
  };

  let sample_rate = match sample_rate {
    8000 => SampleRate::Hz8000,
    12000 => SampleRate::Hz12000,
    16000 => SampleRate::Hz16000,
    24000 => SampleRate::Hz24000,
    48000 => SampleRate::Hz48000,
    _ => return Err(Error::from_reason("Unsupported sample rate")),
  };

  let mut decoder =
    Decoder::new(sample_rate, channels).map_err(|e| Error::from_reason(e.to_string()))?;

  let channel_count = match channels {
    Channels::Mono => 1,
    Channels::Stereo => 2,
    _ => return Err(Error::from_reason("Unsupported number of channels")),
  };

  let mut pcm_buffer = vec![0i16; 5760 * channel_count];

  let packet: &[u8] = opus_packet.as_ref();

  let samples_decoded = decoder
    .decode(Some(packet), &mut pcm_buffer, false)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  pcm_buffer.truncate(samples_decoded * channel_count);
  Ok(Int16Array::from(pcm_buffer))
}
